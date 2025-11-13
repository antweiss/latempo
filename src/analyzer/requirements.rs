use crate::analyzer::ai_client::AnthropicClient;
use crate::tools::ToolRegistry;
use crate::utils::{Result, WorkflowError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info};

/// Requirements extracted from workflow description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequirements {
    pub name: String,
    pub description: String,
    pub tools_needed: Vec<String>,
    pub credentials_needed: Vec<String>,
    pub approval_points: Vec<ApprovalPoint>,
    pub trigger_type: TriggerType,
    pub steps: Vec<RequiredStep>,
}

/// Point in workflow requiring human approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalPoint {
    pub step_id: String,
    pub reason: String,
    pub timeout_minutes: u32,
}

/// Type of workflow trigger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    Webhook,
    Schedule,
    Manual,
}

/// Required step in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredStep {
    pub step_id: String,
    pub step_type: StepType,
    pub description: String,
    pub tool_name: Option<String>,
    pub depends_on: Vec<String>,
}

/// Type of workflow step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    Agent,
    ApiCall,
    Transform,
    Approval,
    Conditional,
}

/// Analyzer for extracting requirements from natural language
pub struct RequirementsAnalyzer {
    ai_client: AnthropicClient,
    tool_registry: Arc<ToolRegistry>,
}

impl RequirementsAnalyzer {
    /// Create a new requirements analyzer
    pub fn new(api_key: String, tool_registry: Arc<ToolRegistry>) -> Self {
        Self {
            ai_client: AnthropicClient::new(api_key, None),
            tool_registry,
        }
    }

    /// Analyze a workflow description and extract requirements
    pub async fn analyze(&self, description: &str) -> Result<WorkflowRequirements> {
        info!(
            "Analyzing workflow description ({} chars)",
            description.len()
        );

        // Validate description
        if description.trim().is_empty() {
            return Err(WorkflowError::validation(
                "Workflow description cannot be empty",
            ));
        }

        if description.len() < 20 {
            return Err(WorkflowError::validation(
                "Workflow description is too short. Please provide more details.",
            ));
        }

        // Build the analysis prompt
        let system_prompt = self.build_system_prompt();
        let user_prompt = self.build_user_prompt(description);

        // Call AI to analyze
        let response = self.ai_client.analyze(&system_prompt, &user_prompt).await?;

        // Parse JSON response
        let requirements = self.parse_response(&response)?;

        // Validate requirements
        self.validate_requirements(&requirements)?;

        info!(
            "Successfully analyzed workflow: {} tools, {} credentials, {} steps",
            requirements.tools_needed.len(),
            requirements.credentials_needed.len(),
            requirements.steps.len()
        );

        Ok(requirements)
    }

    /// Build the system prompt for the AI
    fn build_system_prompt(&self) -> String {
        let tools = self.tool_registry.list_tools();
        let tool_descriptions: Vec<String> = tools
            .iter()
            .map(|t| format!("- {}: {}", t.name, t.description))
            .collect();

        format!(
            r#"You are an expert at analyzing workflow descriptions and extracting structured requirements.

Your task is to analyze natural language workflow descriptions and identify:
1. What tools are needed (from the available tools list)
2. What credentials are required
3. Where human approval is needed
4. The logical steps to execute
5. The trigger type (webhook, schedule, or manual)

Available Tools:
{}

Guidelines:
- Only use tools from the available list above
- Identify approval points for financial transactions, data modifications, or sensitive operations
- Break the workflow into clear, logical steps
- Infer the trigger type from the description
- Extract credential requirements from the tools needed
- Ensure each step has dependencies (previous step IDs it needs)

Respond ONLY with valid JSON in this exact format:
{{
  "name": "short_workflow_name",
  "description": "brief description",
  "tools_needed": ["tool1", "tool2"],
  "credentials_needed": ["service/credential_name"],
  "approval_points": [
    {{
      "step_id": "step_id_needing_approval",
      "reason": "why approval is needed",
      "timeout_minutes": 30
    }}
  ],
  "trigger_type": "webhook",
  "steps": [
    {{
      "step_id": "step_1",
      "step_type": "agent",
      "description": "what this step does",
      "tool_name": "tool_name",
      "depends_on": []
    }}
  ]
}}

Valid step_types: "agent", "api_call", "transform", "approval", "conditional"
Valid trigger_types: "webhook", "schedule", "manual"
"#,
            tool_descriptions.join("\n")
        )
    }

    /// Build the user prompt
    fn build_user_prompt(&self, description: &str) -> String {
        format!(
            r#"Analyze this workflow description and extract the requirements:

{}

Return the analysis as JSON following the specified format."#,
            description
        )
    }

    /// Parse the AI response into requirements
    fn parse_response(&self, response: &str) -> Result<WorkflowRequirements> {
        debug!("Parsing AI response");

        // Extract JSON from response (handle code blocks)
        let json_str = self.extract_json(response)?;

        // Parse JSON
        let requirements: WorkflowRequirements = serde_json::from_str(&json_str).map_err(|e| {
            WorkflowError::analysis(format!("Failed to parse requirements JSON: {}", e))
        })?;

        Ok(requirements)
    }

    /// Extract JSON from response (handles markdown code blocks)
    fn extract_json(&self, response: &str) -> Result<String> {
        let response = response.trim();

        // Check if wrapped in code block
        if response.starts_with("```") {
            // Extract content between ``` markers
            let lines: Vec<&str> = response.lines().collect();
            if lines.len() < 3 {
                return Err(WorkflowError::analysis("Invalid code block format"));
            }

            // Skip first line (```json or ```) and last line (```)
            let json_lines = &lines[1..lines.len() - 1];
            Ok(json_lines.join("\n"))
        } else {
            // Assume the whole response is JSON
            Ok(response.to_string())
        }
    }

    /// Validate the extracted requirements
    fn validate_requirements(&self, requirements: &WorkflowRequirements) -> Result<()> {
        // Validate workflow name
        if requirements.name.is_empty() {
            return Err(WorkflowError::validation("Workflow name cannot be empty"));
        }

        // Validate tools exist
        for tool_name in &requirements.tools_needed {
            self.tool_registry.validate_tool(tool_name)?;
        }

        // Validate steps
        if requirements.steps.is_empty() {
            return Err(WorkflowError::validation(
                "Workflow must have at least one step",
            ));
        }

        // Validate step dependencies
        for step in &requirements.steps {
            for dep in &step.depends_on {
                if !requirements.steps.iter().any(|s| &s.step_id == dep) {
                    return Err(WorkflowError::validation(format!(
                        "Step '{}' depends on non-existent step '{}'",
                        step.step_id, dep
                    )));
                }
            }
        }

        // Validate approval points reference valid steps
        for approval in &requirements.approval_points {
            if !requirements
                .steps
                .iter()
                .any(|s| s.step_id == approval.step_id)
            {
                return Err(WorkflowError::validation(format!(
                    "Approval point references non-existent step '{}'",
                    approval.step_id
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_analyzer() -> RequirementsAnalyzer {
        let registry = Arc::new(ToolRegistry::new());
        RequirementsAnalyzer::new("test-key".to_string(), registry)
    }

    #[test]
    fn test_extract_json_plain() {
        let analyzer = create_test_analyzer();
        let json = r#"{"name": "test"}"#;
        let result = analyzer.extract_json(json).unwrap();
        assert_eq!(result, json);
    }

    #[test]
    fn test_extract_json_code_block() {
        let analyzer = create_test_analyzer();
        let response = r#"```json
{"name": "test"}
```"#;
        let result = analyzer.extract_json(response).unwrap();
        assert_eq!(result, r#"{"name": "test"}"#);
    }

    #[test]
    fn test_validate_requirements_empty_name() {
        let analyzer = create_test_analyzer();
        let requirements = WorkflowRequirements {
            name: "".to_string(),
            description: "test".to_string(),
            tools_needed: vec![],
            credentials_needed: vec![],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![RequiredStep {
                step_id: "step1".to_string(),
                step_type: StepType::Agent,
                description: "test".to_string(),
                tool_name: None,
                depends_on: vec![],
            }],
        };

        assert!(analyzer.validate_requirements(&requirements).is_err());
    }

    #[test]
    fn test_validate_requirements_invalid_tool() {
        let analyzer = create_test_analyzer();
        let requirements = WorkflowRequirements {
            name: "test".to_string(),
            description: "test".to_string(),
            tools_needed: vec!["nonexistent_tool".to_string()],
            credentials_needed: vec![],
            approval_points: vec![],
            trigger_type: TriggerType::Manual,
            steps: vec![RequiredStep {
                step_id: "step1".to_string(),
                step_type: StepType::Agent,
                description: "test".to_string(),
                tool_name: Some("nonexistent_tool".to_string()),
                depends_on: vec![],
            }],
        };

        assert!(analyzer.validate_requirements(&requirements).is_err());
    }
}
