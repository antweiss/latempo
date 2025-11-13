use crate::analyzer::requirements::WorkflowRequirements;
use crate::tools::{ToolDefinition, ToolRegistry};
use crate::utils::{Result, WorkflowError};
use std::collections::HashSet;
use std::sync::Arc;

/// Selects and validates tools for workflows
pub struct ToolSelector {
    registry: Arc<ToolRegistry>,
}

impl ToolSelector {
    /// Create a new tool selector
    pub fn new(registry: Arc<ToolRegistry>) -> Self {
        Self { registry }
    }

    /// Select tools based on requirements
    pub fn select_tools(
        &self,
        requirements: &WorkflowRequirements,
    ) -> Result<Vec<ToolDefinition>> {
        let mut tools = Vec::new();

        for tool_name in &requirements.tools_needed {
            let tool = self
                .registry
                .get_tool(tool_name)
                .ok_or_else(|| WorkflowError::ToolNotFound(tool_name.clone()))?;

            tools.push(tool.clone());
        }

        // Validate tool compatibility
        self.validate_tool_compatibility(&tools)?;

        Ok(tools)
    }

    /// Validate that tools are compatible with each other
    pub fn validate_tool_compatibility(&self, tools: &[ToolDefinition]) -> Result<()> {
        // For now, all tools are compatible
        // Future: Add compatibility rules (e.g., conflicting tools, required combinations)

        if tools.is_empty() {
            return Err(WorkflowError::validation(
                "Workflow must use at least one tool",
            ));
        }

        Ok(())
    }

    /// Resolve all required credentials for the selected tools
    pub fn resolve_credentials(&self, tools: &[ToolDefinition]) -> Vec<String> {
        let mut credentials = HashSet::new();

        for tool in tools {
            for cred in &tool.credentials_required {
                credentials.insert(cred.clone());
            }
        }

        let mut cred_vec: Vec<String> = credentials.into_iter().collect();
        cred_vec.sort();
        cred_vec
    }

    /// Get estimated duration for all tools
    pub fn estimate_total_duration(&self, tools: &[ToolDefinition]) -> u32 {
        tools.iter().map(|t| t.estimated_duration_secs).sum()
    }

    /// Get estimated cost for all tools (per execution)
    pub fn estimate_total_cost(&self, tools: &[ToolDefinition]) -> f64 {
        tools.iter().map(|t| t.cost_per_call_usd).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::requirements::{RequiredStep, StepType, TriggerType};

    #[test]
    fn test_select_tools() {
        let registry = Arc::new(ToolRegistry::new());
        let selector = ToolSelector::new(registry);

        let requirements = WorkflowRequirements {
            name: "test".to_string(),
            description: "test".to_string(),
            tools_needed: vec!["slack_send_message".to_string()],
            credentials_needed: vec![],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![RequiredStep {
                step_id: "step1".to_string(),
                step_type: StepType::ApiCall,
                description: "Send message".to_string(),
                tool_name: Some("slack_send_message".to_string()),
                depends_on: vec![],
            }],
        };

        let tools = selector.select_tools(&requirements).unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "slack_send_message");
    }

    #[test]
    fn test_select_nonexistent_tool() {
        let registry = Arc::new(ToolRegistry::new());
        let selector = ToolSelector::new(registry);

        let requirements = WorkflowRequirements {
            name: "test".to_string(),
            description: "test".to_string(),
            tools_needed: vec!["nonexistent_tool".to_string()],
            credentials_needed: vec![],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![],
        };

        assert!(selector.select_tools(&requirements).is_err());
    }

    #[test]
    fn test_resolve_credentials() {
        let registry = Arc::new(ToolRegistry::new());
        let selector = ToolSelector::new(registry);

        let requirements = WorkflowRequirements {
            name: "test".to_string(),
            description: "test".to_string(),
            tools_needed: vec!["slack_send_message".to_string(), "run_claude_agent".to_string()],
            credentials_needed: vec![],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![],
        };

        let tools = selector.select_tools(&requirements).unwrap();
        let credentials = selector.resolve_credentials(&tools);

        // Should have both slack and anthropic credentials
        assert!(credentials.contains(&"slack/bot_token".to_string()));
        assert!(credentials.contains(&"anthropic/api_key".to_string()));
    }

    #[test]
    fn test_estimate_duration() {
        let registry = Arc::new(ToolRegistry::new());
        let selector = ToolSelector::new(registry);

        let requirements = WorkflowRequirements {
            name: "test".to_string(),
            description: "test".to_string(),
            tools_needed: vec!["slack_send_message".to_string()],
            credentials_needed: vec![],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![],
        };

        let tools = selector.select_tools(&requirements).unwrap();
        let duration = selector.estimate_total_duration(&tools);

        assert!(duration > 0);
    }
}
