use crate::utils::{Result, WorkflowError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY_MS: u64 = 1000;

/// Client for interacting with Anthropic's Claude API
#[derive(Clone)]
pub struct AnthropicClient {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    role: String,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    content_type: String,
    text: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    #[allow(dead_code)]
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

impl AnthropicClient {
    /// Create a new Anthropic client
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            api_key,
            model: model.unwrap_or_else(|| "claude-sonnet-4.5-20250929".to_string()),
        }
    }

    /// Analyze a prompt and return the response
    pub async fn analyze(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        self.analyze_with_retry(system_prompt, user_prompt, MAX_RETRIES)
            .await
    }

    /// Analyze with automatic retry on transient failures
    async fn analyze_with_retry(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        max_retries: u32,
    ) -> Result<String> {
        let mut last_error = None;
        let mut delay = INITIAL_RETRY_DELAY_MS;

        for attempt in 1..=max_retries {
            info!("Anthropic API attempt {}/{}", attempt, max_retries);

            match self.call_api(system_prompt, user_prompt).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);

                    if attempt < max_retries {
                        debug!("Retry attempt {} failed, waiting {}ms", attempt, delay);
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                        delay *= 2; // Exponential backoff
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| WorkflowError::analysis("Max retries exceeded")))
    }

    /// Make the actual API call
    async fn call_api(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![Message {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            }],
            system: Some(system_prompt.to_string()),
        };

        debug!("Calling Anthropic API with model: {}", self.model);

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;

            // Try to parse as structured error
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&error_text) {
                return Err(WorkflowError::analysis(format!(
                    "API error ({}): {}",
                    error_response.error.error_type, error_response.error.message
                )));
            }

            return Err(WorkflowError::analysis(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        if anthropic_response.content.is_empty() {
            return Err(WorkflowError::analysis("Empty response from API"));
        }

        let text = anthropic_response.content[0].text.clone();
        debug!("Received response from Anthropic API ({} chars)", text.len());

        Ok(text)
    }

    /// Validate that the API key is set
    pub fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() || self.api_key == "test-key-for-ci" {
            return Err(WorkflowError::config(
                "Valid Anthropic API key required. Set ANTHROPIC_API_KEY environment variable.",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AnthropicClient::new("test-key".to_string(), None);
        assert_eq!(client.model, "claude-sonnet-4.5-20250929");
    }

    #[test]
    fn test_client_with_custom_model() {
        let client = AnthropicClient::new("test-key".to_string(), Some("custom-model".to_string()));
        assert_eq!(client.model, "custom-model");
    }

    #[test]
    fn test_api_key_validation() {
        let client = AnthropicClient::new("".to_string(), None);
        assert!(client.validate_api_key().is_err());

        let client = AnthropicClient::new("test-key-for-ci".to_string(), None);
        assert!(client.validate_api_key().is_err());

        let client = AnthropicClient::new("sk-ant-valid-key".to_string(), None);
        assert!(client.validate_api_key().is_ok());
    }
}
