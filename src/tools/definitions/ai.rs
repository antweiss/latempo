use crate::tools::models::{ParameterType, ToolCategory, ToolDefinition, ToolParameter};

/// Claude AI agent tool
pub fn run_claude_agent() -> ToolDefinition {
    ToolDefinition::new(
        "run_claude_agent",
        "Execute a Claude AI agent with optional tools",
        ToolCategory::Ai,
    )
    .with_parameter(ToolParameter::required(
        "system_prompt",
        ParameterType::String,
        "System prompt to guide the AI's behavior",
    ))
    .with_parameter(ToolParameter::required(
        "user_message",
        ParameterType::String,
        "User message or question",
    ))
    .with_parameter(ToolParameter::optional(
        "tools",
        ParameterType::Array,
        "Optional tools the agent can use",
        Some(serde_json::json!([])),
    ))
    .with_parameter(ToolParameter::optional(
        "model",
        ParameterType::String,
        "Model to use (default: claude-sonnet-4.5-20250929)",
        Some(serde_json::json!("claude-sonnet-4.5-20250929")),
    ))
    .with_parameter(ToolParameter::optional(
        "temperature",
        ParameterType::Float,
        "Temperature for response randomness (0.0-1.0)",
        Some(serde_json::json!(1.0)),
    ))
    .with_parameter(ToolParameter::optional(
        "max_tokens",
        ParameterType::Integer,
        "Maximum tokens in response",
        Some(serde_json::json!(4096)),
    ))
    .with_credential("anthropic/api_key")
    .with_python_path("activities.ai.claude.run_agent")
    .with_duration(120)
    .with_cost(0.05)
}

/// OpenAI agent tool
pub fn run_openai_agent() -> ToolDefinition {
    ToolDefinition::new(
        "run_openai_agent",
        "Execute an OpenAI agent (ChatGPT)",
        ToolCategory::Ai,
    )
    .with_parameter(ToolParameter::required(
        "system_prompt",
        ParameterType::String,
        "System prompt to guide the AI's behavior",
    ))
    .with_parameter(ToolParameter::required(
        "user_message",
        ParameterType::String,
        "User message or question",
    ))
    .with_parameter(ToolParameter::optional(
        "model",
        ParameterType::String,
        "Model to use (default: gpt-4-turbo)",
        Some(serde_json::json!("gpt-4-turbo")),
    ))
    .with_parameter(ToolParameter::optional(
        "temperature",
        ParameterType::Float,
        "Temperature for response randomness (0.0-2.0)",
        Some(serde_json::json!(1.0)),
    ))
    .with_credential("openai/api_key")
    .with_python_path("activities.ai.openai.run_agent")
    .with_duration(120)
    .with_cost(0.03)
}
