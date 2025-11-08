use crate::tools::models::{ParameterType, ToolCategory, ToolDefinition, ToolParameter};

/// Generic HTTP request tool
pub fn http_request() -> ToolDefinition {
    ToolDefinition::new(
        "http_request",
        "Make a generic HTTP request to any API",
        ToolCategory::Utility,
    )
    .with_parameter(ToolParameter::required(
        "url",
        ParameterType::String,
        "URL to request",
    ))
    .with_parameter(ToolParameter::optional(
        "method",
        ParameterType::String,
        "HTTP method (GET, POST, PUT, DELETE, etc.)",
        Some(serde_json::json!("GET")),
    ))
    .with_parameter(ToolParameter::optional(
        "headers",
        ParameterType::Object,
        "HTTP headers as key-value pairs",
        Some(serde_json::json!({})),
    ))
    .with_parameter(ToolParameter::optional(
        "body",
        ParameterType::String,
        "Request body (for POST/PUT)",
        None,
    ))
    .with_parameter(ToolParameter::optional(
        "timeout",
        ParameterType::Integer,
        "Request timeout in seconds",
        Some(serde_json::json!(30)),
    ))
    .with_python_path("activities.utilities.http.request")
    .with_duration(30)
    .with_cost(0.0)
}
