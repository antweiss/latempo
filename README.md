# Latempo - AI Workflow Generator

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Transform natural language descriptions into production-ready, executable [Temporal](https://temporal.io) workflows with AI agents, automatically deployed to AWS.

## What is Latempo?

Latempo is a Rust-based CLI tool that uses AI to generate complete workflow automation systems from simple natural language descriptions. In under 10 minutes, go from "When a Slack message arrives..." to a fully deployed, production-ready workflow running on AWS.

### Core Features

- **AI-Powered Analysis**: Uses Claude to understand workflow requirements from natural language
- **Code Generation**: Generates Python Temporal workflows with proper error handling and retry logic
- **Secure Credential Management**: Stores all credentials in AWS Secrets Manager with KMS encryption
- **Dry-Run Testing**: Validates workflows with mock data before deployment
- **One-Command Deployment**: Automatically deploys to AWS ECS Fargate with API Gateway webhooks
- **Human-in-the-Loop Approvals**: Built-in approval workflows for sensitive operations

## Quick Start

### Prerequisites

- Rust 1.75+ (or stable)
- Python 3.11+
- AWS account with appropriate permissions
- Temporal Cloud account (or self-hosted Temporal)
- Anthropic API key

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/latempo.git
cd latempo

# Configure environment
cp .env.example .env
# Edit .env and add your API keys and AWS credentials

# Build the project
cargo build --release

# Install the CLI
cargo install --path .
```

### Basic Usage

```bash
# Create a new workflow interactively
latempo create

# Create with a description
latempo create "When a Slack message arrives on #travel, extract details, search flights, and book if approved"

# List all workflows
latempo list

# Test a workflow
latempo test <workflow-id>

# Deploy a workflow
latempo deploy <workflow-id>

# Show workflow details
latempo show <workflow-id>

# Delete a workflow
latempo delete <workflow-id>
```

## How It Works

Latempo follows a six-phase workflow:

1. **Analysis**: AI analyzes your description and identifies required tools, credentials, and approval points
2. **Credential Gathering**: Interactively collects and validates credentials, stores them securely in AWS Secrets Manager
3. **Compilation**: Converts requirements into an internal DSL (WorkflowSpec)
4. **Code Generation**: Generates Python code from templates (workflow, worker, Dockerfile)
5. **Dry-Run**: Tests the workflow with mock data in an isolated environment
6. **Deployment**: Packages and deploys to AWS (ECS, S3, API Gateway, DynamoDB)

## Project Status

**Current Phase**: Phase 1 Complete ✅

- ✅ Project structure and configuration
- ✅ Tool registry with 6 default tools
- ✅ CLI with all commands (stubbed)
- ✅ Error handling and logging
- ⏳ AI analysis and code generation (Phase 2)
- ⏳ Testing infrastructure (Phase 3)
- ⏳ AWS deployment (Phase 4)

## Architecture

```
┌──────────────┐
│     CLI      │  Rust-based command-line interface
└──────┬───────┘
       │
┌──────▼───────┐
│   Analyzer   │  AI-powered requirement extraction
└──────┬───────┘
       │
┌──────▼───────┐
│   Compiler   │  DSL compilation
└──────┬───────┘
       │
┌──────▼───────┐
│  Generator   │  Python code generation (Tera templates)
└──────┬───────┘
       │
┌──────▼───────┐
│  Dry-Runner  │  Validation with mock data
└──────┬───────┘
       │
┌──────▼───────┐
│   Deployer   │  AWS deployment (ECS, S3, DynamoDB)
└──────────────┘
```

## Available Tools

Latempo ships with 6 pre-built tools:

### Messaging
- **slack_send_message**: Send messages to Slack channels

### AI
- **run_claude_agent**: Execute Claude AI agents with tool calling
- **run_openai_agent**: Execute OpenAI ChatGPT agents

### Travel
- **search_flights**: Search for flights using Amadeus API
- **book_flight**: Book flights using Amadeus API

### Utilities
- **http_request**: Make generic HTTP requests to any API

More tools coming in future phases!

## Example Workflows

### Travel Booking
```
When a Slack message arrives on #travel-requests, extract the origin,
destination, and date. Search for flights, find the cheapest option,
request approval, and book if approved.
```

### Customer Support
```
When a support ticket is created, categorize it using AI, search the
knowledge base for relevant articles, and draft a response. Request
human review before sending.
```

### Invoice Processing
```
When an invoice PDF is uploaded to S3, extract the data using AI,
validate against business rules, and route for approval based on amount.
```

## Documentation

- [System Design](docs/SYSTEM_DESIGN.md) - Comprehensive architecture documentation
- [Implementation Plan](docs/IMPLEMENTATION_PLAN.md) - Detailed development roadmap
- [Complete Specification](complete-workflow-spec.md) - Full project specification

## Development

### Building from Source

```bash
cargo build
```

### Running Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# All tests
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings
```

## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to our repository.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Temporal](https://temporal.io) for workflow orchestration
- Powered by [Anthropic Claude](https://anthropic.com) for AI analysis
- Deployed on [AWS](https://aws.amazon.com)

## Roadmap

### Phase 1: Foundation (Weeks 1-2) ✅
- Project setup and configuration
- Tool registry
- CLI structure

### Phase 2: Core Logic (Weeks 3-5) ⏳
- AI-powered requirement analysis
- DSL compilation
- Python code generation

### Phase 3: Testing Infrastructure (Week 6)
- Dry-run execution with mocks
- Cost estimation
- Trace collection

### Phase 4: Deployment (Weeks 7-8)
- AWS deployment automation
- Credential management
- Workflow lifecycle management

### Future Enhancements
- Visual workflow builder
- More pre-built tools
- Multi-language support (TypeScript, Go)
- Workflow marketplace

---

**Built with ❤️ using Rust and AI**
