use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Complete workflow specification (internal DSL)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSpec {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub trigger: TriggerSpec,
    pub steps: Vec<StepSpec>,
    pub credentials: HashMap<String, String>, // credential_key -> Secret ARN
    pub metadata: WorkflowMetadata,
}

/// Metadata about the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub task_queue: String,
    pub estimated_duration_secs: u32,
    pub estimated_cost_usd: f64,
}

/// Trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerSpec {
    pub trigger_type: String, // webhook, schedule, manual
    pub config: HashMap<String, serde_json::Value>,
}

/// Single step in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepSpec {
    pub id: String,
    pub step_type: StepTypeSpec,
    pub description: String,
    pub inputs: Vec<InputMapping>,
    pub outputs: Vec<OutputMapping>,
    pub timeout_seconds: u32,
    pub retry_policy: Option<RetryPolicy>,
}

/// Type-specific step configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StepTypeSpec {
    Agent { config: AgentConfig },
    Activity { config: ActivityConfig },
    Approval { config: ApprovalConfig },
    Conditional { config: ConditionalConfig },
}

/// AI agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub system_prompt: String,
    pub model: String,
    pub tools: Vec<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

/// Activity (tool) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityConfig {
    pub activity_name: String,
    pub python_path: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Approval configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalConfig {
    pub approver: String,
    pub reason: String,
    pub timeout_minutes: u32,
    pub notification_channel: String,
}

/// Conditional branching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalConfig {
    pub condition: String,
    pub then_step: String,
    pub else_step: Option<String>,
}

/// Input data mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMapping {
    pub name: String,
    pub source: DataSource,
    pub transform: Option<String>,
}

/// Output data mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMapping {
    pub name: String,
    pub path: String, // JSONPath to extract from result
}

/// Source of input data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataSource {
    TriggerData { path: String },
    PreviousStep { step_id: String, field: String },
    Constant { value: serde_json::Value },
    Environment { var_name: String },
}

/// Retry policy for steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub maximum_attempts: u32,
    pub initial_interval_seconds: u32,
    pub backoff_coefficient: f32,
    pub maximum_interval_seconds: u32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            maximum_attempts: 3,
            initial_interval_seconds: 1,
            backoff_coefficient: 2.0,
            maximum_interval_seconds: 60,
        }
    }
}

impl WorkflowSpec {
    /// Create a new workflow spec
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let id = Uuid::new_v4();
        let name_str = name.into();

        Self {
            id,
            name: name_str.clone(),
            description: description.into(),
            trigger: TriggerSpec {
                trigger_type: "manual".to_string(),
                config: HashMap::new(),
            },
            steps: Vec::new(),
            credentials: HashMap::new(),
            metadata: WorkflowMetadata {
                created_at: chrono::Utc::now(),
                task_queue: format!("workflow-{}", id),
                estimated_duration_secs: 0,
                estimated_cost_usd: 0.0,
            },
        }
    }

    /// Add a step to the workflow
    pub fn add_step(mut self, step: StepSpec) -> Self {
        self.steps.push(step);
        self
    }

    /// Set the trigger configuration
    pub fn with_trigger(mut self, trigger: TriggerSpec) -> Self {
        self.trigger = trigger;
        self
    }

    /// Add credentials
    pub fn with_credentials(mut self, credentials: HashMap<String, String>) -> Self {
        self.credentials = credentials;
        self
    }

    /// Update metadata
    pub fn with_metadata(mut self, duration: u32, cost: f64) -> Self {
        self.metadata.estimated_duration_secs = duration;
        self.metadata.estimated_cost_usd = cost;
        self
    }

    /// Get the Python class name for this workflow
    pub fn class_name(&self) -> String {
        to_pascal_case(&self.name)
    }

    /// Get the Python module name for this workflow
    pub fn module_name(&self) -> String {
        self.name.to_lowercase().replace('-', "_")
    }
}

/// Convert snake_case to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Convert to valid Python variable name
pub fn to_python_var(s: &str) -> String {
    s.to_lowercase()
        .replace(['-', ' '], "_")
        .replace("__", "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_spec_creation() {
        let spec = WorkflowSpec::new("test_workflow", "A test workflow");
        assert_eq!(spec.name, "test_workflow");
        assert_eq!(spec.description, "A test workflow");
        assert_eq!(spec.steps.len(), 0);
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("test_case"), "TestCase");
        assert_eq!(to_pascal_case("single"), "Single");
    }

    #[test]
    fn test_to_python_var() {
        assert_eq!(to_python_var("Hello World"), "hello_world");
        assert_eq!(to_python_var("test-case"), "test_case");
        assert_eq!(to_python_var("some__var"), "some_var");
    }

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.maximum_attempts, 3);
        assert_eq!(policy.initial_interval_seconds, 1);
        assert_eq!(policy.backoff_coefficient, 2.0);
    }

    #[test]
    fn test_workflow_class_name() {
        let spec = WorkflowSpec::new("slack_travel_booking", "Travel workflow");
        assert_eq!(spec.class_name(), "SlackTravelBooking");
    }

    #[test]
    fn test_workflow_module_name() {
        let spec = WorkflowSpec::new("Slack_Travel_Booking", "Travel workflow");
        assert_eq!(spec.module_name(), "slack_travel_booking");
    }
}
