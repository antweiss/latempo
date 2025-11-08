# AI Workflow Generator - Complete Implementation Specification

**Version:** 1.0  
**Core Language:** Rust 1.75+  
**Generated Code Language:** Python 3.11+  
**Target Environment:** AWS  
**Timeline:** 6-8 weeks

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Executive Summary](#executive-summary)
3. [Design Decisions](#design-decisions)
4. [System Architecture](#system-architecture)
5. [Technology Stack](#technology-stack)
6. [Project Structure](#project-structure)
7. [Phase 1: Foundation (Weeks 1-2)](#phase-1-foundation)
8. [Phase 2: Core Logic (Weeks 3-5)](#phase-2-core-logic)
9. [Phase 3: Testing Infrastructure (Week 6)](#phase-3-testing-infrastructure)
10. [Phase 4: Deployment & UI (Weeks 7-8)](#phase-4-deployment--ui)
11. [Data Models & Schemas](#data-models--schemas)
12. [Security Considerations](#security-considerations)
13. [Testing Strategy](#testing-strategy)
14. [Deployment Guide](#deployment-guide)
15. [Appendices](#appendices)

---

## Project Overview

### What is AI Workflow Generator?

A Rust-based tool that transforms natural language descriptions into production-ready, executable Temporal workflows with AI agents, deployed automatically to AWS.

### Core Concept

**Input:** "When a Slack message arrives on #travel-requests, extract details, search flights, request approval, and book the cheapest option if approved."

**Output:** 
- Python Temporal workflows with error handling
- AI agent configurations with optimal prompts
- Activity implementations for APIs
- Human-in-the-loop approval mechanisms
- AWS deployment with webhooks
- Monitoring and observability

### Use Cases

1. **Customer Support Automation** - Categorize tickets, search knowledge base, draft responses
2. **Travel Booking** - Extract details, search flights, book with approval
3. **Data Processing** - Parse files, transform data, analyze with AI
4. **Code Review** - AI code review, comment on issues, request human review
5. **Invoice Processing** - Extract data, validate, route for approval

---

## Executive Summary

### Problem Statement

Building event-driven workflows with AI requires expertise in:
- Workflow orchestration (Temporal, Airflow)
- Prompt engineering
- API integration
- Infrastructure deployment
- Security and credential management

This creates high barriers and slows development.

### Solution

AI Workflow Generator generates complete, production-ready systems from natural language:

1. **Intelligent Analysis** - AI understands requirements, identifies tools, determines approval points
2. **Code Generation** - Creates Python Temporal workflows with proper structure
3. **Credential Management** - Gathers and securely stores credentials in AWS Secrets Manager
4. **Validation** - Dry-runs workflows with mock data
5. **Deployment** - Automatically deploys to AWS (ECS, API Gateway, DynamoDB)
6. **Monitoring** - Sets up CloudWatch and Temporal observability

### Success Criteria

**Performance:**
- Generate code in < 2 minutes
- Dry-run in < 1 minute
- Deploy in < 5 minutes
- **Total: < 10 minutes from description to production**

**Quality:**
- 95%+ dry-run success rate
- Zero manual code writing
- Secure credential storage

**Scope:**
- 3-5 workflow patterns
- 5-10 pre-built tools
- Single-tenant (POC)

---

## Design Decisions

### 1. Rust for Core, Python for Runtime

**Rust for generator:** Type safety, performance, reliability for code generation infrastructure
**Python for workflows:** Mature Temporal SDK, best AI/ML ecosystem, easier to debug

### 2. Template-Based Code Generation

Use Tera (Jinja2-like) templates to generate Python. Separates business logic from code structure, maintainable, readable.

### 3. AWS-Native Deployment

Deploy to AWS exclusively for POC. Managed services, integrated security, auto-scaling, pay-per-use.

### 4. Temporal for Orchestration

Battle-tested workflow engine with durability, observability, built-in retries/timeouts, state management.

### 5. Interactive Credential Gathering

Prompt users interactively. Credentials never in files, immediate validation, guided process.

### 6. Mandatory Dry-Run

Test before deployment. Catch errors early, estimate costs, build confidence.

### 7. Curated Tool Library (POC)

Start with 5-10 tools. Prove concept before extensibility.

### 8. Human-in-the-Loop Approvals

Built-in approval via Temporal signals. Safety for critical actions, compliance, audit trail.

### 9. Single-Tenant for POC

No multi-tenancy initially. Simpler, faster to build, proves concept.

---

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────┐
│         CLI (Rust - Rich TUI)               │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│      WORKFLOW GENERATOR (Rust)              │
│  ┌──────────┐ ┌──────────┐ ┌────────────┐  │
│  │ Analyzer │→│ Compiler │→│  Generator │  │
│  │(AI Call) │ │  (DSL)   │ │(Py Code)   │  │
│  └──────────┘ └──────────┘ └────────────┘  │
│  ┌──────────┐ ┌──────────┐ ┌────────────┐  │
│  │Credential│ │ Dry-Run  │ │ Deployment │  │
│  │ Manager  │ │ Executor │ │   Engine   │  │
│  └──────────┘ └──────────┘ └────────────┘  │
└─────────────┬───────────────────────────────┘
              │
     ┌────────┴────────┐
     ▼                 ▼
┌─────────┐    ┌──────────────────┐
│   AWS   │    │  PYTHON RUNTIME  │
│Services │    │  ┌────────────┐  │
│         │    │  │  Temporal  │  │
│Secrets  │    │  │ Workflows  │  │
│DynamoDB │    │  └────────────┘  │
│S3       │    │  ┌────────────┐  │
│ECS      │    │  │ Activities │  │
│API GW   │    │  │(Tool Lib)  │  │
└─────────┘    │  └────────────┘  │
               └──────────────────┘
```

### Component Flow

**Phase 1: Analysis**
- User provides description
- Rust calls Anthropic API
- AI returns structured requirements (tools, credentials, steps, approvals)

**Phase 2: Credentials**
- Interactive prompts for each credential
- Validate against actual services
- Store in AWS Secrets Manager
- Cache ARNs

**Phase 3: Compilation**
- Create WorkflowSpec (internal DSL)
- Generate Python code from Tera templates
- Validate syntax
- Create worker, requirements.txt, Dockerfile

**Phase 4: Dry-Run**
- Generate mock data
- Create Python venv
- Inject mock services
- Execute workflow
- Collect trace and estimate costs

**Phase 5: Deployment**
- Package code to tar.gz
- Upload to S3
- Register ECS task definition
- Create ECS service
- Configure API Gateway webhook
- Save to DynamoDB

**Phase 6: Runtime**
- Webhook triggers workflow
- Python worker executes
- Activities call real services
- Approvals via Temporal signals
- Results saved to DynamoDB

---

## Technology Stack

### Rust Dependencies

```toml
[dependencies]
# CLI & TUI
clap = { version = "4.5", features = ["derive"] }
dialoguer = "0.11"
console = "0.15"
indicatif = "0.17"

# Async
tokio = { version = "1.36", features = ["full"] }
async-trait = "0.1"

# HTTP
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }

# AWS SDK
aws-config = "1.1"
aws-sdk-secretsmanager = "1.13"
aws-sdk-dynamodb = "1.13"
aws-sdk-s3 = "1.15"
aws-sdk-ecs = "1.13"
aws-sdk-apigateway = "1.13"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Templates
tera = "1.19"

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utilities
uuid = { version = "1.7", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
regex = "1.10"
dotenv = "0.15"
```

### Python Dependencies (Generated)

```txt
temporalio==1.5.0
anthropic==0.18.0
openai==1.12.0
httpx==0.26.0
boto3==1.34.0
pydantic==2.6.0
python-dotenv==1.0.0
```

### Infrastructure

- **AWS:** ECS Fargate, Secrets Manager, DynamoDB, S3, API Gateway, CloudWatch
- **Temporal:** Cloud or self-hosted
- **APIs:** Slack, Amadeus, Anthropic, OpenAI

---

## Project Structure

```
ai-workflow-generator/
├── Cargo.toml
├── .env.example
├── rust-toolchain.toml
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── app.rs                     # Clap definitions
│   │   ├── commands/
│   │   │   ├── create.rs
│   │   │   ├── deploy.rs
│   │   │   ├── list.rs
│   │   │   ├── test.rs
│   │   │   └── delete.rs
│   │   └── ui/
│   │       ├── prompts.rs             # Dialoguer prompts
│   │       ├── display.rs             # Output formatting
│   │       └── progress.rs            # Progress indicators
│   │
│   ├── analyzer/
│   │   ├── mod.rs
│   │   ├── requirements.rs            # Requirements extraction
│   │   ├── ai_client.rs               # Anthropic API client
│   │   └── tool_selector.rs           # Tool selection
│   │
│   ├── credentials/
│   │   ├── mod.rs
│   │   ├── manager.rs                 # CRUD operations
│   │   ├── interrogator.rs            # Interactive gathering
│   │   ├── validator.rs               # Validation
│   │   └── aws_secrets.rs             # AWS Secrets Manager
│   │
│   ├── compiler/
│   │   ├── mod.rs
│   │   ├── workflow_compiler.rs       # DSL → Python
│   │   ├── code_generator.rs          # Code generation
│   │   ├── validator.rs               # Validation
│   │   └── dsl/
│   │       ├── models.rs              # WorkflowSpec structures
│   │       ├── parser.rs
│   │       └── schema.rs
│   │
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── registry.rs                # Tool registry
│   │   ├── models.rs                  # Tool definitions
│   │   └── definitions/
│   │       ├── messaging.rs           # Slack, email
│   │       ├── ai.rs                  # AI agents
│   │       ├── travel.rs              # Flight APIs
│   │       └── utilities.rs
│   │
│   ├── testing/
│   │   ├── mod.rs
│   │   ├── dry_runner.rs              # Orchestrate dry-runs
│   │   ├── mock_generator.rs          # Generate mock data
│   │   ├── python_executor.rs         # Spawn Python
│   │   └── trace_collector.rs         # Collect traces
│   │
│   ├── deployment/
│   │   ├── mod.rs
│   │   ├── deployer.rs                # Orchestrator
│   │   ├── aws.rs                     # AWS deployment
│   │   ├── temporal_client.rs         # Temporal API
│   │   ├── packager.rs                # Package artifacts
│   │   └── webhook.rs                 # Webhook config
│   │
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── dynamodb.rs                # DynamoDB ops
│   │   └── models.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs                # Configuration
│   │
│   └── utils/
│       ├── mod.rs
│       ├── errors.rs                  # Error types
│       ├── logger.rs                  # Logging
│       └── validation.rs
│
├── python_runtime/                    # Python library
│   ├── requirements.txt
│   ├── activities/
│   │   ├── base.py
│   │   ├── messaging/slack.py
│   │   ├── ai/claude.py
│   │   ├── travel/amadeus.py
│   │   └── utils/credentials.py
│   ├── workflows/
│   │   ├── approval.py
│   │   └── base.py
│   └── worker/
│       └── main.py
│
├── templates/                         # Tera templates
│   ├── workflow.py.tera
│   ├── worker.py.tera
│   ├── requirements.txt.tera
│   └── Dockerfile.tera
│
├── tests/
│   ├── unit/
│   ├── integration/
│   └── fixtures/
│
├── infrastructure/
│   └── terraform/
│       ├── main.tf
│       ├── dynamodb.tf
│       ├── ecs.tf
│       └── secrets.tf
│
└── docs/
    └── examples/
```

---

## Phase 1: Foundation

**Duration:** 2 weeks  
**Goal:** Project setup, configuration, tool registry, credential management, CLI

### 1.1 Project Initialization

**Files to create:**
- `Cargo.toml` - Project manifest with all dependencies
- `.env.example` - Template for environment variables
- `rust-toolchain.toml` - Rust version specification

**Environment Variables:**
```bash
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=

TEMPORAL_HOST=namespace.tmprl.cloud:7233
TEMPORAL_NAMESPACE=namespace
TEMPORAL_CERT_PATH=
TEMPORAL_KEY_PATH=

ANTHROPIC_API_KEY=
OPENAI_API_KEY=

WORKFLOWS_TABLE=ai-workflows
EXECUTIONS_TABLE=ai-workflow-executions
CODE_BUCKET=ai-workflow-code

LOG_LEVEL=info
ENVIRONMENT=development
```

### 1.2 Configuration Module

**Module:** `src/config/settings.rs`

**Structures:**
```rust
pub struct Settings {
    pub aws: AwsConfig,
    pub temporal: TemporalConfig,
    pub ai: AiConfig,
    pub storage: StorageConfig,
    pub app: AppConfig,
}

pub struct AwsConfig {
    pub region: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

pub struct TemporalConfig {
    pub host: String,
    pub namespace: String,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
}

pub struct AiConfig {
    pub anthropic_api_key: String,
    pub openai_api_key: Option<String>,
    pub default_model: String,
}

pub struct StorageConfig {
    pub workflows_table: String,
    pub executions_table: String,
    pub code_bucket: String,
}

pub struct AppConfig {
    pub log_level: String,
    pub environment: String,
}
```

**Key Method:**
- `Settings::from_env()` - Load from environment with dotenv support

### 1.3 Error Handling

**Module:** `src/utils/errors.rs`

**Error Enum:**
```rust
#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Analysis failed: {0}")]
    AnalysisError(String),
    
    #[error("Credential validation failed: {0}")]
    CredentialError(String),
    
    #[error("Code generation failed: {0}")]
    CodeGenerationError(String),
    
    #[error("Deployment failed: {0}")]
    DeploymentError(String),
    
    #[error("AWS error: {0}")]
    AwsError(#[from] aws_sdk_secretsmanager::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid spec: {0}")]
    InvalidSpec(String),
    
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
}

pub type Result<T> = std::result::Result<T, WorkflowError>;
```

### 1.4 Tool Registry

**Module:** `src/tools/models.rs`

**Core Structures:**
```rust
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub parameters: Vec<ToolParameter>,
    pub credentials_required: Vec<String>,
    pub python_activity_path: String,
    pub estimated_duration_secs: u32,
    pub cost_per_call_usd: f64,
}

pub enum ToolCategory {
    Messaging,
    Ai,
    Travel,
    Data,
    Utility,
}

pub struct ToolParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub description: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

pub enum ParameterType {
    String, Integer, Float, Boolean, Object, Array,
}
```

**Module:** `src/tools/registry.rs`

**ToolRegistry Implementation:**
```rust
pub struct ToolRegistry {
    tools: HashMap<String, ToolDefinition>,
}

impl ToolRegistry {
    pub fn new() -> Self;
    fn register_default_tools(&mut self);
    pub fn get_tool(&self, name: &str) -> Option<&ToolDefinition>;
    pub fn list_tools(&self) -> Vec<&ToolDefinition>;
    pub fn list_by_category(&self, category: &ToolCategory) -> Vec<&ToolDefinition>;
    pub fn search_tools(&self, query: &str) -> Vec<&ToolDefinition>;
}
```

**Default Tools:**
1. `slack_send_message` - Send Slack messages
2. `run_claude_agent` - Execute Claude AI agent
3. `run_openai_agent` - Execute OpenAI agent
4. `search_flights` - Search flights (Amadeus)
5. `book_flight` - Book flight (Amadeus)
6. `http_request` - Generic HTTP request

### 1.5 CLI Structure

**Module:** `src/cli/app.rs`

**CLI Definition:**
```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        description: Option<String>,
        #[arg(long)] skip_dry_run: bool,
        #[arg(long)] auto_deploy: bool,
    },
    List {
        #[arg(long)] status: Option<String>,
    },
    Test {
        workflow: String,
        #[arg(long)] mock_data: Option<String>,
    },
    Deploy {
        workflow: String,
        #[arg(long)] force: bool,
    },
    Delete {
        workflow: String,
        #[arg(long)] yes: bool,
    },
    Show {
        workflow: String,
    },
}
```

**Module:** `src/main.rs`

**Main Entry Point:**
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::logger::init_logger();
    let cli = Cli::parse();
    let settings = Settings::from_env()?;
    
    match cli.command {
        Commands::Create { description, skip_dry_run, auto_deploy } => {
            cli::commands::create::execute(description, skip_dry_run, auto_deploy, &settings).await?;
        }
        // ... other commands
    }
    
    Ok(())
}
```

### 1.6 Command Handlers

**Module:** `src/cli/commands/create.rs`

**Signature:**
```rust
pub async fn execute(
    description: Option<String>,
    skip_dry_run: bool,
    auto_deploy: bool,
    settings: &Settings,
) -> Result<()>
```

**Flow:**
1. Get description (prompt if not provided)
2. Analyze requirements with AI
3. Gather credentials interactively
4. Compile workflow to code
5. If not skip_dry_run: run dry-run
6. If auto_deploy: deploy immediately
7. Otherwise: prompt user for next action

**Similar structure for:** `list.rs`, `test.rs`, `deploy.rs`, `delete.rs`, `show.rs`

### 1.7 UI Components

**Module:** `src/cli/ui/prompts.rs`

**Functions using dialoguer:**
```rust
pub fn prompt_text(prompt: &str, default: Option<&str>) -> Result<String>;
pub fn prompt_password(prompt: &str) -> Result<String>;
pub fn prompt_confirm(prompt: &str, default: bool) -> Result<bool>;
pub fn prompt_select<T>(prompt: &str, items: &[T]) -> Result<usize>;
```

**Module:** `src/cli/ui/display.rs`

**Functions using console/indicatif:**
```rust
pub fn display_workflow_list(workflows: &[WorkflowRecord]);
pub fn display_dry_run_results(results: &DryRunResult);
pub fn display_deployment_info(deployment: &Deployment);
pub fn display_error(error: &WorkflowError);
pub fn display_success(message: &str);
```

---

## Phase 2: Core Logic

**Duration:** 3 weeks  
**Goal:** AI analysis, DSL compilation, Python code generation

### 2.1 AI Client

**Module:** `src/analyzer/ai_client.rs`

**Structure:**
```rust
pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self;
    pub async fn analyze(&self, prompt: &str) -> Result<String>;
}
```

**Implementation:**
- Endpoint: `POST https://api.anthropic.com/v1/messages`
- Headers: `x-api-key`, `anthropic-version: 2023-06-01`
- Model: `claude-sonnet-4.5-20250929`
- Return extracted text from response

### 2.2 Requirements Analyzer

**Module:** `src/analyzer/requirements.rs`

**Structures:**
```rust
pub struct WorkflowRequirements {
    pub tools_needed: Vec<String>,
    pub credentials_needed: Vec<String>,
    pub approval_points: Vec<ApprovalPoint>,
    pub trigger_type: TriggerType,
    pub steps: Vec<RequiredStep>,
}

pub struct ApprovalPoint {
    pub step_id: String,
    pub reason: String,
    pub timeout_minutes: u32,
}

pub enum TriggerType {
    Webhook, Schedule, Manual,
}

pub struct RequiredStep {
    pub step_type: StepType,
    pub description: String,
    pub tool_name: Option<String>,
}

pub enum StepType {
    Agent, ApiCall, Transform, Approval, Conditional,
}
```

**RequirementsAnalyzer:**
```rust
pub struct RequirementsAnalyzer {
    ai_client: AnthropicClient,
    tool_registry: ToolRegistry,
}

impl RequirementsAnalyzer {
    pub fn new(api_key: String) -> Self;
    pub async fn analyze(&self, description: &str) -> Result<WorkflowRequirements>;
    fn build_analysis_prompt(&self, description: &str) -> String;
}
```

**Analysis Prompt Template:**
```
Analyze this workflow description and identify requirements.

Workflow: {description}

Available Tools:
{list of tools from registry}

Respond with JSON:
{
    "tools_needed": ["tool1", "tool2"],
    "credentials_needed": ["service/key"],
    "approval_points": [{
        "step_id": "id",
        "reason": "why approval needed",
        "timeout_minutes": 30
    }],
    "trigger_type": "webhook",
    "steps": [{
        "step_type": "agent",
        "description": "what this does",
        "tool_name": "tool_to_use"
    }]
}

Guidelines:
- Only use tools from available list
- Identify approval points (financial, data modification)
- Infer trigger type
- Break into logical steps

Respond ONLY with valid JSON.
```

### 2.3 DSL Models

**Module:** `src/compiler/dsl/models.rs`

**Core Structures:**
```rust
pub struct WorkflowSpec {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub trigger: TriggerSpec,
    pub steps: Vec<StepSpec>,
    pub credentials: HashMap<String, String>,  // key -> Secret ARN
}

pub struct TriggerSpec {
    pub trigger_type: String,
    pub config: HashMap<String, serde_json::Value>,
}

pub struct StepSpec {
    pub id: String,
    pub step_type: StepType,
    pub description: String,
    pub config: StepConfig,
    pub timeout_seconds: u32,
    pub retry_policy: Option<RetryPolicy>,
}

pub enum StepType {
    Agent { config: AgentConfig },
    Activity { config: ActivityConfig },
    Approval { config: ApprovalConfig },
    Conditional { config: ConditionalConfig },
}

pub struct AgentConfig {
    pub system_prompt: String,
    pub model: String,
    pub tools: Vec<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

pub struct ActivityConfig {
    pub activity_name: String,
    pub python_path: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

pub struct ApprovalConfig {
    pub approver: String,
    pub reason: String,
    pub timeout_minutes: u32,
    pub notification_channel: String,
}

pub struct RetryPolicy {
    pub maximum_attempts: u32,
    pub initial_interval_seconds: u32,
    pub backoff_coefficient: f32,
    pub maximum_interval_seconds: u32,
}
```

### 2.4 Workflow Compiler

**Module:** `src/compiler/workflow_compiler.rs`

**Structure:**
```rust
pub struct WorkflowCompiler {
    tool_registry: ToolRegistry,
}

impl WorkflowCompiler {
    pub fn new(tool_registry: ToolRegistry) -> Self;
    pub fn compile(&self, requirements: WorkflowRequirements, credentials: HashMap<String, String>) -> Result<WorkflowSpec>;
}
```

**Compilation Logic:**
- Generate unique workflow ID
- Create workflow name from description
- Map requirements to StepSpec structures
- For each step:
  - Determine type (Agent, Activity, Approval)
  - Create appropriate config
  - Set timeouts and retry policies
  - Map inputs/outputs
- Create TriggerSpec
- Build credential mapping
- Return WorkflowSpec

**Default Values:**
- Agent timeout: 120 seconds
- Activity timeout: 60 seconds
- Approval timeout: 30 minutes
- Retry: 3 attempts, exponential backoff

### 2.5 Code Generator

**Module:** `src/compiler/code_generator.rs`

**Structure:**
```rust
pub struct PythonCodeGenerator {
    tera: Tera,
}

impl PythonCodeGenerator {
    pub fn new() -> Result<Self>;
    pub fn generate_workflow(&self, spec: &WorkflowSpec) -> Result<String>;
    pub fn generate_worker(&self, spec: &WorkflowSpec) -> Result<String>;
    pub fn generate_requirements(&self, spec: &WorkflowSpec) -> Result<String>;
    pub fn generate_dockerfile(&self, spec: &WorkflowSpec) -> Result<String>;
    pub fn validate_python_syntax(&self, code: &str) -> Result<bool>;
}
```

**Helper Methods:**
- `to_class_name(name)` - Convert to PascalCase
- `process_steps(steps)` - Convert to template format
- `determine_imports(steps)` - Extract required imports
- `format_step_config(step)` - Format for template

### 2.6 Tera Templates

**File:** `templates/workflow.py.tera`

```python
"""
AUTO-GENERATED WORKFLOW
Generated: {{ timestamp }}
Description: {{ description }}
"""

from temporalio import workflow
from temporalio.common import RetryPolicy
from datetime import timedelta
from typing import Dict, Any

{% for import in imports %}
{{ import }}
{% endfor %}

@workflow.defn
class {{ class_name }}:
    """{{ description }}"""
    
    @workflow.run
    async def run(self, trigger_data: Dict[str, Any]) -> Dict[str, Any]:
        {% for step in steps %}
        # Step {{ loop.index }}: {{ step.description }}
        {% if step.step_type == "agent" %}
        {{ step.var_name }} = await workflow.execute_activity(
            run_claude_agent,
            {
                "system_prompt": """{{ step.config.system_prompt }}""",
                "user_message": {{ step.config.user_message }},
                "tools": {{ step.config.tools }},
                "model": "{{ step.config.model }}"
            },
            start_to_close_timeout=timedelta(seconds={{ step.timeout }}),
            retry_policy=RetryPolicy(maximum_attempts={{ step.retry.max_attempts }})
        )
        {% elif step.step_type == "activity" %}
        {{ step.var_name }} = await workflow.execute_activity(
            {{ step.activity_function }},
            {{ step.parameters }},
            start_to_close_timeout=timedelta(seconds={{ step.timeout }})
        )
        {% elif step.step_type == "approval" %}
        {{ step.var_name }} = await workflow.execute_child_workflow(
            HumanApprovalWorkflow,
            {{ step.config }},
            id=f"approval-{workflow.info().workflow_id}"
        )
        if not {{ step.var_name }}["approved"]:
            return {"success": False, "reason": "approval_rejected"}
        {% endif %}
        {% endfor %}
        
        return {"success": True, {% for output in outputs %}"{{ output.name }}": {{ output.value }},{% endfor %}}
```

**File:** `templates/worker.py.tera`

```python
"""Worker for {{ workflow_name }}"""
import asyncio
import os
from temporalio.client import Client
from temporalio.worker import Worker
from workflows.{{ workflow_module }} import {{ workflow_class }}
{% for activity in activity_imports %}{{ activity }}{% endfor %}

async def main():
    client = await Client.connect(
        os.environ["TEMPORAL_HOST"],
        namespace=os.environ["TEMPORAL_NAMESPACE"],
    )
    
    worker = Worker(
        client,
        task_queue="{{ task_queue }}",
        workflows=[{{ workflow_class }}],
        activities=[{% for activity in activities %}{{ activity }},{% endfor %}],
    )
    
    await worker.run()

if __name__ == "__main__":
    asyncio.run(main())
```

---

## Phase 3: Testing Infrastructure

**Duration:** 1 week  
**Goal:** Dry-run orchestration, mock data generation

### 3.1 Mock Data Generator

**Module:** `src/testing/mock_generator.rs`

**Structures:**
```rust
pub struct MockDataSet {
    pub trigger_data: serde_json::Value,
    pub service_responses: HashMap<String, Vec<serde_json::Value>>,
    pub ai_responses: HashMap<String, String>,
}

pub struct MockDataGenerator {
    // fields
}

impl MockDataGenerator {
    pub fn new() -> Self;
    pub fn generate_for_workflow(&self, spec: &WorkflowSpec) -> Result<MockDataSet>;
    fn generate_trigger_data(&self, trigger: &TriggerSpec) -> Result<serde_json::Value>;
    fn generate_service_response(&self, tool_name: &str) -> Result<serde_json::Value>;
    fn generate_ai_response(&self, system_prompt: &str, description: &str) -> Result<String>;
}
```

**Mock Data Examples:**
- Slack webhook: `{"channel": "#travel", "user": "U123", "text": "...", "ts": "..."}`
- Flight search: `[{"carrier": "AA", "number": "123", "price": {"total": "450", "currency": "USD"}, ...}]`
- AI response: `{"origin": "JFK", "destination": "LAX", "date": "2025-12-15"}`

### 3.2 Python Executor

**Module:** `src/testing/python_executor.rs`

**Structures:**
```rust
pub struct ExecutionResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration: Duration,
}

pub struct PythonExecutor {
    python_path: PathBuf,
    venv_path: Option<PathBuf>,
}

impl PythonExecutor {
    pub fn new(python_path: PathBuf) -> Self;
    pub async fn create_venv(&self, target_dir: &Path) -> Result<PathBuf>;
    pub async fn install_dependencies(&self, venv_path: &Path, requirements: &[String]) -> Result<()>;
    pub fn validate_syntax(&self, code_file: &Path) -> Result<bool>;
    pub async fn execute_script(&self, script_path: &Path, args: &[String], env_vars: HashMap<String, String>) -> Result<ExecutionResult>;
}
```

**Implementation:**
- Use `tokio::process::Command`
- Set timeouts
- Capture stdout/stderr
- Stream output for progress

### 3.3 Dry-Run Orchestrator

**Module:** `src/testing/dry_runner.rs`

**Structures:**
```rust
pub struct DryRunConfig {
    pub workflow_spec: WorkflowSpec,
    pub generated_code_path: PathBuf,
    pub timeout_seconds: u32,
}

pub struct DryRunResult {
    pub success: bool,
    pub execution_trace: Vec<TraceEvent>,
    pub outputs: serde_json::Value,
    pub duration: Duration,
    pub cost_estimate: CostEstimate,
    pub errors: Vec<String>,
}

pub struct TraceEvent {
    pub timestamp: DateTime<Utc>,
    pub step_id: String,
    pub event_type: TraceEventType,
    pub details: serde_json::Value,
}

pub enum TraceEventType {
    StepStarted, StepCompleted, StepFailed,
    ActivityExecuted, ApprovalRequested,
}

pub struct CostEstimate {
    pub ai_api_cost: f64,
    pub external_api_cost: f64,
    pub total_cost: f64,
}

pub struct DryRunner {
    python_executor: PythonExecutor,
    mock_generator: MockDataGenerator,
}

impl DryRunner {
    pub fn new() -> Self;
    pub async fn run_dry_run(&self, config: DryRunConfig) -> Result<DryRunResult>;
    async fn create_mock_layer(&self, mock_data: &MockDataSet, target_dir: &Path) -> Result<()>;
    fn collect_trace(&self, log_output: &str) -> Vec<TraceEvent>;
    fn estimate_costs(&self, spec: &WorkflowSpec, trace: &[TraceEvent]) -> CostEstimate;
}
```

**Dry-Run Flow:**
1. Create temp directory
2. Generate mock data
3. Copy workflow code
4. Create mock service layer (Python)
5. Create venv, install deps
6. Execute workflow with mocks
7. Collect trace from logs
8. Calculate cost estimates
9. Clean up
10. Return results

### 3.4 Trace Collector

**Module:** `src/testing/trace_collector.rs`

**Methods:**
```rust
impl TraceCollector {
    pub fn new() -> Self;
    pub fn add_event(&mut self, event: TraceEvent);
    pub fn format_trace(&self) -> String;
    pub fn generate_summary(&self) -> TraceSummary;
    pub fn export_json(&self) -> Result<String>;
}
```

**Display Format:**
```
🧪 Dry-Run Execution Trace
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[10:30:00] ▶ Step 1: parse_request
[10:30:02] ✓ Step 1 completed (2.3s)

[10:30:02] ▶ Step 2: search_flights
[10:30:03] ✓ Step 2 completed (1.3s)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary:
  Total: 5 steps
  Success: 5
  Duration: 12.4s
  
Cost Estimate:
  AI: $0.05
  APIs: $0.01
  Total: $0.06 per execution
```

---

## Phase 4: Deployment & UI

**Duration:** 2 weeks  
**Goal:** AWS deployment, webhooks, approval UI

### 4.1 Credential Management

**Module:** `src/credentials/aws_secrets.rs`

**Structures:**
```rust
pub struct StoredCredential {
    pub secret_id: String,
    pub secret_arn: String,
    pub created_at: DateTime<Utc>,
}

pub struct AwsSecretsClient {
    client: aws_sdk_secretsmanager::Client,
    region: String,
}

impl AwsSecretsClient {
    pub fn new(region: String) -> Result<Self>;
    pub async fn store_credential(&self, key: &str, value: &str, description: &str) -> Result<StoredCredential>;
    pub async fn retrieve_credential(&self, key: &str) -> Result<String>;
    pub async fn delete_credential(&self, key: &str) -> Result<()>;
    pub async fn list_credentials(&self, prefix: Option<&str>) -> Result<Vec<StoredCredential>>;
}
```

**Module:** `src/credentials/validator.rs`

**Trait:**
```rust
#[async_trait]
pub trait CredentialValidator {
    async fn validate(&self, credential: &str) -> Result<bool>;
}

pub struct SlackValidator;
pub struct AmadeusValidator;
pub struct AnthropicValidator;
```

**Module:** `src/credentials/interrogator.rs`

**Structure:**
```rust
pub struct CredentialInterrogator {
    aws_client: AwsSecretsClient,
    validators: HashMap<String, Box<dyn CredentialValidator>>,
}

impl CredentialInterrogator {
    pub fn new(aws_client: AwsSecretsClient) -> Self;
    pub async fn gather_credentials(&self, requirements: &[String]) -> Result<HashMap<String, String>>;
    fn prompt_for_credential(&self, prompt: &CredentialPrompt) -> Result<String>;
    async fn validate_and_store(&self, key: &str, value: &str, validator: Option<&dyn CredentialValidator>) -> Result<String>;
}
```

### 4.2 Code Packager

**Module:** `src/deployment/packager.rs`

**Structures:**
```rust
pub struct PackageArtifact {
    pub workflow_id: Uuid,
    pub archive_path: PathBuf,
    pub size_bytes: u64,
    pub checksum: String,
    pub manifest: PackageManifest,
}

pub struct CodePackager {
    work_dir: PathBuf,
}

impl CodePackager {
    pub fn new() -> Self;
    pub fn package_workflow(&self, spec: &WorkflowSpec, code_path: &Path) -> Result<PackageArtifact>;
    fn generate_dockerfile(&self, spec: &WorkflowSpec) -> Result<String>;
    pub async fn upload_to_s3(&self, artifact: &PackageArtifact, bucket: &str) -> Result<String>;
}
```

**Package Contents:**
- Generated workflow code
- python_runtime/ library
- requirements.txt
- Dockerfile
- manifest.json

### 4.3 AWS Deployer

**Module:** `src/deployment/aws.rs`

**Structures:**
```rust
pub struct DeploymentSettings {
    pub cluster_name: String,
    pub task_definition_family: String,
    pub execution_role_arn: String,
    pub task_role_arn: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
}

pub struct Deployment {
    pub workflow_id: Uuid,
    pub task_definition_arn: String,
    pub service_arn: String,
    pub webhook_url: Option<String>,
    pub status: DeploymentStatus,
}

pub struct AwsDeployer {
    ecs_client: aws_sdk_ecs::Client,
    s3_client: aws_sdk_s3::Client,
    apigateway_client: aws_sdk_apigateway::Client,
    settings: DeploymentSettings,
}

impl AwsDeployer {
    pub fn new(settings: DeploymentSettings) -> Result<Self>;
    pub async fn deploy(&self, spec: &WorkflowSpec, artifact: &PackageArtifact) -> Result<Deployment>;
    async fn create_task_definition(&self, spec: &WorkflowSpec, artifact: &PackageArtifact) -> Result<String>;
    async fn create_ecs_service(&self, task_def_arn: &str, spec: &WorkflowSpec) -> Result<String>;
    async fn configure_webhook(&self, spec: &WorkflowSpec, service_arn: &str) -> Result<String>;
    pub async fn delete_deployment(&self, workflow_id: &Uuid) -> Result<()>;
}
```

**Deployment Steps:**
1. Upload package to S3
2. Register ECS task definition
3. Create ECS Fargate service
4. Configure API Gateway (if webhook)
5. Wait for service stability
6. Return deployment info

### 4.4 DynamoDB Storage

**Module:** `src/storage/dynamodb.rs`

**Structures:**
```rust
pub struct WorkflowRecord {
    pub workflow_id: Uuid,
    pub name: String,
    pub description: String,
    pub spec: WorkflowSpec,
    pub status: WorkflowStatus,
    pub deployment: Option<Deployment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum WorkflowStatus {
    Draft, Deployed, Active, Stopped, Failed,
}

pub struct DynamoDbStorage {
    client: aws_sdk_dynamodb::Client,
    workflows_table: String,
    executions_table: String,
}

impl DynamoDbStorage {
    pub fn new(workflows_table: String, executions_table: String) -> Result<Self>;
    pub async fn save_workflow(&self, record: &WorkflowRecord) -> Result<()>;
    pub async fn get_workflow(&self, workflow_id: &Uuid) -> Result<Option<WorkflowRecord>>;
    pub async fn list_workflows(&self, status: Option<WorkflowStatus>) -> Result<Vec<WorkflowRecord>>;
    pub async fn update_workflow_status(&self, workflow_id: &Uuid, status: WorkflowStatus) -> Result<()>;
    pub async fn delete_workflow(&self, workflow_id: &Uuid) -> Result<()>;
}
```

**DynamoDB Tables:**

**ai-workflows:**
- PK: workflow_id
- Attributes: name, description, spec, status, deployment, created_at, updated_at
- GSI: status-index

**ai-workflow-executions:**
- PK: execution_id
- SK: timestamp
- Attributes: workflow_id, run_id, trigger_data, status, result, duration_ms, cost
- GSI: workflow_id-index

### 4.5 Approval UI

**Module:** `src/api/server.rs`

Basic HTTP server using axum or actix-web.

**Routes:**
- `GET /approve/:workflow_id/:approval_id` - Display approval page (HTML)
- `POST /approve/:workflow_id/:approval_id/approve` - Approve action
- `POST /approve/:workflow_id/:approval_id/reject` - Reject action
- `GET /health` - Health check

**HTML Template:**
```html
<!DOCTYPE html>
<html>
<head><title>Approval Request</title></head>
<body>
    <h1>🔔 Approval Request</h1>
    <div class="details">
        <h3>{{ action }}</h3>
        <p><strong>Details:</strong></p>
        <pre>{{ details }}</pre>
    </div>
    <form method="POST" action="/approve/{{workflow_id}}/{{approval_id}}/approve">
        <button type="submit">✓ Approve</button>
    </form>
    <form method="POST" action="/approve/{{workflow_id}}/{{approval_id}}/reject">
        <button type="submit">✗ Reject</button>
    </form>
</body>
</html>
```

---

## Data Models & Schemas

### Python Runtime

**File:** `python_runtime/activities/base.py`

```python
from dataclasses import dataclass
from typing import Any, Dict
from abc import ABC, abstractmethod

@dataclass
class ActivityContext:
    workflow_id: str
    run_id: str
    activity_id: str
    attempt: int

class BaseActivity(ABC):
    @abstractmethod
    async def execute(self, ctx: ActivityContext, **kwargs) -> Dict[str, Any]:
        pass
```

**File:** `python_runtime/workflows/approval.py`

```python
from temporalio import workflow
from datetime import timedelta

@workflow.defn
class HumanApprovalWorkflow:
    def __init__(self):
        self._approved = False
    
    @workflow.run
    async def run(self, request: dict) -> dict:
        # Send approval notification
        await workflow.execute_activity(
            send_approval_notification,
            request,
            start_to_close_timeout=timedelta(seconds=30)
        )
        
        # Wait for signal
        await workflow.wait_condition(
            lambda: self._approved,
            timeout=timedelta(minutes=request["timeout_minutes"])
        )
        
        return {"approved": self._approved}
    
    @workflow.signal
    async def approve(self):
        self._approved = True
    
    @workflow.signal
    async def reject(self):
        self._approved = False
```

---

## Security Considerations

### Credential Security
- Store all credentials in AWS Secrets Manager with KMS encryption
- Never log credential values
- Use IAM roles for service-to-service auth
- Audit all access via CloudWatch

### Code Generation
- Sanitize all user inputs
- No arbitrary code execution
- Templates are controlled
- Generated code runs in isolated containers
- Pin all dependencies

### AWS Security
- ECS tasks in private subnets
- VPC endpoints for AWS services
- Security groups restrict traffic
- Encrypt data at rest (DynamoDB, S3)
- TLS for data in transit

### Webhook Security
- Verify signatures (Slack, GitHub)
- API keys for custom webhooks
- Rate limiting
- Validate payloads

---

## Testing Strategy

### Unit Tests (`tests/unit/`)
- Test all public functions
- Mock external dependencies
- Use mockito for HTTP
- Fixtures for sample data

### Integration Tests (`tests/integration/`)
- Test module interactions
- Use LocalStack for AWS
- Test Python execution
- End-to-end code generation

### E2E Tests (`tests/e2e/`)
- Complete user journeys
- Real AWS (test account)
- Actual workflow execution
- Requires: AWS, Temporal, API keys

**Running Tests:**
```bash
cargo test                    # All tests
cargo test --lib              # Unit only
cargo test --test '*'         # Integration
RUST_LOG=debug cargo test    # With logging
E2E_TESTS=1 cargo test e2e   # E2E (requires AWS)
```

---

## Deployment Guide

### Prerequisites
1. AWS account with permissions
2. Temporal Cloud account (or self-hosted)
3. API keys (Anthropic, Slack, Amadeus)
4. Tools: Rust 1.75+, Python 3.11+, Terraform

### Infrastructure Setup

```bash
cd infrastructure/terraform
terraform init
terraform apply
```

Creates: DynamoDB tables, S3 bucket, ECS cluster, VPC, IAM roles, CloudWatch logs.

### Application Setup

```bash
# Configure
cp .env.example .env
vi .env  # Add credentials

# Build
cargo build --release

# Install
cargo install --path .
```

### Usage

```bash
# Create workflow
ai-workflow create

# Follow prompts:
# 1. Describe workflow
# 2. Enter credentials
# 3. Review dry-run
# 4. Deploy

# List workflows
ai-workflow list

# Deploy specific workflow
ai-workflow deploy <workflow_id>

# Test workflow
ai-workflow test <workflow_id>

# Delete workflow
ai-workflow delete <workflow_id>
```

### Monitoring

- **CloudWatch:** ECS logs, API Gateway logs
- **Temporal UI:** Workflow executions, history
- **DynamoDB:** Workflow metadata, execution records

### Troubleshooting

**Credential validation fails:**
- Check API keys are correct
- Verify network connectivity
- Check AWS Secrets Manager permissions

**Deployment fails:**
- Verify IAM role permissions
- Check VPC and subnet configuration
- Review ECS task logs in CloudWatch

**Workflow execution fails:**
- Check Temporal connection from ECS
- Verify secrets are accessible
- Review activity logs
- Check Python dependencies

**Dry-run fails:**
- Verify Python environment
- Check mock data is valid
- Review generated code syntax

---

## Appendices

### Appendix A: Example Generated Workflow

```python
"""
AUTO-GENERATED WORKFLOW
Generated: 2025-11-02T15:30:00Z
Description: Slack travel booking with approval
"""

from temporalio import workflow
from temporalio.common import RetryPolicy
from datetime import timedelta
from typing import Dict, Any

from activities.ai.claude import run_claude_agent
from activities.travel.amadeus import search_flights, book_flight
from activities.messaging.slack import send_message
from workflows.approval import HumanApprovalWorkflow

@workflow.defn
class SlackTravelBookingWorkflow:
    """Travel booking workflow triggered by Slack"""
    
    @workflow.run
    async def run(self, trigger_data: Dict[str, Any]) -> Dict[str, Any]:
        # Step 1: Parse travel request
        parsed = await workflow.execute_activity(
            run_claude_agent,
            {
                "system_prompt": "Extract origin, destination, date from message. Return JSON.",
                "user_message": trigger_data["text"],
                "tools": [],
                "model": "claude-sonnet-4.5"
            },
            start_to_close_timeout=timedelta(seconds=120),
            retry_policy=RetryPolicy(maximum_attempts=3)
        )
        
        # Step 2: Search flights
        flights = await workflow.execute_activity(
            search_flights,
            {
                "origin": parsed["origin"],
                "destination": parsed["destination"],
                "date": parsed["date"]
            },
            start_to_close_timeout=timedelta(seconds=60)
        )
        
        if not flights:
            await workflow.execute_activity(
                send_message,
                {
                    "channel": trigger_data["channel"],
                    "text": "No flights found",
                    "thread_ts": trigger_data["ts"]
                },
                start_to_close_timeout=timedelta(seconds=10)
            )
            return {"success": False, "reason": "no_flights"}
        
        best = min(flights, key=lambda f: float(f["price"]["total"]))
        
        # Step 3: Request approval
        approval = await workflow.execute_child_workflow(
            HumanApprovalWorkflow,
            {
                "action": "book_flight",
                "details": {
                    "flight": f"{best['carrier']} {best['number']}",
                    "price": f"${best['price']['total']}"
                },
                "approver": trigger_data["user"],
                "timeout_minutes": 30
            },
            id=f"approval-{workflow.info().workflow_id}"
        )
        
        if not approval["approved"]:
            return {"success": False, "reason": "not_approved"}
        
        # Step 4: Book flight
        booking = await workflow.execute_activity(
            book_flight,
            {"flight_offer": best, "passenger_name": "John Doe"},
            start_to_close_timeout=timedelta(seconds=120)
        )
        
        # Step 5: Send confirmation
        await workflow.execute_activity(
            send_message,
            {
                "channel": trigger_data["channel"],
                "text": f"✅ Booked! Confirmation: {booking['confirmation_number']}",
                "thread_ts": trigger_data["ts"]
            },
            start_to_close_timeout=timedelta(seconds=10)
        )
        
        return {"success": True, "confirmation": booking["confirmation_number"]}
```

### Appendix B: Terraform Minimal Configuration

```hcl
# infrastructure/terraform/main.tf

provider "aws" {
  region = var.aws_region
}

# DynamoDB Tables
resource "aws_dynamodb_table" "workflows" {
  name           = "ai-workflows"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "workflow_id"
  
  attribute {
    name = "workflow_id"
    type = "S"
  }
  
  attribute {
    name = "status"
    type = "S"
  }
  
  global_secondary_index {
    name            = "status-index"
    hash_key        = "status"
    projection_type = "ALL"
  }
}

resource "aws_dynamodb_table" "executions" {
  name           = "ai-workflow-executions"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "execution_id"
  range_key      = "timestamp"
  
  attribute {
    name = "execution_id"
    type = "S"
  }
  
  attribute {
    name = "timestamp"
    type = "S"
  }
}

# S3 Bucket
resource "aws_s3_bucket" "code" {
  bucket = "ai-workflow-code-${var.account_id}"
}

resource "aws_s3_bucket_versioning" "code" {
  bucket = aws_s3_bucket.code.id
  versioning_configuration {
    status = "Enabled"
  }
}

# ECS Cluster
resource "aws_ecs_cluster" "workers" {
  name = "ai-workflow-workers"
}

# VPC (use existing or create new)
# IAM Roles
# Security Groups
# ... (see full Terraform examples in docs)
```

### Appendix C: Development Roadmap

**Phase 1-4 (Weeks 1-8): POC** ✅
- Core functionality
- 5-10 curated tools
- Basic approval
- AWS deployment
- CLI interface

**Post-POC Enhancements:**

**Phase 5: Tool Extensibility (Weeks 9-10)**
- User-defined tool registration
- Tool builder interface
- Community tools repository

**Phase 6: Advanced Features (Weeks 11-14)**
- Visual workflow editor
- Conditional branching UI
- Parallel execution
- Advanced error recovery
- Loop/iteration support

**Phase 7: Web UI (Weeks 15-18)**
- Web-based workflow creation
- Monitoring dashboard
- Execution history viewer
- Cost analytics

**Phase 8: Enterprise (Weeks 19-22)**
- Multi-tenancy
- RBAC and permissions
- Audit logging
- Compliance features
- SLA monitoring

---

## Conclusion

This specification provides a complete blueprint for implementing the AI Workflow Generator POC. The coding agent should:

1. **Build sequentially** - Complete Phase 1 before Phase 2, etc.
2. **Test thoroughly** - Write tests for each module
3. **Follow the structure** - Use the exact file/module organization
4. **Use the interfaces** - Implement all specified traits and methods
5. **Generate Python, don't embed** - All workflow code via templates
6. **Secure by default** - Credentials in Secrets Manager, IAM roles

**Key Success Metrics:**
- < 10 minutes from description to production
- 95%+ dry-run success rate
- Zero manual code writing
- Secure credential management

The system separates concerns cleanly: Rust handles infrastructure and code generation, Python handles runtime execution. This enables rapid development while maintaining type safety and reliability.

**Ready for implementation!**