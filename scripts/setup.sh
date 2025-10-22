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

echo "🛠️  Installing cargo tools..."
# Install protobuf compiler and system build deps
echo "📦 Installing system dependencies (protobuf, build-essential, clang, libclang, pkg-config, zlib, openssl)..."

if [ -f /etc/debian_version ]; then
  # Debian/Ubuntu
  sudo apt-get update
  sudo apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    build-essential \
    clang \
    libclang-dev \
    pkg-config \
    zlib1g-dev \
    libssl-dev \
    ca-certificates \
    curl
elif [ -f /etc/redhat-release ]; then
  # Fedora/RHEL
  sudo dnf install -y protobuf-compiler protobuf-devel gcc clang clang-devel pkgconfig zlib-devel openssl-devel
else
  echo "⚠️  Please install protobuf-compiler and development tools manually for your OS"
fi

# Install useful cargo tools
echo "🛠️  Installing cargo tools..."
cargo install cargo-watch || true
cargo install cargo-expand || true

# Install grpcurl (prefer apt or go install)
echo "🌐 Installing grpcurl client..."
if command -v grpcurl &> /dev/null; then
  echo "grpcurl already installed"
else
  if [ -f /etc/debian_version ]; then
    # Debian/Ubuntu: grpcurl available via apt in newer repos, otherwise use go install
    sudo apt-get install -y grpcurl || {
      # fallback to go install
      if command -v go &> /dev/null; then
        echo "Installing grpcurl via 'go install'"
        go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
        export PATH=$PATH:$(go env GOPATH)/bin
      else
        echo "⚠️  'go' not installed. Install Go or grpcurl manually."
      fi
    }
  else
    if command -v go &> /dev/null; then
      echo "Installing grpcurl via 'go install'"
      go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
      export PATH=$PATH:$(go env GOPATH)/bin
    else
      echo "⚠️  Please install grpcurl or Go (for go install) manually."
    fi
  fi
fi

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
