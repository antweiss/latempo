use crate::analyzer::{RequiredStep, StepType as ReqStepType, TriggerType, WorkflowRequirements};
use crate::compiler::dsl::*;
use crate::tools::ToolRegistry;
use crate::utils::{Result, WorkflowError};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Compiles workflow requirements into executable specifications
pub struct WorkflowCompiler {
    tool_registry: Arc<ToolRegistry>,
}

impl WorkflowCompiler {
    /// Create a new workflow compiler
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    /// Compile requirements into a workflow spec
    pub fn compile(
        &self,
        requirements: WorkflowRequirements,
        credentials: HashMap<String, String>,
    ) -> Result<WorkflowSpec> {
        info!("Compiling workflow: {}", requirements.name);

        // Create base workflow spec
        let mut spec = WorkflowSpec::new(&requirements.name, &requirements.description);

        // Set trigger
        spec.trigger = self.compile_trigger(&requirements.trigger_type);

        // Compile steps
        for req_step in &requirements.steps {
            let step = self.compile_step(req_step, &requirements)?;
            spec.steps.push(step);
        }

        // Add credentials
        spec.credentials = credentials;

        // Calculate estimates
        let (duration, cost) = self.calculate_estimates(&spec);
        spec.metadata.estimated_duration_secs = duration;
        spec.metadata.estimated_cost_usd = cost;

        info!(
            "Compiled workflow with {} steps, estimated duration: {}s, cost: ${:.2}",
            spec.steps.len(),
            duration,
            cost
        );

        // Validate the compiled spec
        self.validate_spec(&spec)?;

        Ok(spec)
    }

    /// Compile trigger configuration
    fn compile_trigger(&self, trigger_type: &TriggerType) -> TriggerSpec {
        let trigger_str = match trigger_type {
            TriggerType::Webhook => "webhook",
            TriggerType::Schedule => "schedule",
            TriggerType::Manual => "manual",
        };

        TriggerSpec {
            trigger_type: trigger_str.to_string(),
            config: HashMap::new(),
        }
    }

    /// Compile a single step
    fn compile_step(
        &self,
        req_step: &RequiredStep,
        requirements: &WorkflowRequirements,
    ) -> Result<StepSpec> {
        let step_type_spec = match req_step.step_type {
            ReqStepType::Agent => self.compile_agent_step(req_step)?,
            ReqStepType::ApiCall => self.compile_activity_step(req_step)?,
            ReqStepType::Transform => self.compile_transform_step(req_step)?,
            ReqStepType::Approval => self.compile_approval_step(req_step, requirements)?,
            ReqStepType::Conditional => self.compile_conditional_step(req_step)?,
        };

        let timeout = self.calculate_step_timeout(&step_type_spec);
        let retry_policy = self.create_retry_policy(&step_type_spec);

        let step = StepSpec {
            id: req_step.step_id.clone(),
            step_type: step_type_spec,
            description: req_step.description.clone(),
            inputs: self.compile_inputs(req_step),
            outputs: self.compile_outputs(req_step),
            timeout_seconds: timeout,
            retry_policy: Some(retry_policy),
        };

        Ok(step)
    }

    /// Compile an AI agent step
    fn compile_agent_step(&self, req_step: &RequiredStep) -> Result<StepTypeSpec> {
        Ok(StepTypeSpec::Agent {
            config: AgentConfig {
                system_prompt: format!(
                    "You are helping with: {}. Provide a structured response.",
                    req_step.description
                ),
                model: "claude-sonnet-4.5-20250929".to_string(),
                tools: vec![],
                temperature: Some(1.0),
                max_tokens: Some(4096),
            },
        })
    }

    /// Compile an activity (tool) step
    fn compile_activity_step(&self, req_step: &RequiredStep) -> Result<StepTypeSpec> {
        let tool_name = req_step
            .tool_name
            .as_ref()
            .ok_or_else(|| WorkflowError::validation("Activity step must have tool_name"))?;

        let tool = self
            .tool_registry
            .get_tool(tool_name)
            .ok_or_else(|| WorkflowError::ToolNotFound(tool_name.clone()))?;

        Ok(StepTypeSpec::Activity {
            config: ActivityConfig {
                activity_name: tool.name.clone(),
                python_path: tool.python_activity_path.clone(),
                parameters: HashMap::new(), // Will be filled from inputs
            },
        })
    }

    /// Compile a transform step
    fn compile_transform_step(&self, req_step: &RequiredStep) -> Result<StepTypeSpec> {
        // Transform is essentially an activity with a transform function
        Ok(StepTypeSpec::Activity {
            config: ActivityConfig {
                activity_name: format!("transform_{}", req_step.step_id),
                python_path: "activities.utilities.transform".to_string(),
                parameters: HashMap::new(),
            },
        })
    }

    /// Compile an approval step
    fn compile_approval_step(
        &self,
        req_step: &RequiredStep,
        requirements: &WorkflowRequirements,
    ) -> Result<StepTypeSpec> {
        // Find the approval point for this step
        let approval_point = requirements
            .approval_points
            .iter()
            .find(|ap| ap.step_id == req_step.step_id)
            .ok_or_else(|| {
                WorkflowError::validation(format!(
                    "No approval point found for step {}",
                    req_step.step_id
                ))
            })?;

        Ok(StepTypeSpec::Approval {
            config: ApprovalConfig {
                approver: "human".to_string(),
                reason: approval_point.reason.clone(),
                timeout_minutes: approval_point.timeout_minutes,
                notification_channel: "email".to_string(), // TODO: Make configurable
            },
        })
    }

    /// Compile a conditional step
    fn compile_conditional_step(&self, req_step: &RequiredStep) -> Result<StepTypeSpec> {
        Ok(StepTypeSpec::Conditional {
            config: ConditionalConfig {
                condition: "result.success == true".to_string(),
                then_step: format!("{}_then", req_step.step_id),
                else_step: Some(format!("{}_else", req_step.step_id)),
            },
        })
    }

    /// Compile input mappings for a step
    fn compile_inputs(&self, req_step: &RequiredStep) -> Vec<InputMapping> {
        let mut inputs = Vec::new();

        // If this step depends on previous steps, create mappings
        for dep in &req_step.depends_on {
            inputs.push(InputMapping {
                name: format!("{}_result", dep),
                source: DataSource::PreviousStep {
                    step_id: dep.clone(),
                    field: "result".to_string(),
                },
                transform: None,
            });
        }

        // If no dependencies, use trigger data
        if inputs.is_empty() {
            inputs.push(InputMapping {
                name: "trigger_data".to_string(),
                source: DataSource::TriggerData {
                    path: "$".to_string(), // Root
                },
                transform: None,
            });
        }

        inputs
    }

    /// Compile output mappings for a step
    fn compile_outputs(&self, _req_step: &RequiredStep) -> Vec<OutputMapping> {
        vec![OutputMapping {
            name: "result".to_string(),
            path: "$".to_string(), // Full result
        }]
    }

    /// Calculate timeout for a step based on its type
    fn calculate_step_timeout(&self, step_type: &StepTypeSpec) -> u32 {
        match step_type {
            StepTypeSpec::Agent { .. } => 120,   // 2 minutes for AI
            StepTypeSpec::Activity { .. } => 60, // 1 minute for activities
            StepTypeSpec::Approval { config } => config.timeout_minutes * 60, // Convert to seconds
            StepTypeSpec::Conditional { .. } => 5, // 5 seconds for conditionals
        }
    }

    /// Create a retry policy for a step
    fn create_retry_policy(&self, step_type: &StepTypeSpec) -> RetryPolicy {
        match step_type {
            StepTypeSpec::Agent { .. } => RetryPolicy {
                maximum_attempts: 3,
                initial_interval_seconds: 2,
                backoff_coefficient: 2.0,
                maximum_interval_seconds: 60,
            },
            StepTypeSpec::Activity { .. } => RetryPolicy::default(),
            StepTypeSpec::Approval { .. } => RetryPolicy {
                maximum_attempts: 1, // No retries for approvals
                initial_interval_seconds: 0,
                backoff_coefficient: 1.0,
                maximum_interval_seconds: 0,
            },
            StepTypeSpec::Conditional { .. } => RetryPolicy {
                maximum_attempts: 1, // No retries for conditionals
                initial_interval_seconds: 0,
                backoff_coefficient: 1.0,
                maximum_interval_seconds: 0,
            },
        }
    }

    /// Calculate estimates for the workflow
    fn calculate_estimates(&self, spec: &WorkflowSpec) -> (u32, f64) {
        let mut total_duration = 0u32;
        let mut total_cost = 0.0f64;

        for step in &spec.steps {
            total_duration += step.timeout_seconds;

            // Estimate cost based on step type
            let step_cost = match &step.step_type {
                StepTypeSpec::Agent { .. } => 0.05, // ~$0.05 per AI call
                StepTypeSpec::Activity { config } => {
                    // Look up tool cost if available
                    if let Some(tool) = self.tool_registry.get_tool(&config.activity_name) {
                        tool.cost_per_call_usd
                    } else {
                        0.01
                    }
                }
                _ => 0.0,
            };

            total_cost += step_cost;
        }

        (total_duration, total_cost)
    }

    /// Validate the compiled spec
    fn validate_spec(&self, spec: &WorkflowSpec) -> Result<()> {
        // Check for steps
        if spec.steps.is_empty() {
            return Err(WorkflowError::validation(
                "Workflow must have at least one step",
            ));
        }

        // Validate step dependencies
        for step in &spec.steps {
            for input in &step.inputs {
                if let DataSource::PreviousStep { step_id, .. } = &input.source {
                    if !spec.steps.iter().any(|s| &s.id == step_id) {
                        return Err(WorkflowError::validation(format!(
                            "Step '{}' references non-existent step '{}'",
                            step.id, step_id
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{ApprovalPoint, TriggerType};

    fn create_test_compiler() -> WorkflowCompiler {
        let registry = Arc::new(ToolRegistry::new());
        WorkflowCompiler::new(registry)
    }

    #[test]
    fn test_compile_simple_workflow() {
        let compiler = create_test_compiler();

        let requirements = WorkflowRequirements {
            name: "test_workflow".to_string(),
            description: "A test workflow".to_string(),
            tools_needed: vec!["slack_send_message".to_string()],
            credentials_needed: vec!["slack/bot_token".to_string()],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![RequiredStep {
                step_id: "step1".to_string(),
                step_type: ReqStepType::ApiCall,
                description: "Send message".to_string(),
                tool_name: Some("slack_send_message".to_string()),
                depends_on: vec![],
            }],
        };

        let mut credentials = HashMap::new();
        credentials.insert(
            "slack/bot_token".to_string(),
            "arn:aws:secretsmanager:...".to_string(),
        );

        let spec = compiler.compile(requirements, credentials).unwrap();

        assert_eq!(spec.name, "test_workflow");
        assert_eq!(spec.steps.len(), 1);
        assert_eq!(spec.steps[0].id, "step1");
    }

    #[test]
    fn test_compile_with_approval() {
        let compiler = create_test_compiler();

        let requirements = WorkflowRequirements {
            name: "approval_workflow".to_string(),
            description: "Workflow with approval".to_string(),
            tools_needed: vec![],
            credentials_needed: vec![],
            approval_points: vec![ApprovalPoint {
                step_id: "approve_step".to_string(),
                reason: "Requires approval".to_string(),
                timeout_minutes: 30,
            }],
            trigger_type: TriggerType::Webhook,
            steps: vec![RequiredStep {
                step_id: "approve_step".to_string(),
                step_type: ReqStepType::Approval,
                description: "Approve action".to_string(),
                tool_name: None,
                depends_on: vec![],
            }],
        };

        let spec = compiler.compile(requirements, HashMap::new()).unwrap();

        assert_eq!(spec.steps.len(), 1);
        assert!(matches!(
            spec.steps[0].step_type,
            StepTypeSpec::Approval { .. }
        ));
    }
}
