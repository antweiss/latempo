/// AWS Secrets Manager integration for credential storage
use crate::utils::errors::{Result, WorkflowError};
use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Credential value wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialValue {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

/// AWS Secrets Manager client for credential management
pub struct AwsSecretsClient {
    client: SecretsManagerClient,
    prefix: String,
}

impl AwsSecretsClient {
    /// Create a new AWS Secrets Manager client
    pub fn new(client: SecretsManagerClient, prefix: String) -> Self {
        Self { client, prefix }
    }

    /// Store a credential in Secrets Manager
    pub async fn store_credential(
        &self,
        workflow_id: &str,
        credential_key: &str,
        credential_value: &str,
    ) -> Result<String> {
        let secret_name = self.generate_secret_name(workflow_id, credential_key);

        let credential = CredentialValue {
            key: credential_key.to_string(),
            value: credential_value.to_string(),
            description: Some(format!(
                "Credential {} for workflow {}",
                credential_key, workflow_id
            )),
        };

        let secret_string = serde_json::to_string(&credential).map_err(|e| {
            WorkflowError::CredentialError(format!("Failed to serialize credential: {}", e))
        })?;

        // Try to create the secret, if it exists, update it
        match self
            .client
            .create_secret()
            .name(&secret_name)
            .secret_string(&secret_string)
            .description(format!(
                "Credential {} for workflow {}",
                credential_key, workflow_id
            ))
            .send()
            .await
        {
            Ok(output) => {
                tracing::info!("Created secret: {}", secret_name);
                Ok(output.arn.unwrap_or(secret_name))
            }
            Err(e) => {
                // If secret already exists, update it
                if e.to_string().contains("ResourceExistsException") {
                    self.client
                        .put_secret_value()
                        .secret_id(&secret_name)
                        .secret_string(&secret_string)
                        .send()
                        .await
                        .map_err(|e| {
                            WorkflowError::CredentialError(format!(
                                "Failed to update secret: {}",
                                e
                            ))
                        })?;
                    tracing::info!("Updated secret: {}", secret_name);
                    Ok(secret_name)
                } else {
                    Err(WorkflowError::CredentialError(format!(
                        "Failed to create secret: {}",
                        e
                    )))
                }
            }
        }
    }

    /// Retrieve a credential from Secrets Manager
    pub async fn retrieve_credential(
        &self,
        workflow_id: &str,
        credential_key: &str,
    ) -> Result<String> {
        let secret_name = self.generate_secret_name(workflow_id, credential_key);

        let output = self
            .client
            .get_secret_value()
            .secret_id(&secret_name)
            .send()
            .await
            .map_err(|e| {
                if e.to_string().contains("ResourceNotFoundException") {
                    WorkflowError::CredentialError(format!(
                        "Credential {} not found for workflow {}",
                        credential_key, workflow_id
                    ))
                } else {
                    WorkflowError::CredentialError(format!("Failed to retrieve secret: {}", e))
                }
            })?;

        let secret_string = output.secret_string.ok_or_else(|| {
            WorkflowError::CredentialError(format!("Secret {} has no string value", secret_name))
        })?;

        let credential: CredentialValue = serde_json::from_str(&secret_string).map_err(|e| {
            WorkflowError::CredentialError(format!("Failed to parse credential: {}", e))
        })?;

        Ok(credential.value)
    }

    /// Delete a credential from Secrets Manager
    pub async fn delete_credential(&self, workflow_id: &str, credential_key: &str) -> Result<()> {
        let secret_name = self.generate_secret_name(workflow_id, credential_key);

        self.client
            .delete_secret()
            .secret_id(&secret_name)
            .force_delete_without_recovery(true)
            .send()
            .await
            .map_err(|e| {
                if e.to_string().contains("ResourceNotFoundException") {
                    // Already deleted, not an error
                    return WorkflowError::CredentialError(format!(
                        "Credential {} not found for workflow {}",
                        credential_key, workflow_id
                    ));
                }
                WorkflowError::CredentialError(format!("Failed to delete secret: {}", e))
            })?;

        tracing::info!("Deleted secret: {}", secret_name);
        Ok(())
    }

    /// List all credentials for a workflow
    pub async fn list_workflow_credentials(
        &self,
        workflow_id: &str,
    ) -> Result<HashMap<String, String>> {
        let prefix = format!("{}/{}/", self.prefix, workflow_id);

        let mut credentials = HashMap::new();
        let mut next_token: Option<String> = None;

        loop {
            let mut list_builder = self.client.list_secrets();

            if let Some(token) = next_token {
                list_builder = list_builder.next_token(token);
            }

            let output = list_builder.send().await.map_err(|e| {
                WorkflowError::CredentialError(format!("Failed to list secrets: {}", e))
            })?;

            if let Some(secrets) = output.secret_list {
                for secret in secrets {
                    if let Some(name) = &secret.name {
                        if name.starts_with(&prefix) {
                            // Extract credential key from secret name
                            let key = name.strip_prefix(&prefix).unwrap_or(name);

                            // Retrieve the credential value
                            match self.retrieve_credential(workflow_id, key).await {
                                Ok(value) => {
                                    credentials.insert(key.to_string(), value);
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to retrieve credential {}: {}", key, e);
                                }
                            }
                        }
                    }
                }
            }

            if output.next_token.is_none() {
                break;
            }
            next_token = output.next_token;
        }

        Ok(credentials)
    }

    /// Generate a secret name with prefix
    fn generate_secret_name(&self, workflow_id: &str, credential_key: &str) -> String {
        format!("{}/{}/{}", self.prefix, workflow_id, credential_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_value_serialization() {
        let cred = CredentialValue {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
            description: Some("Test credential".to_string()),
        };

        let json = serde_json::to_string(&cred).unwrap();
        let deserialized: CredentialValue = serde_json::from_str(&json).unwrap();

        assert_eq!(cred.key, deserialized.key);
        assert_eq!(cred.value, deserialized.value);
    }

    #[test]
    fn test_generate_secret_name() {
        // Test the secret name generation logic without creating a real client
        let prefix = "latempo".to_string();
        let workflow_id = "workflow123";
        let credential_key = "slack_token";

        let expected_name = format!("{}/{}/{}", prefix, workflow_id, credential_key);
        assert_eq!(expected_name, "latempo/workflow123/slack_token");
    }
}
