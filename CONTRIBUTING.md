# Contributing to Latempo

Thank you for your interest in contributing to Latempo! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [CI/CD Pipeline](#cicd-pipeline)

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Prioritize the project's best interests
- Be patient and understanding

## Getting Started

### Prerequisites

- **Rust:** 1.75+ or stable
- **Python:** 3.11+ (for Phase 3+)
- **Git:** Latest version
- **Editor:** VS Code, IntelliJ IDEA, or your favorite Rust IDE

### Setup Development Environment

1. **Fork and Clone:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/latempo.git
   cd latempo
   ```

2. **Install Dependencies:**
   ```bash
   # Rust is required
   rustup update stable

   # Install development tools
   cargo install cargo-watch cargo-tarpaulin cargo-audit
   ```

3. **Configure Environment:**
   ```bash
   cp .env.example .env
   # Edit .env with your API keys (for testing)
   ```

4. **Verify Setup:**
   ```bash
   cargo build
   cargo test
   ```

### Project Structure

```
latempo/
├── src/               # Rust source code
│   ├── cli/          # Command-line interface
│   ├── tools/        # Tool registry and definitions
│   ├── analyzer/     # AI analysis (Phase 2)
│   ├── compiler/     # DSL and code generation (Phase 2)
│   ├── testing/      # Dry-run infrastructure (Phase 3)
│   └── deployment/   # AWS deployment (Phase 4)
├── python_runtime/   # Python workflow library (Phase 3+)
├── templates/        # Tera templates for code generation
├── tests/            # Integration and unit tests
├── docs/             # Documentation
└── infrastructure/   # Terraform configurations
```

## Development Workflow

### Branch Strategy

- **main:** Stable, production-ready code
- **develop:** Integration branch for features
- **feature/xxx:** Feature branches
- **fix/xxx:** Bug fix branches
- **claude/xxx:** AI-assisted development branches

### Creating a Feature Branch

```bash
# Start from develop
git checkout develop
git pull origin develop

# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and commit
git add .
git commit -m "feat: add your feature"

# Push to your fork
git push origin feature/your-feature-name
```

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Test additions or modifications
- `chore:` Build process or tooling changes
- `ci:` CI/CD changes

**Examples:**
```bash
feat(tools): add GitHub API tool
fix(cli): handle empty workflow descriptions
docs(readme): update installation instructions
test(registry): add tests for tool validation
```

## Pull Request Process

### Before Submitting

1. **Update Your Branch:**
   ```bash
   git fetch origin
   git rebase origin/develop
   ```

2. **Run Pre-submission Checks:**
   ```bash
   # Format code
   cargo fmt --all

   # Run linter
   cargo clippy --all-targets --all-features -- -D warnings

   # Run tests
   cargo test --all-features

   # Check documentation
   cargo doc --no-deps --all-features
   ```

3. **Review Your Changes:**
   ```bash
   git diff origin/develop
   ```

### Creating a Pull Request

1. **Push to Your Fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open PR on GitHub:**
   - Go to https://github.com/antweiss/latempo/pulls
   - Click "New Pull Request"
   - Select your branch
   - Fill in the PR template

3. **PR Title Format:**
   ```
   feat: Add GitHub API integration tool
   fix: Handle empty descriptions in create command
   docs: Update CI/CD documentation
   ```

4. **PR Description Should Include:**
   - **What:** Description of changes
   - **Why:** Motivation and context
   - **How:** Implementation approach
   - **Testing:** How you tested the changes
   - **Screenshots:** For UI changes
   - **Breaking Changes:** If any
   - **Related Issues:** Closes #123

### PR Review Process

1. **Automated Checks:**
   - CI pipeline must pass
   - Code coverage must not decrease
   - Security scan must pass
   - No merge conflicts

2. **Code Review:**
   - At least one approval required
   - Address all review comments
   - Keep discussion focused and respectful

3. **Merge:**
   - Squash and merge (default)
   - Maintainer will merge when ready

## Coding Standards

### Rust Style Guide

1. **Follow Rust Guidelines:**
   - [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
   - Use `cargo fmt` for formatting
   - Use `cargo clippy` for linting

2. **Naming Conventions:**
   ```rust
   // Types: PascalCase
   struct WorkflowSpec { }
   enum ToolCategory { }

   // Functions, variables: snake_case
   fn create_workflow() { }
   let workflow_id = "abc";

   // Constants: SCREAMING_SNAKE_CASE
   const MAX_RETRIES: u32 = 3;

   // Modules: snake_case
   mod tool_registry;
   ```

3. **Error Handling:**
   ```rust
   // Use Result for fallible operations
   fn validate_tool(&self, name: &str) -> Result<()> {
       // ...
   }

   // Use custom error types
   return Err(WorkflowError::ToolNotFound(name.to_string()));

   // Provide context
   .map_err(|e| WorkflowError::config(format!("Failed to load: {}", e)))?
   ```

4. **Documentation:**
   ```rust
   /// Creates a new workflow from a natural language description.
   ///
   /// # Arguments
   ///
   /// * `description` - Natural language workflow description
   /// * `settings` - Application settings
   ///
   /// # Returns
   ///
   /// Returns `Ok(WorkflowId)` on success, or `WorkflowError` on failure.
   ///
   /// # Examples
   ///
   /// ```
   /// let description = "When a Slack message arrives...";
   /// let workflow_id = create_workflow(description, &settings).await?;
   /// ```
   pub async fn create_workflow(
       description: &str,
       settings: &Settings,
   ) -> Result<WorkflowId> {
       // ...
   }
   ```

### Code Organization

1. **Module Structure:**
   ```rust
   // mod.rs exports public API
   pub mod models;
   pub mod registry;
   pub use models::{ToolDefinition, ToolCategory};
   pub use registry::ToolRegistry;
   ```

2. **Imports:**
   ```rust
   // Standard library
   use std::collections::HashMap;
   use std::sync::Arc;

   // External crates
   use serde::{Deserialize, Serialize};
   use tokio::sync::RwLock;

   // Internal modules
   use crate::utils::{Result, WorkflowError};
   ```

3. **Visibility:**
   ```rust
   // Public API - carefully designed
   pub struct ToolRegistry { }

   // Internal - implementation details
   struct ToolCache { }

   // Public but not re-exported
   pub(crate) fn internal_helper() { }
   ```

## Testing Guidelines

### Test Organization

```
tests/
├── unit/              # Unit tests (in src files)
├── integration/       # Integration tests
└── fixtures/          # Test data and helpers
```

### Writing Tests

1. **Unit Tests:**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_tool_registry_creation() {
           let registry = ToolRegistry::new();
           assert!(!registry.list_tools().is_empty());
       }

       #[test]
       fn test_tool_validation() {
           let registry = ToolRegistry::new();
           assert!(registry.validate_tool("slack_send_message").is_ok());
           assert!(registry.validate_tool("nonexistent").is_err());
       }
   }
   ```

2. **Async Tests:**
   ```rust
   #[tokio::test]
   async fn test_async_operation() {
       let result = async_function().await;
       assert!(result.is_ok());
   }
   ```

3. **Integration Tests:**
   ```rust
   // tests/integration/workflow_creation.rs
   #[tokio::test]
   async fn test_complete_workflow_creation() {
       // Setup
       let settings = Settings::from_env().unwrap();

       // Execute
       let result = create_workflow("test description", &settings).await;

       // Assert
       assert!(result.is_ok());
   }
   ```

### Test Coverage

- **Minimum:** 70% for Phase 1
- **Target:** 80%+ for Phase 2+
- **Critical Paths:** 100%

Run coverage:
```bash
cargo tarpaulin --all-features --workspace --out Html
open tarpaulin-report.html
```

## Documentation

### Types of Documentation

1. **Code Comments:**
   - Explain "why", not "what"
   - Document non-obvious decisions
   - Reference issues/tickets

2. **API Documentation:**
   - All public items must be documented
   - Include examples
   - Document errors and edge cases

3. **User Documentation:**
   - README.md - Project overview
   - docs/ - Detailed guides
   - CHANGELOG.md - Version history

### Documentation Standards

- Write in Markdown
- Use code blocks with syntax highlighting
- Include diagrams where helpful
- Keep it up to date

Generate docs:
```bash
cargo doc --no-deps --open
```

## CI/CD Pipeline

Our CI/CD pipeline runs automatically on every push and PR. See [CI/CD Documentation](docs/CI_CD.md) for details.

### Required Checks

All PRs must pass:
- ✅ Format check (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Build (debug and release)
- ✅ Tests (all features)
- ✅ Security audit
- ✅ Documentation build

### Local Pre-flight Check

Run this before pushing:
```bash
#!/bin/bash
set -e

echo "🔍 Running pre-flight checks..."

echo "📝 Formatting..."
cargo fmt --all -- --check

echo "📎 Linting..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🔨 Building..."
cargo build --all-features

echo "🧪 Testing..."
cargo test --all-features

echo "📚 Documentation..."
cargo doc --no-deps --all-features

echo "✅ All checks passed!"
```

Save as `scripts/preflight.sh` and run before commits.

## Getting Help

### Communication Channels

- **Issues:** Bug reports and feature requests
- **Discussions:** Questions and general discussion
- **Pull Requests:** Code review and collaboration

### Asking Questions

Good question format:
```
**Context:** I'm trying to [what you're doing]

**Problem:** [What's not working]

**What I've Tried:**
1. [Thing 1]
2. [Thing 2]

**Environment:**
- OS: [Ubuntu 22.04]
- Rust: [1.75.0]
- Version: [v0.1.0]

**Code:**
```rust
// Minimal reproducible example
```
```

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- GitHub contributors page
- Release notes (for significant contributions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Thank You!

Every contribution, no matter how small, makes a difference. Thank you for helping make Latempo better!

---

**Questions?** Open an issue or discussion on GitHub.

**Found a Bug?** Please report it! We appreciate your help.

**Have an Idea?** We'd love to hear it! Open a feature request.
