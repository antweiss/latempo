use clap::Parser;
use latempo::cli::{Cli, Commands};
use latempo::config::Settings;
use latempo::utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    utils::logger::init_logger();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load settings from environment
    let settings = Settings::from_env()?;
    settings.validate()?;

    // Execute command
    let result = match cli.command {
        Commands::Create {
            description,
            skip_dry_run,
            auto_deploy,
        } => {
            latempo::cli::commands::create_workflow(description, skip_dry_run, auto_deploy, &settings)
                .await
        }
        Commands::List { status } => {
            latempo::cli::commands::list_workflows(status, &settings).await
        }
        Commands::Test {
            workflow_id,
            mock_data,
        } => latempo::cli::commands::test_workflow(workflow_id, mock_data, &settings).await,
        Commands::Deploy { workflow_id, force } => {
            latempo::cli::commands::deploy_workflow(workflow_id, force, &settings).await
        }
        Commands::Delete { workflow_id, yes } => {
            latempo::cli::commands::delete_workflow(workflow_id, yes, &settings).await
        }
        Commands::Show { workflow_id } => {
            latempo::cli::commands::show_workflow(workflow_id, &settings).await
        }
    };

    // Handle errors gracefully
    if let Err(e) = result {
        latempo::cli::ui::display_error(&format!("Error: {}", e));
        std::process::exit(1);
    }

    Ok(())
}
