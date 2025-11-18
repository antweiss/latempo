#!/bin/bash
set -e

echo "🚀 Setting up Latempo development environment..."

# Update Rust and install components
echo "📦 Installing Rust components..."
rustup update stable
rustup component add rustfmt clippy rust-analyzer

# Install useful Cargo tools
echo "🔧 Installing Cargo tools..."
cargo install --locked cargo-audit || echo "cargo-audit already installed"
cargo install --locked cargo-tarpaulin || echo "cargo-tarpaulin already installed"
cargo install --locked cargo-watch || echo "cargo-watch already installed"

# Set up Python environment
echo "🐍 Setting up Python environment..."
python3 -m pip install --upgrade pip
python3 -m pip install black pylint pytest temporalio

# Create .env.example if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from example..."
    if [ -f .env.example ]; then
        cp .env.example .env
        echo "✅ Created .env from .env.example"
    else
        cat > .env <<EOF
# Anthropic API Configuration
ANTHROPIC_API_KEY=your-api-key-here
ANTHROPIC_API_BASE_URL=https://api.anthropic.com/v1

# AWS Configuration
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=your-access-key-id
AWS_SECRET_ACCESS_KEY=your-secret-access-key

# DynamoDB Configuration
DYNAMODB_TABLE_NAME=latempo-workflows
DYNAMODB_ENDPOINT=http://localhost:8000

# Secrets Manager Configuration
SECRETS_MANAGER_PREFIX=latempo

# Temporal Configuration
TEMPORAL_HOST=localhost:7233
TEMPORAL_NAMESPACE=default
TEMPORAL_TASK_QUEUE=latempo-workflows

# S3 Configuration
S3_BUCKET_NAME=latempo-artifacts
S3_ENDPOINT=http://localhost:9000

# Application Configuration
APP_ENV=development
LOG_LEVEL=info
EOF
        echo "✅ Created default .env file"
    fi
fi

# Build the project to cache dependencies
echo "🏗️  Building project and caching dependencies..."
cargo build

# Run tests to ensure everything works
echo "🧪 Running tests..."
cargo test --all-features || echo "⚠️  Some tests failed - this is expected if AWS services are not configured"

echo ""
echo "✅ Latempo development environment setup complete!"
echo ""
echo "📚 Quick Start:"
echo "  cargo build          - Build the project"
echo "  cargo test           - Run tests"
echo "  cargo run -- --help  - Run CLI with help"
echo "  cargo watch -x test  - Watch mode for tests"
echo ""
echo "🔍 Useful commands:"
echo "  cargo clippy         - Lint code"
echo "  cargo fmt            - Format code"
echo "  cargo audit          - Security audit"
echo "  cargo tarpaulin      - Code coverage"
echo ""
echo "📖 Documentation:"
echo "  docs/SYSTEM_DESIGN.md       - System architecture"
echo "  docs/IMPLEMENTATION_PLAN.md - Implementation roadmap"
echo "  docs/CI_CD.md              - CI/CD pipeline"
echo "  CONTRIBUTING.md            - Contribution guidelines"
echo ""
