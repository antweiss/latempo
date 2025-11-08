# AI Workflow Generator - Implementation Plan

## Overview

This document breaks down the implementation of the AI Workflow Generator into actionable phases and tasks. Each phase builds upon the previous one, ensuring a solid foundation before adding complexity.

**Total Estimated Timeline:** 6-8 weeks
**Current Status:** Not Started

---

## Phase 1: Foundation (Weeks 1-2)

**Goal:** Establish project structure, configuration, tool registry, and basic CLI

### Tasks

#### 1.1 Project Initialization ✓
- [x] Create directory structure
- [ ] Initialize Cargo.toml with dependencies
- [ ] Create rust-toolchain.toml (Rust 1.75+)
- [ ] Create .env.example
- [ ] Create .gitignore
- [ ] Initialize documentation structure

**Files to Create:**
```
├── Cargo.toml
├── rust-toolchain.toml
├── .env.example
├── .gitignore
├── README.md
└── docs/
    ├── SYSTEM_DESIGN.md ✓
    ├── IMPLEMENTATION_PLAN.md (this file)
    └── examples/
```

**Acceptance Criteria:**
- Project compiles with `cargo build`
- All dependencies resolve correctly
- Environment variables documented

---

#### 1.2 Configuration System
- [ ] Implement `src/config/mod.rs`
- [ ] Implement `src/config/settings.rs`
- [ ] Add environment variable loading with dotenv
- [ ] Add configuration validation
- [ ] Write unit tests for configuration

**Structures to Implement:**
```rust
pub struct Settings {
    pub aws: AwsConfig,
    pub temporal: TemporalConfig,
    pub ai: AiConfig,
    pub storage: StorageConfig,
    pub app: AppConfig,
}
```

**Acceptance Criteria:**
- Settings load from environment variables
- Validation catches missing required config
- Defaults work for optional values
- Tests cover all config scenarios

---

#### 1.3 Error Handling Framework
- [ ] Implement `src/utils/mod.rs`
- [ ] Implement `src/utils/errors.rs`
- [ ] Define WorkflowError enum with thiserror
- [ ] Define Result<T> type alias
- [ ] Implement error conversions (From traits)
- [ ] Add context helpers

**Error Types:**
```rust
pub enum WorkflowError {
    AnalysisError(String),
    CredentialError(String),
    CodeGenerationError(String),
    DeploymentError(String),
    AwsError(...),
    HttpError(...),
    IoError(...),
    InvalidSpec(String),
    ToolNotFound(String),
}
```

**Acceptance Criteria:**
- All error types defined
- Error messages are clear and actionable
- Error context preserved through layers
- Tests verify error conversions

---

#### 1.4 Logging System
- [ ] Implement `src/utils/logger.rs`
- [ ] Configure tracing-subscriber
- [ ] Add structured logging helpers
- [ ] Configure log levels from environment
- [ ] Add logging macros for common patterns

**Acceptance Criteria:**
- Logs output in structured format
- Log levels configurable via env var
- Sensitive data not logged
- Consistent logging across modules

---

#### 1.5 Tool Registry
- [ ] Implement `src/tools/mod.rs`
- [ ] Implement `src/tools/models.rs` (ToolDefinition, etc.)
- [ ] Implement `src/tools/registry.rs`
- [ ] Define default tools (6 initial tools)
- [ ] Implement `src/tools/definitions/messaging.rs`
- [ ] Implement `src/tools/definitions/ai.rs`
- [ ] Implement `src/tools/definitions/travel.rs`
- [ ] Implement `src/tools/definitions/utilities.rs`
- [ ] Write unit tests for registry operations

**Default Tools to Define:**
1. `slack_send_message` - Send Slack messages
2. `run_claude_agent` - Execute Claude AI agent
3. `run_openai_agent` - Execute OpenAI agent
4. `search_flights` - Search flights (Amadeus)
5. `book_flight` - Book flight (Amadeus)
6. `http_request` - Generic HTTP request

**Acceptance Criteria:**
- All 6 tools registered with complete metadata
- Registry supports get, list, search operations
- Tool parameters properly typed
- Credential requirements specified
- Tests verify registry operations

---

#### 1.6 CLI Structure
- [ ] Implement `src/cli/mod.rs`
- [ ] Implement `src/cli/app.rs` (Clap definitions)
- [ ] Implement `src/cli/ui/mod.rs`
- [ ] Implement `src/cli/ui/prompts.rs` (dialoguer)
- [ ] Implement `src/cli/ui/display.rs` (console/indicatif)
- [ ] Implement `src/cli/ui/progress.rs`
- [ ] Create command stubs in `src/cli/commands/`

**Commands to Define:**
```bash
ai-workflow create [DESCRIPTION] [--skip-dry-run] [--auto-deploy]
ai-workflow list [--status STATUS]
ai-workflow test <WORKFLOW_ID> [--mock-data PATH]
ai-workflow deploy <WORKFLOW_ID> [--force]
ai-workflow delete <WORKFLOW_ID> [--yes]
ai-workflow show <WORKFLOW_ID>
```

**Acceptance Criteria:**
- All commands parse correctly
- Help text is clear and complete
- Interactive prompts work with dialoguer
- Progress indicators display properly
- Tests verify command parsing

---

#### 1.7 Basic Command Handlers (Stubs)
- [ ] Implement `src/cli/commands/mod.rs`
- [ ] Implement `src/cli/commands/create.rs` (stub)
- [ ] Implement `src/cli/commands/list.rs` (stub)
- [ ] Implement `src/cli/commands/test.rs` (stub)
- [ ] Implement `src/cli/commands/deploy.rs` (stub)
- [ ] Implement `src/cli/commands/delete.rs` (stub)
- [ ] Implement `src/cli/commands/show.rs` (stub)
- [ ] Wire commands to main.rs

**Acceptance Criteria:**
- All commands callable from CLI
- Stubs print "Not implemented" with helpful message
- Command routing works correctly
- Tests verify command execution

---

#### 1.8 Main Entry Point
- [ ] Implement `src/main.rs`
- [ ] Implement `src/lib.rs`
- [ ] Initialize logger
- [ ] Load configuration
- [ ] Route commands
- [ ] Handle top-level errors gracefully

**Acceptance Criteria:**
- Binary builds and runs
- Configuration loads correctly
- Commands dispatch properly
- Errors display user-friendly messages
- Exit codes correct (0 for success, 1 for error)

---

### Phase 1 Deliverables

**Working Features:**
- ✅ Compiling Rust project
- ✅ Configuration from environment
- ✅ 6 tools registered
- ✅ CLI with all commands (stubbed)
- ✅ Error handling framework
- ✅ Logging system

**Test Coverage:** > 70%

**Demo Command:**
```bash
$ cargo run -- create --help
$ cargo run -- list
No workflows found.
```

---

## Phase 2: Core Logic (Weeks 3-5)

**Goal:** Implement AI analysis, DSL compilation, and Python code generation

### Tasks

#### 2.1 AI Client
- [ ] Implement `src/analyzer/mod.rs`
- [ ] Implement `src/analyzer/ai_client.rs`
- [ ] Add Anthropic API integration
- [ ] Implement rate limiting
- [ ] Implement retry logic with exponential backoff
- [ ] Add response validation
- [ ] Write integration tests (with mocked API)

**Acceptance Criteria:**
- Successfully calls Anthropic API
- Handles rate limiting gracefully
- Retries transient failures
- Validates JSON responses
- Tests verify retry logic

---

#### 2.2 Requirements Analyzer
- [ ] Implement `src/analyzer/requirements.rs`
- [ ] Define WorkflowRequirements struct
- [ ] Implement RequirementsAnalyzer
- [ ] Create analysis prompt template
- [ ] Implement prompt builder
- [ ] Parse AI responses into structured data
- [ ] Validate requirements against tool registry
- [ ] Write unit and integration tests

**Acceptance Criteria:**
- Extracts tools, credentials, steps from description
- Identifies approval points correctly
- Validates tools exist in registry
- Handles ambiguous descriptions gracefully
- Tests cover various description types

---

#### 2.3 Tool Selector
- [ ] Implement `src/analyzer/tool_selector.rs`
- [ ] Implement tool selection logic
- [ ] Validate tool compatibility
- [ ] Resolve tool dependencies
- [ ] Write unit tests

**Acceptance Criteria:**
- Selects appropriate tools for requirements
- Detects incompatible tool combinations
- Resolves transitive dependencies
- Tests verify selection logic

---

#### 2.4 DSL Models
- [ ] Implement `src/compiler/mod.rs`
- [ ] Implement `src/compiler/dsl/mod.rs`
- [ ] Implement `src/compiler/dsl/models.rs`
- [ ] Define WorkflowSpec structure
- [ ] Define StepSpec and all variants
- [ ] Define InputMapping and OutputMapping
- [ ] Implement serialization/deserialization
- [ ] Write schema validation

**Acceptance Criteria:**
- All DSL structures defined
- Serde serialization works
- Validation catches invalid specs
- Tests verify all variants

---

#### 2.5 Workflow Compiler
- [ ] Implement `src/compiler/workflow_compiler.rs`
- [ ] Implement WorkflowCompiler
- [ ] Map WorkflowRequirements → WorkflowSpec
- [ ] Implement data flow analysis
- [ ] Assign default policies (timeouts, retries)
- [ ] Validate compiled specs
- [ ] Write unit tests

**Acceptance Criteria:**
- Requirements compile to valid specs
- Data flow between steps correct
- Default policies applied
- Invalid requirements rejected
- Tests cover edge cases

---

#### 2.6 Code Generator Setup
- [ ] Implement `src/compiler/code_generator.rs`
- [ ] Implement PythonCodeGenerator
- [ ] Initialize Tera template engine
- [ ] Create helper functions (to_class_name, etc.)
- [ ] Implement syntax validation

**Acceptance Criteria:**
- Tera engine initialized with templates
- Helper functions work correctly
- Python syntax validation works
- Tests verify helper functions

---

#### 2.7 Tera Templates
- [ ] Create `templates/` directory
- [ ] Implement `templates/workflow.py.tera`
- [ ] Implement `templates/worker.py.tera`
- [ ] Implement `templates/requirements.txt.tera`
- [ ] Implement `templates/Dockerfile.tera`
- [ ] Create partial templates for step types
- [ ] Test template rendering with sample data

**Template Partials:**
```
templates/partials/
├── step_agent.py.tera
├── step_activity.py.tera
├── step_approval.py.tera
├── step_conditional.py.tera
├── retry_policy.py.tera
└── error_handler.py.tera
```

**Acceptance Criteria:**
- Templates render without errors
- Generated Python has valid syntax
- All step types supported
- Templates handle edge cases
- Tests verify rendering

---

#### 2.8 Code Generation Integration
- [ ] Implement generate_workflow method
- [ ] Implement generate_worker method
- [ ] Implement generate_requirements method
- [ ] Implement generate_dockerfile method
- [ ] Integrate with compiler
- [ ] Write end-to-end tests

**Acceptance Criteria:**
- Generates complete, valid Python code
- Generated code passes syntax check
- All dependencies included
- Dockerfile builds successfully
- Tests verify full code generation

---

#### 2.9 Implement Create Command
- [ ] Complete `src/cli/commands/create.rs`
- [ ] Integrate analyzer
- [ ] Integrate compiler
- [ ] Integrate code generator
- [ ] Add user prompts for description
- [ ] Display analysis results
- [ ] Save generated code to filesystem
- [ ] Write integration tests

**Acceptance Criteria:**
- End-to-end workflow creation works
- User can provide description interactively
- Analysis results displayed clearly
- Code saved to appropriate directory
- Tests verify full create flow

---

### Phase 2 Deliverables

**Working Features:**
- ✅ AI-powered requirement analysis
- ✅ DSL compilation
- ✅ Python code generation from templates
- ✅ Complete `create` command (without deployment)

**Test Coverage:** > 75%

**Demo Command:**
```bash
$ cargo run -- create
Describe your workflow: When a Slack message arrives on #travel...
🤖 Analyzing requirements...
✓ Identified 4 tools, 3 credentials, 1 approval point
📝 Generating code...
✓ Workflow code generated: ./output/workflows/slack_travel_booking/
```

---

## Phase 3: Testing Infrastructure (Week 6)

**Goal:** Enable dry-run testing before deployment

### Tasks

#### 3.1 Mock Data Generator
- [ ] Implement `src/testing/mod.rs`
- [ ] Implement `src/testing/mock_generator.rs`
- [ ] Define MockDataSet structure
- [ ] Implement trigger data generation
- [ ] Implement service response mocking
- [ ] Implement AI response mocking
- [ ] Create realistic mock data for each tool
- [ ] Write unit tests

**Acceptance Criteria:**
- Generates realistic mock data for all tools
- Mock data matches tool signatures
- Trigger data appropriate for trigger type
- Tests verify mock generation

---

#### 3.2 Python Executor
- [ ] Implement `src/testing/python_executor.rs`
- [ ] Define PythonExecutor structure
- [ ] Implement venv creation
- [ ] Implement dependency installation
- [ ] Implement script execution
- [ ] Capture stdout/stderr
- [ ] Implement timeout handling
- [ ] Write integration tests

**Acceptance Criteria:**
- Creates Python venv successfully
- Installs dependencies from requirements.txt
- Executes scripts with proper isolation
- Captures all output
- Respects timeouts
- Tests verify execution

---

#### 3.3 Mock Service Layer
- [ ] Create Python mock framework
- [ ] Implement mock Temporal client
- [ ] Implement mock activity functions
- [ ] Implement monkey-patching utilities
- [ ] Write tests for mock layer

**Python Files:**
```
testing_runtime/
├── __init__.py
├── mock_temporal.py
├── mock_activities.py
└── mock_injector.py
```

**Acceptance Criteria:**
- Mock Temporal executes workflows locally
- Mock activities return predefined responses
- Monkey-patching works reliably
- Tests verify mock behavior

---

#### 3.4 Trace Collector
- [ ] Implement `src/testing/trace_collector.rs`
- [ ] Define TraceEvent structure
- [ ] Implement log parsing
- [ ] Implement trace formatting
- [ ] Implement trace summarization
- [ ] Write unit tests

**Acceptance Criteria:**
- Parses structured logs into trace events
- Formats trace for display
- Generates summary statistics
- Tests verify parsing

---

#### 3.5 Cost Estimator
- [ ] Add cost estimation logic
- [ ] Define cost models for each tool
- [ ] Calculate per-execution costs
- [ ] Implement cost aggregation
- [ ] Write unit tests

**Acceptance Criteria:**
- Estimates AI API costs accurately
- Estimates external API costs
- Provides cost breakdown
- Tests verify calculations

---

#### 3.6 Dry-Run Orchestrator
- [ ] Implement `src/testing/dry_runner.rs`
- [ ] Define DryRunConfig and DryRunResult
- [ ] Implement dry-run orchestration
- [ ] Integrate mock generator
- [ ] Integrate Python executor
- [ ] Integrate trace collector
- [ ] Add cleanup logic
- [ ] Write integration tests

**Acceptance Criteria:**
- Executes workflow in isolated environment
- Injects mocks successfully
- Collects complete trace
- Estimates costs
- Cleans up after execution
- Tests verify full dry-run

---

#### 3.7 Implement Test Command
- [ ] Complete `src/cli/commands/test.rs`
- [ ] Integrate dry-run orchestrator
- [ ] Display trace and results
- [ ] Save trace to file (optional)
- [ ] Write integration tests

**Acceptance Criteria:**
- User can test generated workflows
- Dry-run results displayed clearly
- Errors reported with context
- Tests verify test command

---

### Phase 3 Deliverables

**Working Features:**
- ✅ Dry-run execution with mocks
- ✅ Execution trace collection
- ✅ Cost estimation
- ✅ Complete `test` command

**Test Coverage:** > 75%

**Demo Command:**
```bash
$ cargo run -- test workflow_abc123
🧪 Running dry-run...
[10:30:00] ▶ Step 1: parse_request
[10:30:02] ✓ Step 1 completed (2.3s)
[10:30:02] ▶ Step 2: search_flights
[10:30:03] ✓ Step 2 completed (1.3s)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ All steps completed successfully
💰 Estimated cost: $0.06 per execution
```

---

## Phase 4: Deployment & Storage (Weeks 7-8)

**Goal:** Deploy workflows to AWS and manage lifecycle

### Tasks

#### 4.1 AWS Secrets Manager Integration
- [ ] Implement `src/credentials/mod.rs`
- [ ] Implement `src/credentials/aws_secrets.rs`
- [ ] Implement AwsSecretsClient
- [ ] Implement store_credential
- [ ] Implement retrieve_credential
- [ ] Implement delete_credential
- [ ] Write integration tests (LocalStack)

**Acceptance Criteria:**
- Stores credentials in Secrets Manager
- Retrieves credentials by key
- Handles missing secrets gracefully
- Tests verify operations

---

#### 4.2 Credential Validators
- [ ] Implement `src/credentials/validator.rs`
- [ ] Define CredentialValidator trait
- [ ] Implement SlackValidator
- [ ] Implement AnthropicValidator
- [ ] Implement AmadeusValidator
- [ ] Implement GenericHttpValidator
- [ ] Write unit tests (mocked APIs)

**Acceptance Criteria:**
- Validates credentials against real APIs
- Returns clear validation results
- Handles API errors gracefully
- Tests verify validation logic

---

#### 4.3 Credential Interrogator
- [ ] Implement `src/credentials/interrogator.rs`
- [ ] Implement CredentialInterrogator
- [ ] Implement interactive credential gathering
- [ ] Integrate validators
- [ ] Integrate AWS Secrets Manager
- [ ] Add credential caching
- [ ] Write integration tests

**Acceptance Criteria:**
- Prompts for required credentials
- Validates before storing
- Stores in Secrets Manager
- Returns credential mapping
- Tests verify flow

---

#### 4.4 Code Packager
- [ ] Implement `src/deployment/mod.rs`
- [ ] Implement `src/deployment/packager.rs`
- [ ] Define PackageArtifact structure
- [ ] Implement code packaging (tar.gz)
- [ ] Generate manifest.json
- [ ] Calculate checksums
- [ ] Write unit tests

**Acceptance Criteria:**
- Packages all required files
- Generates valid tar.gz
- Creates manifest with metadata
- Verifies checksums
- Tests verify packaging

---

#### 4.5 AWS Deployer - S3
- [ ] Implement `src/deployment/aws.rs`
- [ ] Initialize AWS SDK clients
- [ ] Implement S3 upload
- [ ] Add S3 versioning support
- [ ] Write integration tests (LocalStack)

**Acceptance Criteria:**
- Uploads artifacts to S3
- Uses versioning
- Validates uploads
- Tests verify S3 operations

---

#### 4.6 AWS Deployer - ECS
- [ ] Implement ECS task definition registration
- [ ] Implement ECS service creation
- [ ] Configure Fargate launch type
- [ ] Set up IAM roles
- [ ] Implement health checks
- [ ] Write integration tests

**Acceptance Criteria:**
- Registers valid task definitions
- Creates ECS services
- Configures networking correctly
- Services start successfully
- Tests verify ECS operations

---

#### 4.7 AWS Deployer - API Gateway
- [ ] Implement `src/deployment/webhook.rs`
- [ ] Create API Gateway REST API
- [ ] Configure webhook endpoint
- [ ] Set up request validation
- [ ] Integrate with workflow trigger
- [ ] Write integration tests

**Acceptance Criteria:**
- Creates API Gateway endpoints
- Configures routing correctly
- Validates requests
- Tests verify API Gateway setup

---

#### 4.8 DynamoDB Storage
- [ ] Implement `src/storage/mod.rs`
- [ ] Implement `src/storage/dynamodb.rs`
- [ ] Implement `src/storage/models.rs`
- [ ] Define WorkflowRecord structure
- [ ] Implement save_workflow
- [ ] Implement get_workflow
- [ ] Implement list_workflows
- [ ] Implement update_workflow_status
- [ ] Implement delete_workflow
- [ ] Write integration tests (LocalStack)

**Acceptance Criteria:**
- Saves workflows to DynamoDB
- Retrieves workflows by ID
- Lists with filtering
- Updates status correctly
- Tests verify all operations

---

#### 4.9 Deployment Orchestrator
- [ ] Implement `src/deployment/deployer.rs`
- [ ] Implement full deployment flow
- [ ] Integrate packager, AWS deployer, storage
- [ ] Add deployment status tracking
- [ ] Implement rollback on failure
- [ ] Write integration tests

**Acceptance Criteria:**
- Orchestrates full deployment
- Handles partial failures
- Tracks deployment status
- Rolls back on errors
- Tests verify deployment

---

#### 4.10 Implement Deploy Command
- [ ] Complete `src/cli/commands/deploy.rs`
- [ ] Integrate credential interrogator
- [ ] Integrate deployment orchestrator
- [ ] Display deployment progress
- [ ] Handle deployment errors
- [ ] Write integration tests

**Acceptance Criteria:**
- User can deploy workflows
- Credentials gathered interactively
- Progress displayed clearly
- Deployment info shown on success
- Tests verify deploy command

---

#### 4.11 Implement List Command
- [ ] Complete `src/cli/commands/list.rs`
- [ ] Integrate DynamoDB storage
- [ ] Format workflow list for display
- [ ] Add filtering by status
- [ ] Write integration tests

**Acceptance Criteria:**
- Lists all workflows
- Filters by status work
- Display is readable
- Tests verify list command

---

#### 4.12 Implement Delete Command
- [ ] Complete `src/cli/commands/delete.rs`
- [ ] Integrate AWS deployer (cleanup)
- [ ] Integrate DynamoDB storage
- [ ] Add confirmation prompt
- [ ] Implement full cleanup (ECS, S3, Secrets)
- [ ] Write integration tests

**Acceptance Criteria:**
- Deletes workflow and resources
- Confirms before deletion
- Cleans up all AWS resources
- Tests verify delete command

---

#### 4.13 Implement Show Command
- [ ] Complete `src/cli/commands/show.rs`
- [ ] Retrieve workflow details
- [ ] Display spec and deployment info
- [ ] Format for readability
- [ ] Write integration tests

**Acceptance Criteria:**
- Shows complete workflow details
- Displays deployment status
- Includes webhook URLs
- Tests verify show command

---

### Phase 4 Deliverables

**Working Features:**
- ✅ Credential management with AWS Secrets Manager
- ✅ Full AWS deployment (ECS, API Gateway)
- ✅ DynamoDB storage
- ✅ All CLI commands functional

**Test Coverage:** > 70%

**Demo Command:**
```bash
$ cargo run -- deploy workflow_abc123
🔐 Gathering credentials...
  ✓ Slack bot token (found in Secrets Manager)
  ⚠ Anthropic API key (not found)
  Enter Anthropic API key: sk-ant-...
  ✓ Validated and stored

📦 Packaging workflow...
☁️ Deploying to AWS...
  ✓ Uploaded to S3
  ✓ Registered ECS task definition
  ✓ Created ECS service
  ✓ Configured API Gateway
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Deployment successful!
Webhook URL: https://api.example.com/workflows/abc123/trigger
```

---

## Phase 5: Python Runtime (Parallel with Phases 2-4)

**Goal:** Create reusable Python library for generated workflows

### Tasks

#### 5.1 Python Project Setup
- [ ] Create `python_runtime/` directory
- [ ] Create requirements.txt
- [ ] Create setup.py (optional)
- [ ] Initialize __init__.py files
- [ ] Set up pytest configuration

---

#### 5.2 Base Activity
- [ ] Implement `python_runtime/activities/base.py`
- [ ] Define ActivityContext dataclass
- [ ] Define BaseActivity ABC
- [ ] Write tests

**Acceptance Criteria:**
- Base activity provides common interface
- Context properly typed
- Tests verify interface

---

#### 5.3 Credential Utilities
- [ ] Implement `python_runtime/activities/utils/credentials.py`
- [ ] Implement AWS Secrets Manager retrieval
- [ ] Implement credential caching
- [ ] Write tests

**Acceptance Criteria:**
- Retrieves credentials from Secrets Manager
- Caches for performance
- Tests verify retrieval

---

#### 5.4 Messaging Activities
- [ ] Implement `python_runtime/activities/messaging/slack.py`
- [ ] Implement send_message activity
- [ ] Implement receive_message activity (webhook)
- [ ] Add error handling
- [ ] Write tests (mocked Slack API)

**Acceptance Criteria:**
- Sends Slack messages successfully
- Handles errors gracefully
- Tests verify functionality

---

#### 5.5 AI Activities
- [ ] Implement `python_runtime/activities/ai/claude.py`
- [ ] Implement run_claude_agent activity
- [ ] Add tool calling support
- [ ] Implement streaming (optional)
- [ ] Write tests (mocked Anthropic API)

**Acceptance Criteria:**
- Executes Claude agents
- Supports tools parameter
- Handles errors
- Tests verify execution

---

#### 5.6 Travel Activities
- [ ] Implement `python_runtime/activities/travel/amadeus.py`
- [ ] Implement search_flights activity
- [ ] Implement book_flight activity
- [ ] Handle OAuth token refresh
- [ ] Write tests (mocked Amadeus API)

**Acceptance Criteria:**
- Searches flights successfully
- Books flights with confirmation
- Handles authentication
- Tests verify functionality

---

#### 5.7 Approval Workflow
- [ ] Implement `python_runtime/workflows/approval.py`
- [ ] Implement HumanApprovalWorkflow
- [ ] Implement signal handlers (approve/reject)
- [ ] Implement timeout logic
- [ ] Write tests (Temporal test environment)

**Acceptance Criteria:**
- Approval workflow works correctly
- Signals processed properly
- Timeouts enforced
- Tests verify approval flow

---

#### 5.8 Worker Template
- [ ] Implement `python_runtime/worker/main.py`
- [ ] Add Temporal connection logic
- [ ] Add dynamic workflow/activity loading
- [ ] Add graceful shutdown
- [ ] Write tests

**Acceptance Criteria:**
- Worker connects to Temporal
- Loads workflows/activities dynamically
- Handles shutdown gracefully
- Tests verify worker behavior

---

### Phase 5 Deliverables

**Working Features:**
- ✅ Complete Python runtime library
- ✅ 6 activity implementations
- ✅ Approval workflow
- ✅ Worker template

**Test Coverage:** > 80%

---

## Phase 6: Infrastructure (Parallel with other phases)

**Goal:** Terraform configuration for AWS infrastructure

### Tasks

#### 6.1 Terraform Setup
- [ ] Create `infrastructure/terraform/` directory
- [ ] Create main.tf
- [ ] Create variables.tf
- [ ] Create outputs.tf
- [ ] Create terraform.tfvars.example

---

#### 6.2 DynamoDB Tables
- [ ] Create `infrastructure/terraform/dynamodb.tf`
- [ ] Define ai-workflows table
- [ ] Define ai-workflow-executions table
- [ ] Add global secondary indexes
- [ ] Configure billing mode

**Acceptance Criteria:**
- Tables created with correct schema
- Indexes support required queries
- Pay-per-request billing configured

---

#### 6.3 S3 Bucket
- [ ] Create `infrastructure/terraform/s3.tf`
- [ ] Define code bucket
- [ ] Enable versioning
- [ ] Configure encryption
- [ ] Set lifecycle policies

**Acceptance Criteria:**
- Bucket created with encryption
- Versioning enabled
- Lifecycle policies configured

---

#### 6.4 ECS Cluster
- [ ] Create `infrastructure/terraform/ecs.tf`
- [ ] Define ECS cluster
- [ ] Configure Fargate capacity provider
- [ ] Set up CloudWatch log groups

**Acceptance Criteria:**
- Cluster created and ready
- Fargate configured
- Logging enabled

---

#### 6.5 VPC and Networking
- [ ] Create `infrastructure/terraform/vpc.tf`
- [ ] Define VPC or use existing
- [ ] Create private subnets
- [ ] Configure NAT gateway
- [ ] Set up VPC endpoints

**Acceptance Criteria:**
- VPC configured correctly
- Private subnets for ECS tasks
- NAT gateway for outbound access
- VPC endpoints for AWS services

---

#### 6.6 IAM Roles
- [ ] Create `infrastructure/terraform/iam.tf`
- [ ] Define ECS task execution role
- [ ] Define ECS task role
- [ ] Add policies for Secrets Manager, DynamoDB, S3
- [ ] Add CloudWatch Logs permissions

**Acceptance Criteria:**
- Roles have least privilege
- All required permissions granted
- Trust relationships correct

---

#### 6.7 Security Groups
- [ ] Create `infrastructure/terraform/security.tf`
- [ ] Define security group for ECS tasks
- [ ] Configure ingress/egress rules
- [ ] Follow least privilege

**Acceptance Criteria:**
- Security groups restrict traffic
- Only required ports open
- Egress allows necessary connections

---

### Phase 6 Deliverables

**Working Features:**
- ✅ Complete Terraform configuration
- ✅ Infrastructure deployable with `terraform apply`
- ✅ All AWS resources defined

**Deployment:**
```bash
$ cd infrastructure/terraform
$ terraform init
$ terraform plan
$ terraform apply
```

---

## Phase 7: Documentation & Examples (Final Week)

**Goal:** Complete documentation and example workflows

### Tasks

#### 7.1 README
- [ ] Create comprehensive README.md
- [ ] Add project description
- [ ] Add installation instructions
- [ ] Add usage examples
- [ ] Add architecture diagram
- [ ] Add troubleshooting guide

---

#### 7.2 API Documentation
- [ ] Generate rustdoc comments for all public APIs
- [ ] Create module-level documentation
- [ ] Add code examples in docs
- [ ] Generate docs with `cargo doc`

---

#### 7.3 Example Workflows
- [ ] Create `docs/examples/` directory
- [ ] Add travel booking example
- [ ] Add customer support example
- [ ] Add invoice processing example
- [ ] Document each example

---

#### 7.4 Deployment Guide
- [ ] Create DEPLOYMENT.md
- [ ] Document prerequisites
- [ ] Document Terraform setup
- [ ] Document application setup
- [ ] Add monitoring guide

---

#### 7.5 Development Guide
- [ ] Create DEVELOPMENT.md
- [ ] Document build process
- [ ] Document testing strategy
- [ ] Add contribution guidelines
- [ ] Document release process

---

### Phase 7 Deliverables

**Working Features:**
- ✅ Complete documentation
- ✅ 3 example workflows
- ✅ Deployment guide
- ✅ Development guide

---

## Testing Strategy

### Unit Tests
- **Target:** Individual functions and modules
- **Coverage:** > 70%
- **Tools:** cargo test, mockall
- **Run:** `cargo test --lib`

### Integration Tests
- **Target:** Module interactions
- **Coverage:** Key workflows
- **Tools:** cargo test, LocalStack (AWS), mockito (HTTP)
- **Run:** `cargo test --test '*'`

### End-to-End Tests
- **Target:** Complete user journeys
- **Tools:** Real AWS (test account), Temporal test server
- **Run:** `E2E_TESTS=1 cargo test e2e`

### Python Tests
- **Target:** Python runtime library
- **Coverage:** > 80%
- **Tools:** pytest, pytest-asyncio
- **Run:** `cd python_runtime && pytest`

---

## Success Metrics

### Performance
- [ ] Workflow creation in < 2 minutes
- [ ] Dry-run in < 1 minute
- [ ] Deployment in < 5 minutes
- [ ] **Total: < 10 minutes from description to production**

### Quality
- [ ] 95%+ dry-run success rate
- [ ] Zero manual code writing required
- [ ] Secure credential storage (AWS Secrets Manager)
- [ ] All workflows execute successfully

### Scope
- [ ] 3-5 workflow patterns supported
- [ ] 6+ pre-built tools
- [ ] Single-tenant (POC)

---

## Risk Mitigation

### Technical Risks

**Risk:** AI analysis produces invalid requirements
**Mitigation:** Comprehensive validation after analysis, retry with refined prompt

**Risk:** Generated Python code has bugs
**Mitigation:** Mandatory dry-run before deployment, syntax validation

**Risk:** AWS deployment failures
**Mitigation:** Extensive integration testing, rollback on failure

**Risk:** Credential validation issues
**Mitigation:** Multiple validators per service, clear error messages

### Timeline Risks

**Risk:** Phase overruns
**Mitigation:** Build incrementally, prioritize core features, skip optional features

**Risk:** Dependency issues
**Mitigation:** Lock dependency versions, test early

---

## Next Steps

1. **Week 1-2:** Complete Phase 1 (Foundation)
2. **Week 3-5:** Complete Phase 2 (Core Logic)
3. **Week 6:** Complete Phase 3 (Testing)
4. **Week 7-8:** Complete Phase 4 (Deployment)
5. **Throughout:** Complete Phase 5 (Python) and Phase 6 (Infrastructure)
6. **Final Week:** Complete Phase 7 (Documentation)

**Current Status:** Starting Phase 1.1 - Project Initialization

**Let's begin implementation!**
