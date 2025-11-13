/// Mock data generator for dry-run testing
use crate::tools::models::ToolDefinition;
use crate::utils::errors::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// Mock data for a single tool invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockData {
    pub tool_name: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
    pub success: bool,
    pub duration_ms: u64,
}

/// Collection of mock data for an entire workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockDataSet {
    pub workflow_id: String,
    pub trigger_data: serde_json::Value,
    pub tool_mocks: HashMap<String, MockData>,
    pub approval_responses: HashMap<String, bool>,
}

/// Generator for creating realistic mock data
pub struct MockGenerator {
    tools: Vec<ToolDefinition>,
}

impl MockGenerator {
    /// Create a new mock generator
    pub fn new(tools: Vec<ToolDefinition>) -> Self {
        Self { tools }
    }

    /// Generate a complete mock dataset for a workflow
    pub fn generate_dataset(
        &self,
        workflow_id: String,
        trigger_type: &str,
        required_tools: &[String],
    ) -> Result<MockDataSet> {
        let trigger_data = self.generate_trigger_data(trigger_type);
        let mut tool_mocks = HashMap::new();

        for tool_name in required_tools {
            if let Some(tool) = self.tools.iter().find(|t| t.name == *tool_name) {
                let mock = self.generate_tool_mock(tool)?;
                tool_mocks.insert(tool_name.clone(), mock);
            }
        }

        Ok(MockDataSet {
            workflow_id,
            trigger_data,
            tool_mocks,
            approval_responses: HashMap::new(),
        })
    }

    /// Generate mock trigger data based on trigger type
    fn generate_trigger_data(&self, trigger_type: &str) -> serde_json::Value {
        match trigger_type {
            "api" => json!({
                "method": "POST",
                "path": "/api/webhook",
                "headers": {
                    "Content-Type": "application/json",
                    "X-Request-ID": "mock-req-12345"
                },
                "body": {
                    "message": "Test webhook payload",
                    "timestamp": "2024-01-15T10:30:00Z"
                }
            }),
            "slack" => json!({
                "type": "message",
                "channel": "#general",
                "user": "U12345678",
                "text": "Please book a flight to NYC next Monday",
                "ts": "1642248600.000000"
            }),
            "schedule" => json!({
                "scheduled_time": "2024-01-15T10:30:00Z",
                "cron_expression": "0 9 * * MON"
            }),
            "manual" => json!({
                "triggered_by": "user@example.com",
                "triggered_at": "2024-01-15T10:30:00Z"
            }),
            _ => json!({
                "type": trigger_type,
                "timestamp": "2024-01-15T10:30:00Z"
            }),
        }
    }

    /// Generate mock data for a specific tool
    fn generate_tool_mock(&self, tool: &ToolDefinition) -> Result<MockData> {
        let (input, output) = match tool.name.as_str() {
            "slack_send_message" => (
                json!({
                    "channel": "#general",
                    "text": "Your flight has been booked successfully!",
                    "blocks": []
                }),
                json!({
                    "ok": true,
                    "channel": "C12345678",
                    "ts": "1642248600.000000",
                    "message": {
                        "text": "Your flight has been booked successfully!"
                    }
                }),
            ),
            "run_claude_agent" => (
                json!({
                    "prompt": "Extract flight details from this message: Please book a flight to NYC",
                    "max_tokens": 1024,
                    "model": "claude-sonnet-4-5-20250929"
                }),
                json!({
                    "response": "User wants to book a flight to NYC (New York City)",
                    "tokens_used": 45,
                    "finish_reason": "end_turn"
                }),
            ),
            "run_openai_agent" => (
                json!({
                    "prompt": "Summarize this conversation",
                    "max_tokens": 1024,
                    "model": "gpt-4"
                }),
                json!({
                    "response": "The conversation discusses travel plans and booking requirements.",
                    "tokens_used": 38,
                    "finish_reason": "stop"
                }),
            ),
            "search_flights" => (
                json!({
                    "origin": "SFO",
                    "destination": "JFK",
                    "departure_date": "2024-01-22",
                    "passengers": 1
                }),
                json!({
                    "flights": [
                        {
                            "flight_number": "UA1234",
                            "airline": "United Airlines",
                            "departure_time": "2024-01-22T08:00:00Z",
                            "arrival_time": "2024-01-22T16:30:00Z",
                            "price": 450.00,
                            "currency": "USD"
                        },
                        {
                            "flight_number": "AA5678",
                            "airline": "American Airlines",
                            "departure_time": "2024-01-22T10:15:00Z",
                            "arrival_time": "2024-01-22T18:45:00Z",
                            "price": 425.00,
                            "currency": "USD"
                        }
                    ]
                }),
            ),
            "book_flight" => (
                json!({
                    "flight_number": "UA1234",
                    "passenger": {
                        "first_name": "John",
                        "last_name": "Doe",
                        "email": "john.doe@example.com"
                    },
                    "payment_method": "credit_card"
                }),
                json!({
                    "booking_reference": "ABC123",
                    "status": "confirmed",
                    "confirmation_email_sent": true,
                    "total_price": 450.00,
                    "currency": "USD"
                }),
            ),
            "http_request" => (
                json!({
                    "url": "https://api.example.com/data",
                    "method": "GET",
                    "headers": {
                        "Authorization": "Bearer token123"
                    }
                }),
                json!({
                    "status_code": 200,
                    "body": {
                        "data": "Mock response data",
                        "timestamp": "2024-01-15T10:30:00Z"
                    }
                }),
            ),
            _ => {
                // Generic mock for unknown tools
                let mut input_obj = serde_json::Map::new();
                for param in &tool.parameters {
                    input_obj.insert(param.name.clone(), json!(format!("mock_{}", param.name)));
                }
                (
                    serde_json::Value::Object(input_obj),
                    json!({
                        "success": true,
                        "result": "Mock result for unknown tool"
                    }),
                )
            }
        };

        Ok(MockData {
            tool_name: tool.name.clone(),
            input,
            output,
            success: true,
            duration_ms: self.estimate_mock_duration(&tool.name),
        })
    }

    /// Estimate realistic duration for mock execution
    fn estimate_mock_duration(&self, tool_name: &str) -> u64 {
        match tool_name {
            "slack_send_message" => 800,
            "run_claude_agent" => 2300,
            "run_openai_agent" => 1800,
            "search_flights" => 1500,
            "book_flight" => 2000,
            "http_request" => 500,
            _ => 1000,
        }
    }

    /// Add an approval response to a dataset
    pub fn add_approval_response(dataset: &mut MockDataSet, approval_id: String, approved: bool) {
        dataset.approval_responses.insert(approval_id, approved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::models::ToolParameter;

    fn create_test_tool(name: &str) -> ToolDefinition {
        ToolDefinition {
            name: name.to_string(),
            description: format!("Test tool {}", name),
            category: crate::tools::models::ToolCategory::Utility,
            parameters: vec![ToolParameter {
                name: "test_param".to_string(),
                param_type: crate::tools::models::ParameterType::String,
                description: "Test parameter".to_string(),
                required: true,
                default: None,
            }],
            credentials_required: vec![],
            python_activity_path: format!("activities.{}", name),
            estimated_duration_secs: 1,
            cost_per_call_usd: 0.0,
        }
    }

    #[test]
    fn test_generate_dataset() {
        let tools = vec![
            create_test_tool("slack_send_message"),
            create_test_tool("run_claude_agent"),
        ];
        let generator = MockGenerator::new(tools);

        let dataset = generator
            .generate_dataset(
                "test-workflow".to_string(),
                "api",
                &["slack_send_message".to_string()],
            )
            .unwrap();

        assert_eq!(dataset.workflow_id, "test-workflow");
        assert!(dataset.trigger_data.is_object());
        assert_eq!(dataset.tool_mocks.len(), 1);
        assert!(dataset.tool_mocks.contains_key("slack_send_message"));
    }

    #[test]
    fn test_generate_trigger_data_api() {
        let generator = MockGenerator::new(vec![]);
        let trigger = generator.generate_trigger_data("api");

        assert!(trigger["method"].is_string());
        assert_eq!(trigger["method"], "POST");
    }

    #[test]
    fn test_generate_trigger_data_slack() {
        let generator = MockGenerator::new(vec![]);
        let trigger = generator.generate_trigger_data("slack");

        assert!(trigger["type"].is_string());
        assert_eq!(trigger["type"], "message");
        assert!(trigger["channel"].is_string());
    }

    #[test]
    fn test_generate_tool_mock_slack() {
        let tool = create_test_tool("slack_send_message");
        let generator = MockGenerator::new(vec![]);
        let mock = generator.generate_tool_mock(&tool).unwrap();

        assert_eq!(mock.tool_name, "slack_send_message");
        assert!(mock.success);
        assert!(mock.duration_ms > 0);
        assert!(mock.input.is_object());
        assert!(mock.output.is_object());
    }

    #[test]
    fn test_add_approval_response() {
        let tools = vec![];
        let generator = MockGenerator::new(tools);
        let mut dataset = generator
            .generate_dataset("test-workflow".to_string(), "manual", &[])
            .unwrap();

        MockGenerator::add_approval_response(&mut dataset, "approval_1".to_string(), true);

        assert!(dataset
            .approval_responses
            .get("approval_1")
            .is_some_and(|&v| v));
    }

    #[test]
    fn test_estimate_mock_duration() {
        let generator = MockGenerator::new(vec![]);

        assert_eq!(generator.estimate_mock_duration("slack_send_message"), 800);
        assert_eq!(generator.estimate_mock_duration("run_claude_agent"), 2300);
        assert_eq!(generator.estimate_mock_duration("unknown_tool"), 1000);
    }
}
