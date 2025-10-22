#!/bin/bash

# Build script for the blockchain project

set -e

echo "🔨 Building Blockchain-GRPC..."

# Check if protoc is installed
if ! command -v protoc &> /dev/null; then
    echo "❌ protoc not found. Please install Protocol Buffers compiler."
    echo "   Ubuntu/Debian: sudo apt install protobuf-compiler"
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Format code
echo "📝 Formatting code..."
cargo fmt

# Run clippy
echo "🔍 Running clippy..."
cargo clippy -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test

# Build release
echo "🚀 Building release..."
cargo build --release

echo "✅ Build completed successfully!"
echo "📦 Binary location: target/release/blockchain-grpc"
echo ""
echo "To run the server:"
echo "  cargo run --release"
echo ""
echo "Or directly:"
echo "  ./target/release/blockchain-grpc"
