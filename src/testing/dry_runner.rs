/// Dry-run orchestrator for testing workflows before deployment
use crate::compiler::code_generator::GeneratedCode;
use crate::compiler::dsl::models::WorkflowSpec;
use crate::testing::mock_generator::{MockDataSet, MockGenerator};
use crate::testing::python_executor::{ExecutionResult, PythonExecutor};
use crate::testing::trace_collector::{TraceCollector, TraceEvent, TraceEventType, TraceSummary};
use crate::tools::registry::ToolRegistry;
use crate::utils::errors::{Result, WorkflowError};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Configuration for dry-run execution
#[derive(Debug, Clone)]
pub struct DryRunConfig {
    pub workflow_id: String,
    pub mock_data: Option<MockDataSet>,
    pub timeout_secs: u64,
    pub save_artifacts: bool,
    pub artifacts_dir: Option<PathBuf>,
}

/// Result of dry-run execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunResult {
    pub success: bool,
    pub trace_summary: TraceSummary,
    pub execution_output: String,
    pub error_message: Option<String>,
    pub artifacts_path: Option<PathBuf>,
}

/// Orchestrator for dry-run workflow testing
pub struct DryRunner {
    python_executor: PythonExecutor,
    tool_registry: ToolRegistry,
}

impl DryRunner {
    /// Create a new dry-run orchestrator
    pub fn new(tool_registry: ToolRegistry) -> Self {
        Self {
            python_executor: PythonExecutor::new(),
            tool_registry,
        }
    }

    /// Execute a workflow in dry-run mode
    pub async fn run(
        &self,
        spec: &WorkflowSpec,
        generated_code: &GeneratedCode,
        config: DryRunConfig,
    ) -> Result<DryRunResult> {
        tracing::info!("Starting dry-run for workflow: {}", config.workflow_id);

        // Create temporary directory for execution
        let temp_dir = TempDir::new().map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to create temp directory: {}", e))
        })?;

        let temp_path = temp_dir.path();

        // Extract config fields before moving
        let save_artifacts = config.save_artifacts;
        let artifacts_dir = config.artifacts_dir.clone();
        let timeout_secs = config.timeout_secs;
        let mock_data_opt = config.mock_data;

        // Step 1: Set up execution environment
        self.setup_environment(temp_path, generated_code).await?;

        // Step 2: Generate or load mock data
        let mock_data = self.prepare_mock_data(spec, mock_data_opt)?;

        // Step 3: Inject mocks into the environment
        self.inject_mocks(temp_path, &mock_data).await?;

        // Step 4: Execute the workflow
        let execution_result = self.execute_workflow(temp_path, timeout_secs).await?;

        // Step 5: Collect and analyze trace
        let trace_collector = self.collect_trace(&execution_result)?;
        let mut trace_summary = trace_collector.summarize();

        // Step 6: Estimate costs
        trace_summary.estimated_cost = self.estimate_cost(spec, &trace_summary);

        // Step 7: Save artifacts if requested
        let artifacts_path = if save_artifacts {
            let artifacts_config = DryRunConfig {
                workflow_id: config.workflow_id.clone(),
                mock_data: None,
                timeout_secs,
                save_artifacts,
                artifacts_dir: artifacts_dir.clone(),
            };
            Some(self.save_artifacts(temp_path, &artifacts_config).await?)
        } else {
            None
        };

        // Determine overall success
        let success = execution_result.success && trace_summary.failed_steps == 0;

        let error_message = if !success {
            Some(execution_result.stderr.clone())
        } else {
            None
        };

        Ok(DryRunResult {
            success,
            trace_summary,
            execution_output: execution_result.stdout.clone(),
            error_message,
            artifacts_path,
        })
    }

    /// Set up the execution environment
    async fn setup_environment(
        &self,
        temp_path: &Path,
        generated_code: &GeneratedCode,
    ) -> Result<()> {
        tracing::info!("Setting up execution environment");

        // Create directory structure
        fs::create_dir_all(temp_path).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to create directory: {}", e))
        })?;

        // Write workflow code
        let workflow_path = temp_path.join("workflow.py");
        fs::write(&workflow_path, &generated_code.workflow_code).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to write workflow code: {}", e))
        })?;

        // Write worker code
        let worker_path = temp_path.join("worker.py");
        fs::write(&worker_path, &generated_code.worker_code).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to write worker code: {}", e))
        })?;

        // Write requirements.txt
        let requirements_path = temp_path.join("requirements.txt");
        fs::write(&requirements_path, &generated_code.requirements).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to write requirements: {}", e))
        })?;

        // Create virtual environment
        let venv_path = temp_path.join("venv");
        self.python_executor.create_venv(&venv_path).await?;

        // Install dependencies
        self.python_executor
            .install_dependencies(&venv_path, &requirements_path)
            .await?;

        Ok(())
    }

    /// Prepare mock data for the workflow
    fn prepare_mock_data(
        &self,
        spec: &WorkflowSpec,
        provided_mock: Option<MockDataSet>,
    ) -> Result<MockDataSet> {
        if let Some(mock_data) = provided_mock {
            Ok(mock_data)
        } else {
            // Generate mock data
            let tools: Vec<_> = self
                .tool_registry
                .list_tools()
                .into_iter()
                .cloned()
                .collect();
            let generator = MockGenerator::new(tools);

            let trigger_type = spec.trigger.trigger_type.as_str();

            // Collect required tools from workflow steps
            let required_tools: Vec<String> = spec
                .steps
                .iter()
                .filter_map(|step| {
                    if let crate::compiler::dsl::models::StepTypeSpec::Activity { config } =
                        &step.step_type
                    {
                        Some(config.activity_name.clone())
                    } else {
                        None
                    }
                })
                .collect();

            generator.generate_dataset(spec.id.to_string(), trigger_type, &required_tools)
        }
    }

    /// Inject mocks into the execution environment
    async fn inject_mocks(&self, temp_path: &Path, mock_data: &MockDataSet) -> Result<()> {
        tracing::info!("Injecting mock data");

        // Write mock data to a JSON file
        let mock_data_path = temp_path.join("mock_data.json");
        let mock_json = serde_json::to_string_pretty(mock_data).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to serialize mock data: {}", e))
        })?;

        fs::write(&mock_data_path, mock_json)
            .map_err(|e| WorkflowError::DryRunError(format!("Failed to write mock data: {}", e)))?;

        // Write mock injector Python script
        let mock_injector = self.create_mock_injector_script();
        let injector_path = temp_path.join("mock_injector.py");
        fs::write(&injector_path, mock_injector).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to write mock injector: {}", e))
        })?;

        Ok(())
    }

    /// Create Python mock injector script
    fn create_mock_injector_script(&self) -> String {
        r#"
import json
import sys
from unittest.mock import Mock, patch

# Load mock data
with open('mock_data.json', 'r') as f:
    mock_data = json.load(f)

def create_mock_function(tool_name):
    """Create a mock function for a tool"""
    def mock_fn(*args, **kwargs):
        mock = mock_data['tool_mocks'].get(tool_name)
        if mock:
            return mock['output']
        return {"error": f"No mock data for {tool_name}"}
    return mock_fn

# Monkey-patch common tool imports
# This would be expanded based on actual tool integrations
"#
        .to_string()
    }

    /// Execute the workflow
    async fn execute_workflow(
        &self,
        temp_path: &Path,
        timeout_secs: u64,
    ) -> Result<ExecutionResult> {
        tracing::info!("Executing workflow with timeout: {}s", timeout_secs);

        let venv_path = temp_path.join("venv");
        let workflow_path = temp_path.join("workflow.py");

        self.python_executor
            .execute_script(Some(&venv_path), &workflow_path, timeout_secs)
            .await
    }

    /// Collect execution trace
    fn collect_trace(&self, execution_result: &ExecutionResult) -> Result<TraceCollector> {
        let mut collector = TraceCollector::new();

        // Add workflow start event
        collector.add_event(TraceEvent {
            timestamp: Utc::now(),
            event_type: TraceEventType::WorkflowStart,
            step_id: "workflow".to_string(),
            message: "Dry-run execution started".to_string(),
            duration_ms: None,
            metadata: serde_json::json!({}),
        });

        // Parse logs from stdout/stderr
        collector.parse_logs(&execution_result.stdout);
        collector.parse_logs(&execution_result.stderr);

        // Add workflow end event
        collector.add_event(TraceEvent {
            timestamp: Utc::now(),
            event_type: if execution_result.success {
                TraceEventType::WorkflowEnd
            } else {
                TraceEventType::StepError
            },
            step_id: "workflow".to_string(),
            message: if execution_result.success {
                "Dry-run execution completed successfully".to_string()
            } else {
                format!(
                    "Dry-run execution failed: exit code {}",
                    execution_result.exit_code
                )
            },
            duration_ms: Some(execution_result.duration_ms),
            metadata: serde_json::json!({
                "exit_code": execution_result.exit_code
            }),
        });

        Ok(collector)
    }

    /// Estimate workflow execution cost
    fn estimate_cost(&self, spec: &WorkflowSpec, summary: &TraceSummary) -> f64 {
        let mut total_cost = 0.0;

        // Estimate cost per step based on tool usage
        for step in &spec.steps {
            if let crate::compiler::dsl::models::StepTypeSpec::Activity { config } = &step.step_type
            {
                if let Some(tool) = self.tool_registry.get_tool(&config.activity_name) {
                    total_cost += tool.cost_per_call_usd;
                }
            } else if let crate::compiler::dsl::models::StepTypeSpec::Agent { config } =
                &step.step_type
            {
                // Estimate AI agent costs (rough estimate)
                let token_estimate = config.max_tokens.unwrap_or(1024) as f64;
                let cost_per_1k_tokens = match config.model.as_str() {
                    "claude-sonnet-4-5-20250929" => 0.003, // $3 per 1M tokens
                    "gpt-4" => 0.03,                       // $30 per 1M tokens
                    _ => 0.001,
                };
                total_cost += (token_estimate / 1000.0) * cost_per_1k_tokens;
            }
        }

        // Multiply by number of successful executions in trace
        if summary.successful_steps > 0 {
            total_cost * (summary.successful_steps as f64 / spec.steps.len() as f64)
        } else {
            total_cost
        }
    }

    /// Save execution artifacts
    async fn save_artifacts(&self, temp_path: &Path, config: &DryRunConfig) -> Result<PathBuf> {
        let artifacts_dir = if let Some(dir) = &config.artifacts_dir {
            dir.clone()
        } else {
            PathBuf::from("./artifacts").join(&config.workflow_id)
        };

        fs::create_dir_all(&artifacts_dir).map_err(|e| {
            WorkflowError::DryRunError(format!("Failed to create artifacts directory: {}", e))
        })?;

        // Copy generated code
        let files = vec![
            "workflow.py",
            "worker.py",
            "requirements.txt",
            "mock_data.json",
        ];

        for file in files {
            let src = temp_path.join(file);
            if src.exists() {
                let dst = artifacts_dir.join(file);
                fs::copy(&src, &dst).map_err(|e| {
                    WorkflowError::DryRunError(format!("Failed to copy artifact {}: {}", file, e))
                })?;
            }
        }

        tracing::info!("Artifacts saved to: {:?}", artifacts_dir);

        Ok(artifacts_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_dry_run_config_creation() {
        let config = DryRunConfig {
            workflow_id: "test-123".to_string(),
            mock_data: None,
            timeout_secs: 60,
            save_artifacts: false,
            artifacts_dir: None,
        };

        assert_eq!(config.workflow_id, "test-123");
        assert_eq!(config.timeout_secs, 60);
        assert!(!config.save_artifacts);
    }

    #[test]
    fn test_dry_runner_creation() {
        let registry = ToolRegistry::new();
        let runner = DryRunner::new(registry);

        assert!(std::mem::size_of_val(&runner) > 0);
    }

    #[test]
    fn test_estimate_cost_no_steps() {
        let registry = ToolRegistry::new();
        let runner = DryRunner::new(registry);

        let spec = WorkflowSpec {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            description: "Test workflow".to_string(),
            trigger: crate::compiler::dsl::models::TriggerSpec {
                trigger_type: "manual".to_string(),
                config: std::collections::HashMap::new(),
            },
            steps: vec![],
            credentials: std::collections::HashMap::new(),
            metadata: crate::compiler::dsl::models::WorkflowMetadata {
                created_at: chrono::Utc::now(),
                task_queue: "test-queue".to_string(),
                estimated_duration_secs: 60,
                estimated_cost_usd: 0.0,
            },
        };

        let summary = TraceSummary {
            total_duration_ms: 0,
            total_steps: 0,
            successful_steps: 0,
            failed_steps: 0,
            total_tool_invocations: 0,
            total_approvals: 0,
            step_durations: vec![],
            estimated_cost: 0.0,
        };

        let cost = runner.estimate_cost(&spec, &summary);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_create_mock_injector_script() {
        let registry = ToolRegistry::new();
        let runner = DryRunner::new(registry);

        let script = runner.create_mock_injector_script();
        assert!(script.contains("import json"));
        assert!(script.contains("mock_data"));
    }
}
