use serde::{Deserialize, Serialize};

/// Definition of a tool that can be used in workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub parameters: Vec<ToolParameter>,
    pub credentials_required: Vec<String>,
    pub python_activity_path: String,
    pub estimated_duration_secs: u32,
    pub cost_per_call_usd: f64,
}

/// Category of tool for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToolCategory {
    Messaging,
    Ai,
    Travel,
    Data,
    Utility,
}

/// Parameter definition for a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub description: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

/// Type of a parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Object,
    Array,
}

impl ToolDefinition {
    /// Create a new tool definition
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: ToolCategory,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            category,
            parameters: Vec::new(),
            credentials_required: Vec::new(),
            python_activity_path: String::new(),
            estimated_duration_secs: 60,
            cost_per_call_usd: 0.0,
        }
    }

    /// Add a parameter to this tool
    pub fn with_parameter(mut self, param: ToolParameter) -> Self {
        self.parameters.push(param);
        self
    }

    /// Add a required credential
    pub fn with_credential(mut self, cred: impl Into<String>) -> Self {
        self.credentials_required.push(cred.into());
        self
    }

    /// Set the Python activity path
    pub fn with_python_path(mut self, path: impl Into<String>) -> Self {
        self.python_activity_path = path.into();
        self
    }

    /// Set the estimated duration
    pub fn with_duration(mut self, seconds: u32) -> Self {
        self.estimated_duration_secs = seconds;
        self
    }

    /// Set the cost per call
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost_per_call_usd = cost;
        self
    }
}

impl ToolParameter {
    /// Create a required parameter
    pub fn required(
        name: impl Into<String>,
        param_type: ParameterType,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            param_type,
            description: description.into(),
            required: true,
            default: None,
        }
    }

    /// Create an optional parameter
    pub fn optional(
        name: impl Into<String>,
        param_type: ParameterType,
        description: impl Into<String>,
        default: Option<serde_json::Value>,
    ) -> Self {
        Self {
            name: name.into(),
            param_type,
            description: description.into(),
            required: false,
            default,
        }
    }
}
