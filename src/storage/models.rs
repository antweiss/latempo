/// Storage models for workflow persistence
use crate::compiler::code_generator::GeneratedCode;
use crate::compiler::dsl::models::WorkflowSpec;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Workflow deployment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStatus {
    /// Workflow created but not deployed
    Created,
    /// Deployment in progress
    Deploying,
    /// Successfully deployed and running
    Active,
    /// Deployment failed
    Failed,
    /// Workflow stopped/deleted
    Inactive,
}

/// Complete workflow record for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRecord {
    /// Unique workflow ID
    pub id: Uuid,
    /// Workflow name
    pub name: String,
    /// Original description
    pub description: String,
    /// Workflow specification (DSL)
    pub spec: WorkflowSpec,
    /// Generated code artifacts
    pub generated_code: GeneratedCode,
    /// Current deployment status
    pub status: WorkflowStatus,
    /// AWS ECS service ARN (if deployed)
    pub service_arn: Option<String>,
    /// AWS API Gateway webhook URL (if API trigger)
    pub webhook_url: Option<String>,
    /// S3 artifact location
    pub s3_artifact_uri: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Deployment error message (if failed)
    pub error_message: Option<String>,
    /// Tags for organization
    pub tags: Vec<String>,
}

impl WorkflowRecord {
    /// Create a new workflow record
    pub fn new(spec: WorkflowSpec, generated_code: GeneratedCode, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: spec.id,
            name: spec.name.clone(),
            description,
            spec,
            generated_code,
            status: WorkflowStatus::Created,
            service_arn: None,
            webhook_url: None,
            s3_artifact_uri: None,
            created_at: now,
            updated_at: now,
            error_message: None,
            tags: Vec::new(),
        }
    }

    /// Update workflow status
    pub fn update_status(&mut self, status: WorkflowStatus, error: Option<String>) {
        self.status = status;
        self.error_message = error;
        self.updated_at = Utc::now();
    }

    /// Mark as deployed
    pub fn mark_deployed(&mut self, service_arn: String, webhook_url: Option<String>) {
        self.status = WorkflowStatus::Active;
        self.service_arn = Some(service_arn);
        self.webhook_url = webhook_url;
        self.updated_at = Utc::now();
    }

    /// Mark as failed
    pub fn mark_failed(&mut self, error: String) {
        self.status = WorkflowStatus::Failed;
        self.error_message = Some(error);
        self.updated_at = Utc::now();
    }
}

/// Query filter for listing workflows
#[derive(Debug, Clone, Default)]
pub struct WorkflowFilter {
    pub status: Option<WorkflowStatus>,
    pub tags: Vec<String>,
    pub limit: Option<usize>,
}
