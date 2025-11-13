/// Trace collection and analysis for workflow execution
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single event in the workflow execution trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: TraceEventType,
    pub step_id: String,
    pub message: String,
    pub duration_ms: Option<u64>,
    pub metadata: serde_json::Value,
}

/// Types of trace events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TraceEventType {
    WorkflowStart,
    WorkflowEnd,
    StepStart,
    StepEnd,
    StepError,
    ApprovalRequested,
    ApprovalReceived,
    ToolInvocation,
    ToolResponse,
    ConditionalEvaluation,
    Log,
}

/// Summary of workflow execution trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSummary {
    pub total_duration_ms: u64,
    pub total_steps: usize,
    pub successful_steps: usize,
    pub failed_steps: usize,
    pub total_tool_invocations: usize,
    pub total_approvals: usize,
    pub step_durations: Vec<(String, u64)>,
    pub estimated_cost: f64,
}

/// Collector for parsing and analyzing execution traces
pub struct TraceCollector {
    events: Vec<TraceEvent>,
}

impl TraceCollector {
    /// Create a new trace collector
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Parse log output into trace events
    pub fn parse_logs(&mut self, logs: &str) -> usize {
        let mut parsed_count = 0;

        for line in logs.lines() {
            if let Some(event) = self.parse_log_line(line) {
                self.events.push(event);
                parsed_count += 1;
            }
        }

        parsed_count
    }

    /// Parse a single log line into a trace event
    fn parse_log_line(&self, line: &str) -> Option<TraceEvent> {
        // Try to parse JSON structured logs
        if let Ok(json_event) = serde_json::from_str::<serde_json::Value>(line) {
            return self.parse_json_event(&json_event);
        }

        // Try to parse simple text logs with patterns
        if line.contains("[WORKFLOW_START]") {
            return Some(TraceEvent {
                timestamp: Utc::now(),
                event_type: TraceEventType::WorkflowStart,
                step_id: "workflow".to_string(),
                message: "Workflow execution started".to_string(),
                duration_ms: None,
                metadata: serde_json::json!({}),
            });
        }

        if line.contains("[WORKFLOW_END]") {
            return Some(TraceEvent {
                timestamp: Utc::now(),
                event_type: TraceEventType::WorkflowEnd,
                step_id: "workflow".to_string(),
                message: "Workflow execution completed".to_string(),
                duration_ms: None,
                metadata: serde_json::json!({}),
            });
        }

        if line.contains("[STEP_START]") {
            let step_id = self.extract_value(line, "step_id:").unwrap_or_default();
            return Some(TraceEvent {
                timestamp: Utc::now(),
                event_type: TraceEventType::StepStart,
                step_id,
                message: line.to_string(),
                duration_ms: None,
                metadata: serde_json::json!({}),
            });
        }

        if line.contains("[STEP_END]") {
            let step_id = self.extract_value(line, "step_id:").unwrap_or_default();
            let duration_ms = self.extract_duration(line);
            return Some(TraceEvent {
                timestamp: Utc::now(),
                event_type: TraceEventType::StepEnd,
                step_id,
                message: line.to_string(),
                duration_ms,
                metadata: serde_json::json!({}),
            });
        }

        None
    }

    /// Parse a JSON log event
    fn parse_json_event(&self, json: &serde_json::Value) -> Option<TraceEvent> {
        let event_type_str = json.get("event_type")?.as_str()?;
        let event_type = match event_type_str {
            "workflow_start" => TraceEventType::WorkflowStart,
            "workflow_end" => TraceEventType::WorkflowEnd,
            "step_start" => TraceEventType::StepStart,
            "step_end" => TraceEventType::StepEnd,
            "step_error" => TraceEventType::StepError,
            "approval_requested" => TraceEventType::ApprovalRequested,
            "approval_received" => TraceEventType::ApprovalReceived,
            "tool_invocation" => TraceEventType::ToolInvocation,
            "tool_response" => TraceEventType::ToolResponse,
            "conditional_evaluation" => TraceEventType::ConditionalEvaluation,
            _ => TraceEventType::Log,
        };

        Some(TraceEvent {
            timestamp: Utc::now(),
            event_type,
            step_id: json
                .get("step_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            message: json
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            duration_ms: json.get("duration_ms").and_then(|v| v.as_u64()),
            metadata: json
                .get("metadata")
                .cloned()
                .unwrap_or(serde_json::json!({})),
        })
    }

    /// Extract a value from a text line
    fn extract_value(&self, line: &str, prefix: &str) -> Option<String> {
        line.find(prefix).map(|start| {
            let value_start = start + prefix.len();
            let rest = &line[value_start..];
            rest.split_whitespace().next().unwrap_or("").to_string()
        })
    }

    /// Extract duration from a text line
    fn extract_duration(&self, line: &str) -> Option<u64> {
        self.extract_value(line, "duration_ms:")
            .and_then(|s| s.parse().ok())
    }

    /// Generate a summary of the trace
    pub fn summarize(&self) -> TraceSummary {
        let mut total_duration_ms = 0u64;
        let mut successful_steps = 0;
        let mut failed_steps = 0;
        let mut tool_invocations = 0;
        let mut approvals = 0;
        let mut step_durations = Vec::new();

        let mut step_starts = std::collections::HashMap::new();

        for event in &self.events {
            match event.event_type {
                TraceEventType::StepStart => {
                    step_starts.insert(event.step_id.clone(), event.timestamp);
                }
                TraceEventType::StepEnd => {
                    successful_steps += 1;
                    if let Some(duration) = event.duration_ms {
                        total_duration_ms += duration;
                        step_durations.push((event.step_id.clone(), duration));
                    }
                }
                TraceEventType::StepError => {
                    failed_steps += 1;
                }
                TraceEventType::ToolInvocation => {
                    tool_invocations += 1;
                }
                TraceEventType::ApprovalRequested => {
                    approvals += 1;
                }
                _ => {}
            }
        }

        let total_steps = successful_steps + failed_steps;

        TraceSummary {
            total_duration_ms,
            total_steps,
            successful_steps,
            failed_steps,
            total_tool_invocations: tool_invocations,
            total_approvals: approvals,
            step_durations,
            estimated_cost: 0.0, // Will be calculated by cost estimator
        }
    }

    /// Format trace for display
    pub fn format_trace(&self) -> String {
        let mut output = String::new();
        output.push_str("═══════════════════════════════════════════════════════════\n");
        output.push_str("                    EXECUTION TRACE                        \n");
        output.push_str("═══════════════════════════════════════════════════════════\n\n");

        for event in &self.events {
            let symbol = match event.event_type {
                TraceEventType::WorkflowStart => "🚀",
                TraceEventType::WorkflowEnd => "🏁",
                TraceEventType::StepStart => "▶",
                TraceEventType::StepEnd => "✓",
                TraceEventType::StepError => "✗",
                TraceEventType::ApprovalRequested => "⏸",
                TraceEventType::ApprovalReceived => "▶",
                TraceEventType::ToolInvocation => "🔧",
                TraceEventType::ToolResponse => "📦",
                TraceEventType::ConditionalEvaluation => "🔀",
                TraceEventType::Log => "ℹ",
            };

            let duration_str = if let Some(duration) = event.duration_ms {
                format!(" ({}ms)", duration)
            } else {
                String::new()
            };

            output.push_str(&format!(
                "[{}] {} {} - {}{}\n",
                event.timestamp.format("%H:%M:%S%.3f"),
                symbol,
                event.step_id,
                event.message,
                duration_str
            ));
        }

        output.push_str("\n═══════════════════════════════════════════════════════════\n");

        output
    }

    /// Get all events
    pub fn events(&self) -> &[TraceEvent] {
        &self.events
    }

    /// Add an event manually
    pub fn add_event(&mut self, event: TraceEvent) {
        self.events.push(event);
    }
}

impl Default for TraceCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_creation() {
        let collector = TraceCollector::new();
        assert_eq!(collector.events.len(), 0);
    }

    #[test]
    fn test_parse_simple_logs() {
        let mut collector = TraceCollector::new();
        let logs = r#"
[WORKFLOW_START] Starting workflow execution
[STEP_START] step_id: step1
[STEP_END] step_id: step1 duration_ms: 1500
[WORKFLOW_END] Workflow completed
        "#;

        let count = collector.parse_logs(logs);
        assert_eq!(count, 4);
        assert_eq!(collector.events.len(), 4);
    }

    #[test]
    fn test_parse_json_logs() {
        let mut collector = TraceCollector::new();
        let logs = r#"{"event_type":"workflow_start","step_id":"workflow","message":"Starting"}"#;

        let count = collector.parse_logs(logs);
        assert_eq!(count, 1);
        assert_eq!(
            collector.events[0].event_type,
            TraceEventType::WorkflowStart
        );
    }

    #[test]
    fn test_summarize_empty() {
        let collector = TraceCollector::new();
        let summary = collector.summarize();

        assert_eq!(summary.total_steps, 0);
        assert_eq!(summary.successful_steps, 0);
        assert_eq!(summary.failed_steps, 0);
    }

    #[test]
    fn test_summarize_with_events() {
        let mut collector = TraceCollector::new();

        collector.add_event(TraceEvent {
            timestamp: Utc::now(),
            event_type: TraceEventType::StepStart,
            step_id: "step1".to_string(),
            message: "Starting step".to_string(),
            duration_ms: None,
            metadata: serde_json::json!({}),
        });

        collector.add_event(TraceEvent {
            timestamp: Utc::now(),
            event_type: TraceEventType::StepEnd,
            step_id: "step1".to_string(),
            message: "Step completed".to_string(),
            duration_ms: Some(1500),
            metadata: serde_json::json!({}),
        });

        let summary = collector.summarize();
        assert_eq!(summary.total_steps, 1);
        assert_eq!(summary.successful_steps, 1);
        assert_eq!(summary.total_duration_ms, 1500);
    }

    #[test]
    fn test_format_trace() {
        let mut collector = TraceCollector::new();

        collector.add_event(TraceEvent {
            timestamp: Utc::now(),
            event_type: TraceEventType::WorkflowStart,
            step_id: "workflow".to_string(),
            message: "Starting workflow".to_string(),
            duration_ms: None,
            metadata: serde_json::json!({}),
        });

        let formatted = collector.format_trace();
        assert!(formatted.contains("EXECUTION TRACE"));
        assert!(formatted.contains("🚀"));
        assert!(formatted.contains("Starting workflow"));
    }

    #[test]
    fn test_add_event() {
        let mut collector = TraceCollector::new();

        let event = TraceEvent {
            timestamp: Utc::now(),
            event_type: TraceEventType::ToolInvocation,
            step_id: "tool_step".to_string(),
            message: "Calling tool".to_string(),
            duration_ms: Some(500),
            metadata: serde_json::json!({"tool": "slack"}),
        };

        collector.add_event(event);
        assert_eq!(collector.events().len(), 1);
        assert_eq!(
            collector.events()[0].event_type,
            TraceEventType::ToolInvocation
        );
    }
}
