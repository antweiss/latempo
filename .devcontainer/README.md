# GitHub Codespaces Configuration

This directory contains the development container configuration for GitHub Codespaces and VS Code Remote - Containers.

## Features

The development environment includes:

### Languages & Runtimes
- **Rust 1.75+** (stable) with rustfmt, clippy, and rust-analyzer
- **Python 3.11+** with pip, black, pylint, and pytest
- **Node.js LTS** for any web tooling needs

### Tools & CLI
- **AWS CLI** - For interacting with AWS services
- **GitHub CLI (gh)** - For GitHub operations
- **Docker** - For container operations
- **Cargo Tools**:
  - `cargo-audit` - Security vulnerability scanning
  - `cargo-tarpaulin` - Code coverage
  - `cargo-watch` - File watcher for auto-rebuilding

### VS Code Extensions

Pre-installed extensions include:
- `rust-analyzer` - Rust language server
- `even-better-toml` - TOML file support
- `crates` - Cargo.toml dependency management
- `vscode-lldb` - Rust debugging
- `python` & `pylance` - Python development
- `aws-toolkit-vscode` - AWS integration
- `copilot` & `copilot-chat` - AI assistance
- `vscode-docker` - Docker support

## Quick Start

### Using GitHub Codespaces

1. Click the **Code** button on the repository
2. Select **Codespaces** tab
3. Click **Create codespace on [branch]**
4. Wait for the environment to build (first time takes ~5-10 minutes)

### Using VS Code Locally

1. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension
2. Open the repository in VS Code
3. Press `F1` and select **Remote-Containers: Reopen in Container**
4. Wait for the container to build

## Environment Configuration

### Environment Variables

The container automatically loads environment variables from:
- `.env` file in the workspace root
- Local environment variables (for sensitive data)

Required environment variables:
```bash
ANTHROPIC_API_KEY=your-api-key-here
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=your-access-key-id
AWS_SECRET_ACCESS_KEY=your-secret-access-key
```

### Setting Secrets in Codespaces

For sensitive values like API keys:

1. Go to repository **Settings** → **Secrets and variables** → **Codespaces**
2. Add secrets:
   - `ANTHROPIC_API_KEY`
   - `AWS_ACCESS_KEY_ID`
   - `AWS_SECRET_ACCESS_KEY`
3. Restart your Codespace

These will be available as environment variables in the container.

## Post-Create Setup

The `setup.sh` script automatically:
- Installs Rust components (rustfmt, clippy, rust-analyzer)
- Installs Cargo tools (cargo-audit, cargo-tarpaulin, cargo-watch)
- Sets up Python with necessary packages
- Creates a default `.env` file if needed
- Builds the project to cache dependencies
- Runs initial tests

## Development Workflow

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test --all-features

# Run with watch mode
cargo watch -x test

# Check code style
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all
```

### Running the CLI

```bash
# Show help
cargo run -- --help

# Create a workflow
cargo run -- create

# Test a workflow
cargo run -- test workflow_id

# Deploy a workflow
cargo run -- deploy workflow_id
```

### Security & Quality

```bash
# Security audit
cargo audit

# Code coverage
cargo tarpaulin --out Html --output-dir coverage

# Documentation
cargo doc --no-deps --open
```

## Port Forwarding

The following ports are automatically forwarded:
- **8080** - API Server
- **3000** - Development Server

You can access these via the **Ports** panel in VS Code.

## Customization

### Modifying the Container

To customize the development environment:

1. Edit `.devcontainer/devcontainer.json`
2. Rebuild the container:
   - Press `F1`
   - Select **Remote-Containers: Rebuild Container**

### Adding VS Code Extensions

Add extension IDs to the `extensions` array in `devcontainer.json`:

```json
"customizations": {
  "vscode": {
    "extensions": [
      "your-extension-id"
    ]
  }
}
```

### Adding System Dependencies

Edit `setup.sh` to add installation commands:

```bash
# Example: Install additional tools
sudo apt-get update
sudo apt-get install -y your-package
```

## Troubleshooting

### Container Build Fails

1. Check the build logs in the terminal
2. Ensure you have sufficient resources (4GB RAM minimum)
3. Try rebuilding: **Remote-Containers: Rebuild Container**

### Tests Fail

- Most tests require AWS services to be configured
- Set up LocalStack for local AWS service emulation
- Or configure real AWS credentials in `.env`

### Slow Performance

- Codespaces: Upgrade to a more powerful machine type
- Local: Allocate more resources to Docker
- Consider using volume mounts instead of bind mounts

## Resources

- [Codespaces Documentation](https://docs.github.com/en/codespaces)
- [Dev Containers Specification](https://containers.dev/)
- [VS Code Remote Development](https://code.visualstudio.com/docs/remote/remote-overview)

## Support

For issues with the development container:
1. Check this README
2. Review the logs in the terminal
3. Open an issue in the repository
