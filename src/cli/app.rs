use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "latempo")]
#[command(version, about = "AI Workflow Generator - Transform natural language into production-ready Temporal workflows", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new workflow from a description
    Create {
        /// Workflow description (will prompt if not provided)
        description: Option<String>,

        /// Skip the dry-run step
        #[arg(long)]
        skip_dry_run: bool,

        /// Automatically deploy after successful dry-run
        #[arg(long)]
        auto_deploy: bool,
    },

    /// List all workflows
    List {
        /// Filter by status (e.g., deployed, active, stopped)
        #[arg(long)]
        status: Option<String>,
    },

    /// Test a workflow with a dry-run
    Test {
        /// Workflow ID to test
        workflow_id: String,

        /// Path to custom mock data JSON file
        #[arg(long)]
        mock_data: Option<String>,
    },

    /// Deploy a workflow to AWS
    Deploy {
        /// Workflow ID to deploy
        workflow_id: String,

        /// Force deployment even if already deployed
        #[arg(long)]
        force: bool,
    },

    /// Delete a workflow and its resources
    Delete {
        /// Workflow ID to delete
        workflow_id: String,

        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
    },

    /// Show details about a workflow
    Show {
        /// Workflow ID to display
        workflow_id: String,
    },
}
