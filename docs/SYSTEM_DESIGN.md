# AI Workflow Generator - Detailed System Design

## Executive Summary

The AI Workflow Generator (codename: Latempo) is a Rust-based CLI tool that transforms natural language workflow descriptions into production-ready, executable Temporal workflows with integrated AI agents, automatically deployed to AWS infrastructure.

**Core Value Proposition:** From natural language description to production deployment in under 10 minutes, with zero manual code writing.

---

## System Architecture

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER INTERFACE                           │
│                    (Rust CLI with Rich TUI)                      │
└────────────┬────────────────────────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    WORKFLOW ORCHESTRATOR                         │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  1. Analysis Phase                                       │   │
│  │     ├─ Natural Language Parser                           │   │
│  │     ├─ AI Requirements Extractor (Claude API)            │   │
│  │     └─ Tool Selector                                     │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  2. Credential Management Phase                          │   │
│  │     ├─ Interactive Credential Gatherer                   │   │
│  │     ├─ Credential Validator                              │   │
│  │     └─ AWS Secrets Manager Integration                   │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  3. Compilation Phase                                    │   │
│  │     ├─ DSL Compiler (Requirements → WorkflowSpec)        │   │
│  │     ├─ Python Code Generator (Tera Templates)            │   │
│  │     └─ Syntax Validator                                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  4. Testing Phase                                        │   │
│  │     ├─ Mock Data Generator                               │   │
│  │     ├─ Python Virtual Environment Manager                │   │
│  │     ├─ Dry-Run Executor                                  │   │
│  │     └─ Trace Collector & Cost Estimator                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  5. Deployment Phase                                     │   │
│  │     ├─ Code Packager (tar.gz + manifest)                 │   │
│  │     ├─ AWS ECS Deployer                                  │   │
│  │     ├─ API Gateway Configurator                          │   │
│  │     └─ DynamoDB Record Manager                           │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────┬───────────────────────────────────────────────────┘
              │
    ┌─────────┴─────────┐
    ▼                   ▼
┌──────────┐      ┌──────────────────────────────────┐
│   AWS    │      │      PYTHON RUNTIME              │
│ Services │      │                                  │
│          │      │  ┌────────────────────────────┐  │
│ Secrets  │◀────▶│  │  Temporal Workers          │  │
│ Manager  │      │  │  - Workflow Execution      │  │
│          │      │  │  - Activity Orchestration  │  │
│ DynamoDB │◀────▶│  └────────────────────────────┘  │
│ Tables   │      │                                  │
│          │      │  ┌────────────────────────────┐  │
│ S3       │◀────▶│  │  Activity Library          │  │
│ Buckets  │      │  │  - Messaging (Slack)       │  │
│          │      │  │  - AI Agents (Claude)      │  │
│ ECS      │◀────▶│  │  - Travel APIs (Amadeus)   │  │
│ Fargate  │      │  │  - Utilities               │  │
│          │      │  └────────────────────────────┘  │
│ API      │◀────▶│                                  │
│ Gateway  │      │  ┌────────────────────────────┐  │
│          │      │  │  Approval Workflows        │  │
│CloudWatch│◀────▶│  │  - Human-in-the-Loop       │  │
│          │      │  │  - Signal Handlers         │  │
└──────────┘      │  └────────────────────────────┘  │
                  └──────────────────────────────────┘
```

---

## Component Design

### 1. Analysis Component

**Responsibility:** Transform natural language into structured requirements

**Sub-components:**

#### 1.1 AI Client (`src/analyzer/ai_client.rs`)
```rust
pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
    rate_limiter: RateLimiter,
}

impl AnthropicClient {
    pub async fn analyze(&self, prompt: &str) -> Result<String>;
    pub async fn analyze_with_retry(&self, prompt: &str, max_retries: u32) -> Result<String>;
    fn handle_rate_limit(&self, retry_after: u64) -> impl Future<Output = ()>;
}
```

**Design Decisions:**
- Uses exponential backoff for retries
- Implements rate limiting to avoid API throttling
- Validates response format before parsing

#### 1.2 Requirements Analyzer (`src/analyzer/requirements.rs`)
```rust
pub struct RequirementsAnalyzer {
    ai_client: AnthropicClient,
    tool_registry: Arc<ToolRegistry>,
    prompt_template: String,
}

impl RequirementsAnalyzer {
    pub async fn analyze(&self, description: &str) -> Result<WorkflowRequirements>;
    fn build_analysis_prompt(&self, description: &str) -> String;
    fn validate_requirements(&self, req: &WorkflowRequirements) -> Result<()>;
}
```

**Analysis Prompt Strategy:**
- Few-shot learning with examples
- Structured output format (JSON)
- Tool registry embedded in prompt
- Explicit guidelines for approval points

#### 1.3 Tool Selector (`src/analyzer/tool_selector.rs`)
```rust
pub struct ToolSelector {
    registry: Arc<ToolRegistry>,
}

impl ToolSelector {
    pub fn select_tools(&self, requirements: &WorkflowRequirements) -> Result<Vec<ToolDefinition>>;
    pub fn validate_tool_compatibility(&self, tools: &[ToolDefinition]) -> Result<()>;
    fn resolve_dependencies(&self, tools: &[ToolDefinition]) -> Vec<String>;
}
```

---

### 2. Credential Management Component

**Responsibility:** Securely gather, validate, and store credentials

#### 2.1 Credential Interrogator (`src/credentials/interrogator.rs`)

**Interaction Flow:**
```
1. Identify required credentials from requirements
2. For each credential:
   a. Check if already stored in AWS Secrets Manager
   b. If not stored:
      - Display context-aware prompt
      - Collect credential interactively
      - Validate against service API
      - Store in AWS Secrets Manager
   c. Return Secret ARN
3. Build credential mapping (service_key -> ARN)
```

**Security Principles:**
- Credentials never logged
- Immediate encryption via AWS KMS
- Validation before storage
- Ephemeral memory storage (cleared on drop)

#### 2.2 Validators (`src/credentials/validator.rs`)

```rust
#[async_trait]
pub trait CredentialValidator {
    async fn validate(&self, credential: &str) -> Result<ValidationResult>;
}

pub struct SlackValidator {
    test_endpoint: &'static str,
}

impl CredentialValidator for SlackValidator {
    async fn validate(&self, token: &str) -> Result<ValidationResult> {
        // Call Slack auth.test API
        // Return success/failure with details
    }
}

// Similar implementations for:
// - AmadeusValidator (OAuth flow)
// - AnthropicValidator (simple key check)
// - GenericHttpValidator (custom endpoints)
```

---

### 3. Compilation Component

**Responsibility:** Transform requirements into executable code

#### 3.1 DSL Design

**Internal DSL Structure:**
```rust
pub struct WorkflowSpec {
    pub metadata: WorkflowMetadata,
    pub trigger: TriggerSpec,
    pub steps: Vec<StepSpec>,
    pub error_handling: ErrorHandlingSpec,
    pub credentials: CredentialMapping,
}

pub struct StepSpec {
    pub id: String,
    pub step_type: StepType,
    pub inputs: Vec<InputMapping>,  // Where data comes from
    pub outputs: Vec<OutputMapping>, // Where data goes
    pub config: StepConfig,
    pub policies: ExecutionPolicies,
}

pub struct InputMapping {
    pub name: String,
    pub source: DataSource,  // PreviousStep, TriggerData, Constant
    pub transform: Option<Transform>,
}

pub enum DataSource {
    TriggerData { path: String },
    PreviousStep { step_id: String, field: String },
    Constant { value: serde_json::Value },
}
```

**Design Rationale:**
- Explicit data flow between steps
- Separates business logic from execution policies
- Enables validation before code generation
- Supports future optimizations (parallelization, caching)

#### 3.2 Code Generator (`src/compiler/code_generator.rs`)

**Template System Design:**

```
templates/
├── workflow.py.tera         # Main workflow class
├── worker.py.tera           # Worker entrypoint
├── requirements.txt.tera    # Python dependencies
├── Dockerfile.tera          # Container definition
└── partials/
    ├── step_agent.py.tera       # Agent step template
    ├── step_activity.py.tera    # Activity step template
    ├── step_approval.py.tera    # Approval step template
    ├── step_conditional.py.tera # Conditional step template
    ├── retry_policy.py.tera     # Retry configuration
    └── error_handler.py.tera    # Error handling
```

**Template Context:**
```rust
#[derive(Serialize)]
struct TemplateContext {
    workflow_name: String,
    workflow_class: String,
    description: String,
    timestamp: String,
    steps: Vec<StepContext>,
    imports: Vec<String>,
    credentials: HashMap<String, String>,
    task_queue: String,
}

#[derive(Serialize)]
struct StepContext {
    id: String,
    description: String,
    step_type: String,
    var_name: String,
    config: serde_json::Value,
    timeout: u32,
    retry: RetryContext,
}
```

---

### 4. Testing Component

**Responsibility:** Validate workflows before deployment

#### 4.1 Dry-Run Architecture

```
┌──────────────────────────────────────┐
│     Dry-Run Orchestrator             │
└───────┬──────────────────────────────┘
        │
        ├─▶ 1. Create isolated temp directory
        │
        ├─▶ 2. Generate mock data
        │      └─ Based on tool signatures
        │
        ├─▶ 3. Create Python venv
        │      └─ Install dependencies
        │
        ├─▶ 4. Inject mock layer
        │      └─ Monkey-patch external services
        │
        ├─▶ 5. Execute workflow
        │      └─ With instrumentation
        │
        ├─▶ 6. Collect execution trace
        │      └─ Parse structured logs
        │
        ├─▶ 7. Estimate costs
        │      └─ Based on API calls
        │
        └─▶ 8. Cleanup
               └─ Remove temp files
```

#### 4.2 Mock Data Strategy

**Mock Generator Logic:**
```rust
impl MockDataGenerator {
    fn generate_for_workflow(&self, spec: &WorkflowSpec) -> Result<MockDataSet> {
        let mut dataset = MockDataSet::new();

        // Generate realistic trigger data
        dataset.trigger_data = self.generate_trigger_data(&spec.trigger)?;

        // For each tool, generate realistic responses
        for step in &spec.steps {
            if let StepType::Activity { config } = &step.step_type {
                let mock = self.generate_service_response(
                    &config.activity_name,
                    &step.inputs
                )?;
                dataset.service_responses.insert(step.id.clone(), mock);
            }
        }

        Ok(dataset)
    }

    fn generate_service_response(&self, tool: &str, inputs: &[InputMapping]) -> Result<Value> {
        match tool {
            "search_flights" => self.generate_flight_results(inputs),
            "run_claude_agent" => self.generate_ai_response(inputs),
            "slack_send_message" => json!({"ok": true, "ts": "1234567.890"}),
            _ => json!({"success": true}),
        }
    }
}
```

---

### 5. Deployment Component

**Responsibility:** Package and deploy to AWS infrastructure

#### 5.1 Deployment Architecture

```
Deployment Flow:
1. Package → 2. Upload → 3. Register → 4. Launch → 5. Configure

┌─────────────┐
│  1. Package │ Create deployment artifact
└──────┬──────┘
       │ • Generated workflow code
       │ • Python runtime library
       │ • requirements.txt
       │ • Dockerfile
       │ • manifest.json
       ▼
┌─────────────┐
│  2. Upload  │ S3 bucket
└──────┬──────┘
       │ • Versioned storage
       │ • Checksum validation
       ▼
┌─────────────┐
│ 3. Register │ ECS Task Definition
└──────┬──────┘
       │ • Docker image reference
       │ • Environment variables
       │ • IAM roles
       │ • Resource limits
       ▼
┌─────────────┐
│  4. Launch  │ ECS Service
└──────┬──────┘
       │ • Fargate launch type
       │ • Auto-scaling config
       │ • Health checks
       ▼
┌─────────────┐
│ 5. Configure│ API Gateway (if webhook)
└─────────────┘
       │ • REST API endpoint
       │ • Lambda integration
       │ • Request validation
```

#### 5.2 ECS Task Configuration

```json
{
  "family": "ai-workflow-{workflow_id}",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "containerDefinitions": [
    {
      "name": "worker",
      "image": "{ecr_repo}:{workflow_id}",
      "essential": true,
      "environment": [
        {"name": "TEMPORAL_HOST", "value": "{temporal_host}"},
        {"name": "TEMPORAL_NAMESPACE", "value": "{namespace}"},
        {"name": "TASK_QUEUE", "value": "workflow-{workflow_id}"}
      ],
      "secrets": [
        {
          "name": "ANTHROPIC_API_KEY",
          "valueFrom": "{secret_arn}"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/ai-workflows",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "{workflow_id}"
        }
      }
    }
  ]
}
```

---

## Data Flow

### End-to-End Workflow Execution

```
User Input: "When a Slack message arrives on #travel, extract details,
             search flights, request approval, book cheapest if approved"

↓ [Analysis Phase]

WorkflowRequirements {
  tools_needed: ["slack_receive", "run_claude_agent", "search_flights", "book_flight"],
  credentials_needed: ["slack/bot_token", "anthropic/api_key", "amadeus/api_key"],
  approval_points: [ApprovalPoint { step_id: "book_flight", ... }],
  steps: [
    { type: Agent, description: "Extract travel details", ... },
    { type: Activity, description: "Search flights", tool: "search_flights" },
    { type: Approval, description: "Approve booking", ... },
    { type: Activity, description: "Book flight", tool: "book_flight" }
  ]
}

↓ [Credential Phase]

CredentialMapping {
  "slack/bot_token" -> "arn:aws:secretsmanager:...:secret:slack-bot-token-xyz",
  "anthropic/api_key" -> "arn:aws:secretsmanager:...:secret:anthropic-key-abc",
  "amadeus/api_key" -> "arn:aws:secretsmanager:...:secret:amadeus-key-def"
}

↓ [Compilation Phase]

WorkflowSpec {
  id: "550e8400-e29b-41d4-a716-446655440000",
  name: "slack_travel_booking",
  steps: [
    StepSpec {
      id: "step_1",
      step_type: Agent {
        system_prompt: "Extract origin, destination, date from message...",
        model: "claude-sonnet-4.5"
      },
      inputs: [InputMapping { source: TriggerData("text"), ... }],
      outputs: [OutputMapping { name: "parsed_request", ... }]
    },
    StepSpec {
      id: "step_2",
      step_type: Activity {
        activity_name: "search_flights",
        parameters: { /* mapped from step_1 outputs */ }
      },
      ...
    },
    ...
  ]
}

↓ [Code Generation]

Python Workflow File (350 lines)
Python Worker File (80 lines)
Dockerfile (25 lines)
requirements.txt (15 lines)

↓ [Dry-Run]

DryRunResult {
  success: true,
  execution_trace: [
    TraceEvent { timestamp: ..., step: "step_1", type: Completed, duration: 2.3s },
    TraceEvent { timestamp: ..., step: "step_2", type: Completed, duration: 1.5s },
    ...
  ],
  cost_estimate: { ai_cost: $0.05, api_cost: $0.02, total: $0.07 }
}

↓ [Deployment]

Deployment {
  workflow_id: "550e8400-...",
  task_definition_arn: "arn:aws:ecs:...:task-definition/ai-workflow-...:1",
  service_arn: "arn:aws:ecs:...:service/ai-workflows/...",
  webhook_url: "https://api.example.com/workflows/550e8400/trigger",
  status: Active
}

↓ [Runtime]

Webhook receives Slack event
  → Temporal workflow started
    → Step 1: Claude extracts details
    → Step 2: Amadeus searches flights
    → Step 3: Human approval requested (Temporal signal)
    → Step 4: Flight booked
    → Step 5: Confirmation sent to Slack
  → Workflow completed
  → Execution record saved to DynamoDB
```

---

## Security Architecture

### Defense in Depth

**Layer 1: Input Validation**
- Sanitize all user inputs
- Validate against schemas
- Reject malicious patterns
- Size limits on descriptions

**Layer 2: Code Generation**
- Use controlled templates only
- No eval() or exec() in generated code
- Parameterized queries for data access
- Input sanitization in Python code

**Layer 3: Credential Security**
- AWS Secrets Manager with KMS encryption
- Credentials never in logs or files
- IAM role-based access
- Automatic rotation (future)

**Layer 4: Network Security**
- ECS tasks in private subnets
- VPC endpoints for AWS services
- Security groups with least privilege
- No public IP addresses on workers

**Layer 5: Runtime Security**
- Container isolation
- Read-only file systems where possible
- Resource limits (CPU, memory)
- Non-root container users

**Layer 6: Audit & Monitoring**
- CloudWatch logs for all operations
- DynamoDB audit trail
- Failed authentication alerts
- Anomaly detection (future)

---

## Scalability Considerations

### Current Design (POC)
- Single tenant
- Regional deployment
- Manual scaling
- Estimated capacity: 10-50 workflows

### Future Scaling Path

**Phase 1: Horizontal Scaling (100s of workflows)**
- Auto-scaling ECS services
- DynamoDB on-demand pricing
- CloudFront for webhook endpoints
- Multi-AZ deployment

**Phase 2: Multi-Tenancy (1000s of workflows)**
- Tenant isolation (separate task queues)
- Resource quotas per tenant
- Shared infrastructure
- Billing/metering

**Phase 3: Global Distribution (10,000s of workflows)**
- Multi-region deployment
- Global DynamoDB tables
- Edge locations for webhooks
- Distributed Temporal clusters

---

## Cost Model

### Per-Workflow Fixed Costs
- DynamoDB storage: ~$0.25/month per workflow
- S3 storage: ~$0.02/month per workflow
- ECS task (always-on): ~$10-15/month per workflow

### Per-Execution Variable Costs
- AI API calls: $0.01-0.10 per execution (depends on steps)
- External APIs: $0.00-0.50 per execution (varies by service)
- Compute: $0.001-0.01 per execution
- Data transfer: negligible

### Optimization Strategies
- Lazy worker startup (on-demand)
- Shared workers for low-volume workflows
- Caching AI responses
- Batch API calls where possible

---

## Monitoring & Observability

### Metrics to Track

**Application Metrics:**
- Workflow creation time
- Dry-run success rate
- Deployment success rate
- Average time to production

**Runtime Metrics:**
- Workflow execution count
- Step success/failure rates
- Average execution duration
- Cost per execution

**Infrastructure Metrics:**
- ECS task health
- API Gateway latency
- DynamoDB throttling
- S3 request rates

### Logging Strategy

```
Log Levels:
- ERROR: Failures requiring attention
- WARN: Degraded performance, retries
- INFO: Major operations (create, deploy)
- DEBUG: Detailed execution traces
- TRACE: Full request/response data
```

**Structured Logging Format:**
```json
{
  "timestamp": "2025-11-08T10:30:00Z",
  "level": "INFO",
  "component": "workflow_compiler",
  "workflow_id": "550e8400-...",
  "operation": "generate_code",
  "duration_ms": 234,
  "status": "success",
  "metadata": { ... }
}
```

---

## Error Handling Strategy

### Error Categories

**1. User Errors (Recoverable)**
- Invalid description
- Unsupported tool combination
- Missing credentials
→ Clear error message, prompt for correction

**2. Validation Errors (Preventable)**
- Syntax errors in generated code
- Missing dependencies
- Configuration conflicts
→ Fail fast with detailed diagnostics

**3. External Service Errors (Retryable)**
- API timeouts
- Rate limiting
- Network failures
→ Exponential backoff, retry with circuit breaker

**4. Infrastructure Errors (Alert)**
- AWS service failures
- Temporal connection loss
- Deployment failures
→ Log, alert, fallback if possible

### Retry Policies

```rust
pub struct RetryPolicy {
    pub max_attempts: u32,          // Default: 3
    pub initial_interval_ms: u64,   // Default: 1000
    pub backoff_coefficient: f64,   // Default: 2.0
    pub max_interval_ms: u64,       // Default: 60000
}

// Example progression:
// Attempt 1: wait 1s
// Attempt 2: wait 2s
// Attempt 3: wait 4s
// Attempt 4: wait 8s
```

---

## Future Enhancements

### Short-Term (Post-POC)
- Visual workflow builder (drag-and-drop UI)
- More pre-built tools (GitHub, Jira, email)
- Advanced error recovery patterns
- Cost optimization recommendations

### Medium-Term
- Multi-language support (generate TypeScript, Go workers)
- Workflow versioning and migration
- A/B testing for workflow variants
- Custom approval UIs per workflow

### Long-Term
- AI-powered workflow optimization
- Automatic scaling recommendations
- Workflow marketplace
- Natural language workflow debugging

---

## Conclusion

This system design provides a comprehensive blueprint for building a production-grade AI workflow generator. Key design principles:

1. **Separation of Concerns**: Rust for infrastructure, Python for runtime
2. **Security First**: Credentials encrypted, containers isolated, audit trails
3. **Developer Experience**: < 10 minutes to production, zero manual coding
4. **Reliability**: Dry-runs before deployment, comprehensive error handling
5. **Scalability**: Cloud-native architecture, stateless components

The modular design allows for incremental development and testing, with clear interfaces between components enabling parallel development where possible.
