use crate::cli::ui::{display_error, display_info, display_success};
use crate::compiler::code_generator::PythonCodeGenerator;
use crate::compiler::dsl::models::WorkflowSpec;
use crate::config::Settings;
use crate::testing::{DryRunConfig, DryRunner, MockDataSet};
use crate::tools::registry::ToolRegistry;
use crate::utils::{Result, WorkflowError};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub async fn execute(
    workflow_id: String,
    mock_data_path: Option<String>,
    _settings: &Settings,
) -> Result<()> {
    display_info(&format!(
        "🧪 Running dry-run test for workflow: {}",
        style(&workflow_id).cyan().bold()
    ));

    // Create progress indicator
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Step 1: Load workflow spec
    spinner.set_message("Loading workflow specification...");
    let spec = load_workflow_spec(&workflow_id)?;
    spinner.finish_and_clear();
    display_success(&format!("Loaded workflow: {}", spec.name));

    // Step 2: Generate code
    spinner.reset();
    spinner.set_message("Generating Python code...");
    let code_generator = PythonCodeGenerator::new()?;
    let generated_code = code_generator.generate_all(&spec)?;
    spinner.finish_and_clear();
    display_success("Code generated successfully");

    // Step 3: Load or generate mock data
    spinner.reset();
    spinner.set_message("Preparing mock data...");
    let mock_data = if let Some(path) = mock_data_path {
        load_mock_data(&path)?
    } else {
        None
    };
    spinner.finish_and_clear();
    if mock_data.is_some() {
        display_success("Mock data loaded");
    } else {
        display_info("Will generate mock data automatically");
    }

    // Step 4: Execute dry-run
    spinner.reset();
    spinner.set_message("Executing workflow in dry-run mode...");
    let tool_registry = ToolRegistry::new();
    let dry_runner = DryRunner::new(tool_registry);

    let config = DryRunConfig {
        workflow_id: workflow_id.clone(),
        mock_data,
        timeout_secs: 120,
        save_artifacts: true,
        artifacts_dir: Some(PathBuf::from("./artifacts")),
    };

    let result = dry_runner.run(&spec, &generated_code, config).await?;
    spinner.finish_and_clear();

    // Step 5: Display results
    println!("\n{}", style("━".repeat(60)).dim());
    println!(
        "{}",
        style("                    TEST RESULTS                    ")
            .bold()
            .cyan()
    );
    println!("{}\n", style("━".repeat(60)).dim());

    if result.success {
        display_success("✓ All steps completed successfully");
    } else {
        display_error("✗ Workflow execution failed");
        if let Some(error) = result.error_message {
            println!("\n{}\n{}\n", style("Error details:").red().bold(), error);
        }
    }

    // Display trace summary
    let summary = &result.trace_summary;
    println!(
        "{}",
        style(format!(
            "Total steps: {} | Success: {} | Failed: {}",
            summary.total_steps, summary.successful_steps, summary.failed_steps
        ))
        .cyan()
    );
    println!(
        "{}",
        style(format!(
            "Tool invocations: {}",
            summary.total_tool_invocations
        ))
        .cyan()
    );
    println!(
        "{}",
        style(format!(
            "Total duration: {:.2}s",
            summary.total_duration_ms as f64 / 1000.0
        ))
        .cyan()
    );
    println!(
        "{}",
        style(format!(
            "💰 Estimated cost per execution: ${:.4}",
            summary.estimated_cost
        ))
        .yellow()
        .bold()
    );

    // Display step durations
    if !summary.step_durations.is_empty() {
        println!("\n{}", style("Step durations:").bold());
        for (step_id, duration_ms) in &summary.step_durations {
            println!(
                "  {} {:.2}s",
                style(format!("• {}:", step_id)).dim(),
                *duration_ms as f64 / 1000.0
            );
        }
    }

    // Display artifacts location
    if let Some(artifacts_path) = result.artifacts_path {
        println!(
            "\n{} {:?}",
            style("📦 Artifacts saved to:").green(),
            artifacts_path
        );
    }

    println!("\n{}", style("━".repeat(60)).dim());

    if !result.success {
        return Err(WorkflowError::ExecutionError(
            "Dry-run test failed".to_string(),
        ));
    }

    Ok(())
}

/// Load workflow specification from storage
fn load_workflow_spec(workflow_id: &str) -> Result<WorkflowSpec> {
    // For now, create a simple test workflow spec
    // In Phase 4, this will load from DynamoDB
    let spec_path = PathBuf::from("./workflows")
        .join(workflow_id)
        .join("spec.json");

    if spec_path.exists() {
        let spec_json = fs::read_to_string(&spec_path).map_err(|e| WorkflowError::IoError(e))?;
        serde_json::from_str(&spec_json)
            .map_err(|e| WorkflowError::ValidationError(format!("Invalid workflow spec: {}", e)))
    } else {
        Err(WorkflowError::ValidationError(format!(
            "Workflow not found: {}. Expected at: {:?}",
            workflow_id, spec_path
        )))
    }
}

/// Load mock data from file
fn load_mock_data(path: &str) -> Result<Option<MockDataSet>> {
    let mock_json = fs::read_to_string(path).map_err(|e| WorkflowError::IoError(e))?;

    let mock_data: MockDataSet = serde_json::from_str(&mock_json)
        .map_err(|e| WorkflowError::ValidationError(format!("Invalid mock data: {}", e)))?;

    Ok(Some(mock_data))
}
