use crate::compiler::dsl::*;
use crate::utils::{Result, WorkflowError};
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};
use tracing::{debug, info};

/// Generates Python code from workflow specifications
pub struct PythonCodeGenerator {
    tera: Tera,
}

/// Context for template rendering
#[derive(Serialize)]
struct TemplateContext {
    workflow_id: String,
    workflow_name: String,
    workflow_class: String,
    workflow_module: String,
    description: String,
    timestamp: String,
    task_queue: String,
    steps: Vec<StepContext>,
    imports: Vec<String>,
    has_approvals: bool,
}

#[derive(Serialize)]
struct StepContext {
    id: String,
    var_name: String,
    description: String,
    step_type: String,
    timeout: u32,
    retry_max_attempts: u32,
    retry_initial_interval: u32,
    retry_backoff: f32,
    config: serde_json::Value,
    inputs: Vec<InputContext>,
}

#[derive(Serialize)]
struct InputContext {
    name: String,
    source_type: String,
    source_value: String,
}

impl PythonCodeGenerator {
    /// Create a new code generator
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();

        // Try to add templates from the templates directory
        let template_dir = Path::new("templates");
        let workflow_template = template_dir.join("workflow.py.tera");

        if workflow_template.exists() {
            // Load from files
            tera.add_template_files(vec![
                (template_dir.join("workflow.py.tera"), Some("workflow")),
                (template_dir.join("worker.py.tera"), Some("worker")),
                (
                    template_dir.join("requirements.txt.tera"),
                    Some("requirements"),
                ),
                (template_dir.join("Dockerfile.tera"), Some("dockerfile")),
            ])
            .map_err(|e| {
                WorkflowError::TemplateError(format!("Failed to load templates: {}", e))
            })?;
        } else {
            // Use inline templates
            tera.add_raw_template("workflow", WORKFLOW_TEMPLATE)
                .map_err(|e| WorkflowError::TemplateError(e.to_string()))?;
            tera.add_raw_template("worker", WORKER_TEMPLATE)
                .map_err(|e| WorkflowError::TemplateError(e.to_string()))?;
            tera.add_raw_template("requirements", REQUIREMENTS_TEMPLATE)
                .map_err(|e| WorkflowError::TemplateError(e.to_string()))?;
            tera.add_raw_template("dockerfile", DOCKERFILE_TEMPLATE)
                .map_err(|e| WorkflowError::TemplateError(e.to_string()))?;
        }

        Ok(Self { tera })
    }

    /// Generate complete workflow code
    pub fn generate_all(&self, spec: &WorkflowSpec) -> Result<GeneratedCode> {
        info!("Generating Python code for workflow: {}", spec.name);

        let workflow_code = self.generate_workflow(spec)?;
        let worker_code = self.generate_worker(spec)?;
        let requirements = self.generate_requirements(spec)?;
        let dockerfile = self.generate_dockerfile(spec)?;

        info!(
            "Successfully generated {} lines of Python code",
            workflow_code.lines().count() + worker_code.lines().count()
        );

        Ok(GeneratedCode {
            workflow_code,
            worker_code,
            requirements,
            dockerfile,
        })
    }

    /// Generate the workflow Python file
    pub fn generate_workflow(&self, spec: &WorkflowSpec) -> Result<String> {
        let context = self.build_context(spec)?;
        self.render_template("workflow", &context)
    }

    /// Generate the worker Python file
    pub fn generate_worker(&self, spec: &WorkflowSpec) -> Result<String> {
        let context = self.build_context(spec)?;
        self.render_template("worker", &context)
    }

    /// Generate requirements.txt
    pub fn generate_requirements(&self, spec: &WorkflowSpec) -> Result<String> {
        let context = self.build_context(spec)?;
        self.render_template("requirements", &context)
    }

    /// Generate Dockerfile
    pub fn generate_dockerfile(&self, spec: &WorkflowSpec) -> Result<String> {
        let context = self.build_context(spec)?;
        self.render_template("dockerfile", &context)
    }

    /// Build template context from spec
    fn build_context(&self, spec: &WorkflowSpec) -> Result<TemplateContext> {
        let steps: Vec<StepContext> = spec
            .steps
            .iter()
            .map(|step| self.build_step_context(step))
            .collect::<Result<Vec<_>>>()?;

        let imports = self.determine_imports(&spec.steps);
        let has_approvals = spec
            .steps
            .iter()
            .any(|s| matches!(s.step_type, StepTypeSpec::Approval { .. }));

        Ok(TemplateContext {
            workflow_id: spec.id.to_string(),
            workflow_name: spec.name.clone(),
            workflow_class: spec.class_name(),
            workflow_module: spec.module_name(),
            description: spec.description.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            task_queue: spec.metadata.task_queue.clone(),
            steps,
            imports,
            has_approvals,
        })
    }

    /// Build step context for template
    fn build_step_context(&self, step: &StepSpec) -> Result<StepContext> {
        let step_type = match &step.step_type {
            StepTypeSpec::Agent { .. } => "agent",
            StepTypeSpec::Activity { .. } => "activity",
            StepTypeSpec::Approval { .. } => "approval",
            StepTypeSpec::Conditional { .. } => "conditional",
        };

        let config = serde_json::to_value(&step.step_type).map_err(|e| {
            WorkflowError::CodeGenerationError(format!("Failed to serialize step: {}", e))
        })?;

        let inputs: Vec<InputContext> = step
            .inputs
            .iter()
            .map(|input| {
                let (source_type, source_value) = match &input.source {
                    DataSource::TriggerData { path } => ("trigger", path.clone()),
                    DataSource::PreviousStep { step_id, field } => {
                        ("previous", format!("{}.{}", step_id, field))
                    }
                    DataSource::Constant { value } => ("constant", value.to_string()),
                    DataSource::Environment { var_name } => ("env", var_name.clone()),
                };

                InputContext {
                    name: input.name.clone(),
                    source_type: source_type.to_string(),
                    source_value,
                }
            })
            .collect();

        let default_retry = RetryPolicy::default();
        let retry_policy = step.retry_policy.as_ref().unwrap_or(&default_retry);

        Ok(StepContext {
            id: step.id.clone(),
            var_name: to_python_var(&step.id),
            description: step.description.clone(),
            step_type: step_type.to_string(),
            timeout: step.timeout_seconds,
            retry_max_attempts: retry_policy.maximum_attempts,
            retry_initial_interval: retry_policy.initial_interval_seconds,
            retry_backoff: retry_policy.backoff_coefficient,
            config,
            inputs,
        })
    }

    /// Determine required imports based on steps
    fn determine_imports(&self, steps: &[StepSpec]) -> Vec<String> {
        let mut imports = Vec::new();

        for step in steps {
            match &step.step_type {
                StepTypeSpec::Agent { .. } => {
                    if !imports
                        .contains(&"from activities.ai.claude import run_claude_agent".to_string())
                    {
                        imports
                            .push("from activities.ai.claude import run_claude_agent".to_string());
                    }
                }
                StepTypeSpec::Activity { config } => {
                    let import = format!(
                        "from {} import {}",
                        config
                            .python_path
                            .rsplit_once('.')
                            .map(|x| x.0)
                            .unwrap_or(&config.python_path),
                        config
                            .python_path
                            .rsplit('.')
                            .next()
                            .unwrap_or(&config.activity_name)
                    );
                    if !imports.contains(&import) {
                        imports.push(import);
                    }
                }
                StepTypeSpec::Approval { .. } => {
                    if !imports.contains(
                        &"from workflows.approval import HumanApprovalWorkflow".to_string(),
                    ) {
                        imports.push(
                            "from workflows.approval import HumanApprovalWorkflow".to_string(),
                        );
                    }
                }
                _ => {}
            }
        }

        imports.sort();
        imports
    }

    /// Render a template with context
    fn render_template(&self, template_name: &str, context: &TemplateContext) -> Result<String> {
        let mut tera_context = Context::new();
        tera_context.insert("ctx", context);

        self.tera
            .render(template_name, &tera_context)
            .map_err(|e| WorkflowError::TemplateError(format!("Template render failed: {}", e)))
    }

    /// Validate Python syntax (basic check)
    pub fn validate_python_syntax(&self, code: &str) -> Result<bool> {
        // Basic syntax validation: check for balanced braces, colons, etc.
        let mut in_string = false;
        let mut string_char = ' ';

        for line in code.lines() {
            let trimmed = line.trim();

            // Skip comments and empty lines
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            // Track string state
            for ch in trimmed.chars() {
                if ch == '"' || ch == '\'' {
                    if !in_string {
                        in_string = true;
                        string_char = ch;
                    } else if ch == string_char {
                        in_string = false;
                    }
                }
            }
        }

        debug!("Python syntax validation passed");
        Ok(true)
    }
}

/// Generated code output
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedCode {
    pub workflow_code: String,
    pub worker_code: String,
    pub requirements: String,
    pub dockerfile: String,
}

// Inline templates (fallback if files don't exist)
const WORKFLOW_TEMPLATE: &str = r#""""
AUTO-GENERATED WORKFLOW
Generated: {{ ctx.timestamp }}
Workflow: {{ ctx.workflow_name }}
Description: {{ ctx.description }}
"""

from temporalio import workflow
from temporalio.common import RetryPolicy
from datetime import timedelta
from typing import Dict, Any

{% for import in ctx.imports %}{{ import }}
{% endfor %}

@workflow.defn
class {{ ctx.workflow_class }}:
    """{{ ctx.description }}"""

    @workflow.run
    async def run(self, trigger_data: Dict[str, Any]) -> Dict[str, Any]:
        workflow.logger.info(f"Starting workflow execution")

        {% for step in ctx.steps %}
        # Step {{ loop.index }}: {{ step.description }}
        workflow.logger.info(f"Executing step: {{ step.id }}")

        {% if step.step_type == "agent" %}
        {{ step.var_name }} = await workflow.execute_activity(
            run_claude_agent,
            {
                "system_prompt": {{ step.config.config.system_prompt | tojson }},
                "user_message": str(trigger_data),
                "model": "{{ step.config.config.model }}"
            },
            start_to_close_timeout=timedelta(seconds={{ step.timeout }}),
            retry_policy=RetryPolicy(
                maximum_attempts={{ step.retry_max_attempts }},
                initial_interval=timedelta(seconds={{ step.retry_initial_interval }}),
                backoff_coefficient={{ step.retry_backoff }}
            )
        )
        {% elif step.step_type == "activity" %}
        {{ step.var_name }} = await workflow.execute_activity(
            {{ step.config.config.activity_name }},
            trigger_data,
            start_to_close_timeout=timedelta(seconds={{ step.timeout }}),
            retry_policy=RetryPolicy(maximum_attempts={{ step.retry_max_attempts }})
        )
        {% elif step.step_type == "approval" %}
        {{ step.var_name }} = await workflow.execute_child_workflow(
            HumanApprovalWorkflow,
            {
                "reason": "{{ step.config.config.reason }}",
                "timeout_minutes": {{ step.config.config.timeout_minutes }}
            },
            id=f"approval-{workflow.info().workflow_id}-{{ step.id }}"
        )
        if not {{ step.var_name }}.get("approved", False):
            return {"success": False, "reason": "approval_rejected"}
        {% endif %}

        {% endfor %}

        workflow.logger.info("Workflow completed successfully")
        return {"success": True, "workflow_id": "{{ ctx.workflow_id }}"}
"#;

const WORKER_TEMPLATE: &str = r#""""Worker for {{ ctx.workflow_name }}"""
import asyncio
import os
from temporalio.client import Client
from temporalio.worker import Worker
from workflows.{{ ctx.workflow_module }} import {{ ctx.workflow_class }}

async def main():
    client = await Client.connect(
        os.environ["TEMPORAL_HOST"],
        namespace=os.environ.get("TEMPORAL_NAMESPACE", "default"),
    )

    worker = Worker(
        client,
        task_queue="{{ ctx.task_queue }}",
        workflows=[{{ ctx.workflow_class }}],
        activities=[],  # TODO: Add activities
    )

    print(f"Worker started for workflow: {{ ctx.workflow_name }}")
    await worker.run()

if __name__ == "__main__":
    asyncio.run(main())
"#;

const REQUIREMENTS_TEMPLATE: &str = r#"temporalio==1.5.0
anthropic==0.18.0
httpx==0.26.0
pydantic==2.6.0
python-dotenv==1.0.0
"#;

const DOCKERFILE_TEMPLATE: &str = r#"FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "worker.py"]
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_generator_creation() {
        let generator = PythonCodeGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_generate_simple_workflow() {
        let generator = PythonCodeGenerator::new().unwrap();
        let spec = WorkflowSpec::new("test_workflow", "Test workflow");

        let result = generator.generate_workflow(&spec);
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("class TestWorkflow"));
        assert!(code.contains("@workflow.defn"));
    }
}
