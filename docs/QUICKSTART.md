# Quick Start Guide

## Installation

### Prerequisites

1. **Install Rust** (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. **Install Protocol Buffers Compiler**:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y protobuf-compiler libprotobuf-dev build-essential

# Fedora
sudo dnf install protobuf-compiler protobuf-devel gcc

# Arch Linux
sudo pacman -S protobuf
```

3. **Clone the repository**:
```bash
git clone https://github.com/LucasDiasJorge/Blockchain-GRPC.git
cd Blockchain-GRPC
```

## Running the Server

### Quick Start

```bash
# Build and run
cargo run --release
```

The server will start on `0.0.0.0:50051`

### Using Setup Script

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Run setup (installs dependencies and builds)
./scripts/setup.sh

# Run the server
cargo run --release
```

## Testing the API

### Using the Example Client

```bash
# Run the example client
cargo run --example client_example
```

This will:
1. Create multiple graphs (transactions, identity, assets)
2. Add blocks with data
3. Add blocks with cross-references
4. Verify graph integrity
5. Perform cross-validation

### Using grpcurl

Install grpcurl:
```bash
# Ubuntu/Debian
sudo apt install grpcurl

# Or via go
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
```

Test the API:
```bash
# Run test script
chmod +x scripts/test_grpc.sh
./scripts/test_grpc.sh
```

Or manually:

```bash
# List services
grpcurl -plaintext localhost:50051 list

# Create a graph
grpcurl -plaintext -d '{
  "graph_id": "my_graph",
  "graph_type": 0,
  "description": "My first blockchain"
}' localhost:50051 blockchain.BlockchainService/CreateGraph

# Add a block
grpcurl -plaintext -d '{
  "graph_id": "my_graph",
  "data": "Hello, Blockchain!"
}' localhost:50051 blockchain.BlockchainService/AddBlock

# Get graph info
grpcurl -plaintext -d '{
  "graph_id": "my_graph"
}' localhost:50051 blockchain.BlockchainService/GetGraphInfo
```

## Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_block_creation

# Run integration tests
cargo test --test integration_tests
```

## Configuration

Create or edit `config.json`:

```json
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
```

Options:
- `default_difficulty`: Proof of work difficulty (higher = more secure but slower)
- `max_block_size`: Maximum block data size in bytes
- `data_dir`: Where blockchain data is stored

## Docker Deployment

### Build and Run

```bash
# Build image
docker build -t blockchain-grpc .

# Run container
docker run -p 50051:50051 -v blockchain-data:/app/data blockchain-grpc
```

### Using Docker Compose

```bash
# Start service
docker-compose up -d

# View logs
docker-compose logs -f

# Stop service
docker-compose down
```

## Development Workflow

### Auto-reload on Changes

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Fix automatically
cargo clippy --fix
```

## Common Operations

### Creating Different Graph Types

```bash
# Transaction graph
grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "graph_type": 0,
  "description": "Financial transactions"
}' localhost:50051 blockchain.BlockchainService/CreateGraph

# Identity graph
grpcurl -plaintext -d '{
  "graph_id": "identity",
  "graph_type": 1,
  "description": "User identities"
}' localhost:50051 blockchain.BlockchainService/CreateGraph

# Asset graph
grpcurl -plaintext -d '{
  "graph_id": "assets",
  "graph_type": 2,
  "description": "Asset ownership"
}' localhost:50051 blockchain.BlockchainService/CreateGraph
```

### Adding Blocks with Cross-References

```bash
# Add block to transactions
grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "data": "{\"from\":\"Alice\",\"to\":\"Bob\",\"amount\":100}"
}' localhost:50051 blockchain.BlockchainService/AddBlock

# Get the hash from response, then add to identity with cross-reference
grpcurl -plaintext -d '{
  "graph_id": "identity",
  "data": "{\"user\":\"Alice\",\"verified\":true}",
  "cross_references": ["<hash_from_previous_block>"]
}' localhost:50051 blockchain.BlockchainService/AddBlock
```

### Validating the Network

```bash
# Verify single graph
grpcurl -plaintext -d '{
  "graph_id": "transactions"
}' localhost:50051 blockchain.BlockchainService/VerifyGraph

# Cross-validate all graphs
grpcurl -plaintext -d '{}' localhost:50051 blockchain.BlockchainService/CrossValidateGraphs
```

## Monitoring

### Check Logs

```bash
# Run with detailed logging
RUST_LOG=debug cargo run

# Different log levels
RUST_LOG=info cargo run    # Info and above
RUST_LOG=warn cargo run    # Warnings and errors only
RUST_LOG=error cargo run   # Errors only
```

### View Data Directory

```bash
ls -la data/blockchain/
```

## Troubleshooting

### Port Already in Use

```bash
# Kill process on port 50051
sudo lsof -ti:50051 | xargs kill -9

# Or change port in config.json
```

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Permission Denied

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Fix data directory permissions
sudo chown -R $USER:$USER data/
```

### RocksDB Issues

```bash
# Remove corrupted data
rm -rf data/blockchain/

# Restart server (will create fresh database)
cargo run --release
```

## Next Steps

1. **Read the [Architecture Documentation](docs/ARCHITECTURE.md)** to understand the system design
2. **Check the [API Reference](docs/API.md)** for detailed endpoint documentation
3. **Integrate with C#** - see README for C# client examples
4. **Customize graph types** - modify `proto/blockchain.proto` for your needs

## Getting Help

- Open an issue on GitHub
- Check existing issues for solutions
- Review the documentation in `docs/`

## Performance Tips

1. **Increase difficulty** for production environments (3-4 recommended)
2. **Use release builds** for better performance
3. **Adjust RocksDB settings** for your hardware
4. **Enable LTO** in release profile (already configured)
5. **Monitor memory usage** with multiple graphs

## Security Checklist

- [ ] Change default port if exposed to internet
- [ ] Add authentication (coming in future release)
- [ ] Use TLS for production (configure in Tonic)
- [ ] Limit block size to prevent DoS
- [ ] Monitor disk usage
- [ ] Regular backups of data directory

---

Happy blockchain building! 🚀
