use crate::utils::{Result, WorkflowError};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub aws: AwsConfig,
    pub temporal: TemporalConfig,
    pub ai: AiConfig,
    pub storage: StorageConfig,
    pub app: AppConfig,
}

/// AWS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

/// Temporal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConfig {
    pub host: String,
    pub namespace: String,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
}

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub anthropic_api_key: String,
    pub openai_api_key: Option<String>,
    pub default_model: String,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub workflows_table: String,
    pub executions_table: String,
    pub code_bucket: String,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log_level: String,
    pub environment: String,
}

impl Settings {
    /// Load settings from environment variables
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        Ok(Settings {
            aws: AwsConfig {
                region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                access_key_id: env::var("AWS_ACCESS_KEY_ID").ok(),
                secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").ok(),
            },
            temporal: TemporalConfig {
                host: env::var("TEMPORAL_HOST")
                    .unwrap_or_else(|_| "localhost:7233".to_string()),
                namespace: env::var("TEMPORAL_NAMESPACE")
                    .unwrap_or_else(|_| "default".to_string()),
                cert_path: env::var("TEMPORAL_CERT_PATH")
                    .ok()
                    .map(PathBuf::from),
                key_path: env::var("TEMPORAL_KEY_PATH")
                    .ok()
                    .map(PathBuf::from),
            },
            ai: AiConfig {
                anthropic_api_key: env::var("ANTHROPIC_API_KEY")
                    .map_err(|_| {
                        WorkflowError::config("ANTHROPIC_API_KEY environment variable is required")
                    })?,
                openai_api_key: env::var("OPENAI_API_KEY").ok(),
                default_model: env::var("DEFAULT_AI_MODEL")
                    .unwrap_or_else(|_| "claude-sonnet-4.5-20250929".to_string()),
            },
            storage: StorageConfig {
                workflows_table: env::var("WORKFLOWS_TABLE")
                    .unwrap_or_else(|_| "ai-workflows".to_string()),
                executions_table: env::var("EXECUTIONS_TABLE")
                    .unwrap_or_else(|_| "ai-workflow-executions".to_string()),
                code_bucket: env::var("CODE_BUCKET")
                    .unwrap_or_else(|_| "ai-workflow-code".to_string()),
            },
            app: AppConfig {
                log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
                environment: env::var("ENVIRONMENT")
                    .unwrap_or_else(|_| "development".to_string()),
            },
        })
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.ai.anthropic_api_key.is_empty() {
            return Err(WorkflowError::config("Anthropic API key cannot be empty"));
        }

        if self.temporal.host.is_empty() {
            return Err(WorkflowError::config("Temporal host cannot be empty"));
        }

        if self.temporal.namespace.is_empty() {
            return Err(WorkflowError::config("Temporal namespace cannot be empty"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        env::set_var("ANTHROPIC_API_KEY", "test-key");
        let settings = Settings::from_env().unwrap();

        assert_eq!(settings.aws.region, "us-east-1");
        assert_eq!(settings.temporal.namespace, "default");
        assert_eq!(settings.app.log_level, "info");

        env::remove_var("ANTHROPIC_API_KEY");
    }
}
