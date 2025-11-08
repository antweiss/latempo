use super::models::{ToolCategory, ToolDefinition};
use crate::utils::{Result, WorkflowError};
use std::collections::HashMap;
use std::sync::Arc;

/// Registry for managing available tools
#[derive(Debug, Clone)]
pub struct ToolRegistry {
    tools: Arc<HashMap<String, ToolDefinition>>,
}

impl ToolRegistry {
    /// Create a new registry with default tools
    pub fn new() -> Self {
        let mut tools = HashMap::new();

        // Register all default tools
        for tool in Self::default_tools() {
            tools.insert(tool.name.clone(), tool);
        }

        Self {
            tools: Arc::new(tools),
        }
    }

    /// Get a tool by name
    pub fn get_tool(&self, name: &str) -> Option<&ToolDefinition> {
        self.tools.get(name)
    }

    /// List all tools
    pub fn list_tools(&self) -> Vec<&ToolDefinition> {
        self.tools.values().collect()
    }

    /// List tools by category
    pub fn list_by_category(&self, category: &ToolCategory) -> Vec<&ToolDefinition> {
        self.tools
            .values()
            .filter(|t| &t.category == category)
            .collect()
    }

    /// Search tools by query string (searches name and description)
    pub fn search_tools(&self, query: &str) -> Vec<&ToolDefinition> {
        let query_lower = query.to_lowercase();
        self.tools
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower)
                    || t.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Validate that a tool exists
    pub fn validate_tool(&self, name: &str) -> Result<()> {
        if self.tools.contains_key(name) {
            Ok(())
        } else {
            Err(WorkflowError::ToolNotFound(name.to_string()))
        }
    }

    /// Get the default tools
    fn default_tools() -> Vec<ToolDefinition> {
        vec![
            super::definitions::messaging::slack_send_message(),
            super::definitions::ai::run_claude_agent(),
            super::definitions::ai::run_openai_agent(),
            super::definitions::travel::search_flights(),
            super::definitions::travel::book_flight(),
            super::definitions::utilities::http_request(),
        ]
    }

    /// Get all required credentials for a set of tools
    pub fn get_required_credentials(&self, tool_names: &[String]) -> Result<Vec<String>> {
        let mut credentials = Vec::new();

        for name in tool_names {
            let tool = self
                .get_tool(name)
                .ok_or_else(|| WorkflowError::ToolNotFound(name.clone()))?;

            for cred in &tool.credentials_required {
                if !credentials.contains(cred) {
                    credentials.push(cred.clone());
                }
            }
        }

        Ok(credentials)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert!(!registry.list_tools().is_empty());
    }

    #[test]
    fn test_get_tool() {
        let registry = ToolRegistry::new();
        let tool = registry.get_tool("slack_send_message");
        assert!(tool.is_some());
        assert_eq!(tool.unwrap().name, "slack_send_message");
    }

    #[test]
    fn test_search_tools() {
        let registry = ToolRegistry::new();
        let results = registry.search_tools("slack");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_validate_tool() {
        let registry = ToolRegistry::new();
        assert!(registry.validate_tool("slack_send_message").is_ok());
        assert!(registry.validate_tool("nonexistent_tool").is_err());
    }
}
