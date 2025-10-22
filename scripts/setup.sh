#!/bin/bash

# Development setup script

set -e

echo "🔧 Setting up development environment..."

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Install protobuf compiler
echo "📦 Installing dependencies..."

if [ -f /etc/debian_version ]; then
    # Debian/Ubuntu
    sudo apt-get update
    sudo apt-get install -y protobuf-compiler libprotobuf-dev build-essential
elif [ -f /etc/redhat-release ]; then
    # Fedora/RHEL
    sudo dnf install -y protobuf-compiler protobuf-devel gcc
else
    echo "⚠️  Please install protobuf-compiler manually for your OS"
fi

# Install useful cargo tools
echo "🛠️  Installing cargo tools..."
cargo install cargo-watch
cargo install cargo-expand
cargo install grpcurl

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p data/blockchain
mkdir -p logs

# Generate initial config if it doesn't exist
if [ ! -f config.json ]; then
    echo "⚙️  Creating default config..."
    cat > config.json << 'EOF'
{
  "server": {
    "host": "0.0.0.0",
    "port": 50051
  },
  "blockchain": {
    "default_difficulty": 2,
    "max_block_size": 1048576
  },
  "storage": {
    "data_dir": "./data/blockchain"
  }
}
EOF
fi

# Build the project
echo "🔨 Building project..."
cargo build

echo "✅ Development environment ready!"
echo ""
echo "Quick start commands:"
echo "  cargo run              # Run the server"
echo "  cargo test             # Run tests"
echo "  cargo watch -x run     # Auto-reload on changes"
echo "  ./scripts/test_grpc.sh # Test gRPC endpoints"
