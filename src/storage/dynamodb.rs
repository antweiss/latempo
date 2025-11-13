/// DynamoDB storage implementation for workflows
use crate::storage::models::{WorkflowFilter, WorkflowRecord, WorkflowStatus};
use crate::utils::errors::{Result, WorkflowError};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use std::collections::HashMap;
use uuid::Uuid;

/// DynamoDB storage client for workflows
pub struct DynamoDbStorage {
    client: DynamoDbClient,
    table_name: String,
}

impl DynamoDbStorage {
    /// Create a new DynamoDB storage client
    pub fn new(client: DynamoDbClient, table_name: String) -> Self {
        Self { client, table_name }
    }

    /// Save a workflow record
    pub async fn save_workflow(&self, record: &WorkflowRecord) -> Result<()> {
        let item = self.record_to_item(record)?;

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| WorkflowError::StorageError(format!("Failed to save workflow: {}", e)))?;

        tracing::info!("Saved workflow {} to DynamoDB", record.id);
        Ok(())
    }

    /// Get a workflow by ID
    pub async fn get_workflow(&self, workflow_id: Uuid) -> Result<Option<WorkflowRecord>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(workflow_id.to_string()))
            .send()
            .await
            .map_err(|e| WorkflowError::StorageError(format!("Failed to get workflow: {}", e)))?;

        if let Some(item) = result.item {
            let record = self.item_to_record(item)?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    /// List all workflows with optional filtering
    pub async fn list_workflows(&self, filter: WorkflowFilter) -> Result<Vec<WorkflowRecord>> {
        let mut scan_builder = self.client.scan().table_name(&self.table_name);

        // Apply status filter if provided
        if let Some(status) = &filter.status {
            let status_str = serde_json::to_string(status).map_err(|e| {
                WorkflowError::StorageError(format!("Failed to serialize status: {}", e))
            })?;
            scan_builder = scan_builder
                .filter_expression("workflow_status = :status")
                .expression_attribute_values(":status", AttributeValue::S(status_str));
        }

        // Apply limit if provided
        if let Some(limit) = filter.limit {
            scan_builder = scan_builder.limit(limit as i32);
        }

        let result = scan_builder
            .send()
            .await
            .map_err(|e| WorkflowError::StorageError(format!("Failed to list workflows: {}", e)))?;

        let mut records = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                match self.item_to_record(item) {
                    Ok(record) => {
                        // Apply tag filter if provided
                        if !filter.tags.is_empty() {
                            if filter.tags.iter().any(|tag| record.tags.contains(tag)) {
                                records.push(record);
                            }
                        } else {
                            records.push(record);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to deserialize workflow record: {}", e);
                    }
                }
            }
        }

        Ok(records)
    }

    /// Update workflow status
    pub async fn update_workflow_status(
        &self,
        workflow_id: Uuid,
        status: WorkflowStatus,
        error_message: Option<String>,
    ) -> Result<()> {
        let status_str = serde_json::to_string(&status).map_err(|e| {
            WorkflowError::StorageError(format!("Failed to serialize status: {}", e))
        })?;

        let updated_at = chrono::Utc::now().to_rfc3339();

        let mut update_builder = self
            .client
            .update_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(workflow_id.to_string()))
            .update_expression("SET workflow_status = :status, updated_at = :updated");

        update_builder = update_builder
            .expression_attribute_values(":status", AttributeValue::S(status_str))
            .expression_attribute_values(":updated", AttributeValue::S(updated_at));

        if let Some(error) = error_message {
            update_builder = update_builder
                .update_expression(
                    "SET workflow_status = :status, updated_at = :updated, error_message = :error",
                )
                .expression_attribute_values(":error", AttributeValue::S(error));
        }

        update_builder.send().await.map_err(|e| {
            WorkflowError::StorageError(format!("Failed to update workflow status: {}", e))
        })?;

        tracing::info!("Updated workflow {} status to {:?}", workflow_id, status);
        Ok(())
    }

    /// Delete a workflow
    pub async fn delete_workflow(&self, workflow_id: Uuid) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(workflow_id.to_string()))
            .send()
            .await
            .map_err(|e| {
                WorkflowError::StorageError(format!("Failed to delete workflow: {}", e))
            })?;

        tracing::info!("Deleted workflow {}", workflow_id);
        Ok(())
    }

    /// Convert WorkflowRecord to DynamoDB item
    fn record_to_item(&self, record: &WorkflowRecord) -> Result<HashMap<String, AttributeValue>> {
        let json = serde_json::to_string(record).map_err(|e| {
            WorkflowError::StorageError(format!("Failed to serialize workflow record: {}", e))
        })?;

        let mut item = HashMap::new();
        item.insert("id".to_string(), AttributeValue::S(record.id.to_string()));
        item.insert("name".to_string(), AttributeValue::S(record.name.clone()));
        item.insert(
            "workflow_status".to_string(),
            AttributeValue::S(serde_json::to_string(&record.status).unwrap()),
        );
        item.insert(
            "created_at".to_string(),
            AttributeValue::S(record.created_at.to_rfc3339()),
        );
        item.insert(
            "updated_at".to_string(),
            AttributeValue::S(record.updated_at.to_rfc3339()),
        );
        item.insert("data".to_string(), AttributeValue::S(json));

        Ok(item)
    }

    /// Convert DynamoDB item to WorkflowRecord
    fn item_to_record(&self, item: HashMap<String, AttributeValue>) -> Result<WorkflowRecord> {
        let data = item
            .get("data")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| {
                WorkflowError::StorageError("Missing or invalid data field".to_string())
            })?;

        serde_json::from_str(data).map_err(|e| {
            WorkflowError::StorageError(format!("Failed to deserialize workflow record: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_filter_default() {
        let filter = WorkflowFilter::default();
        assert!(filter.status.is_none());
        assert!(filter.tags.is_empty());
        assert!(filter.limit.is_none());
    }

    #[test]
    fn test_workflow_filter_with_status() {
        let filter = WorkflowFilter {
            status: Some(WorkflowStatus::Active),
            tags: vec![],
            limit: None,
        };
        assert!(filter.status.is_some());
    }
}
