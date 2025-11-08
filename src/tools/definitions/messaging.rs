use crate::tools::models::{ParameterType, ToolCategory, ToolDefinition, ToolParameter};

/// Slack send message tool
pub fn slack_send_message() -> ToolDefinition {
    ToolDefinition::new(
        "slack_send_message",
        "Send a message to a Slack channel",
        ToolCategory::Messaging,
    )
    .with_parameter(ToolParameter::required(
        "channel",
        ParameterType::String,
        "Slack channel ID or name (e.g., #general or C1234567890)",
    ))
    .with_parameter(ToolParameter::required(
        "text",
        ParameterType::String,
        "Message text to send",
    ))
    .with_parameter(ToolParameter::optional(
        "thread_ts",
        ParameterType::String,
        "Thread timestamp to reply to (optional)",
        None,
    ))
    .with_credential("slack/bot_token")
    .with_python_path("activities.messaging.slack.send_message")
    .with_duration(10)
    .with_cost(0.0)
}
