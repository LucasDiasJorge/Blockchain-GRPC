# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Authentication and authorization system
- REST API alongside gRPC
- Prometheus metrics integration
- Basic smart contract support
- Distributed consensus (Raft/PBFT)
- CLI client for testing
- Web dashboard for visualization

## [0.1.0] - 2025-10-21

### Added
- Initial implementation of multi-graph blockchain architecture
- Core domain entities: Block, BlockchainGraph, Transaction
- Repository pattern with RocksDB persistence
- gRPC server with comprehensive API
- Proof of Work consensus mechanism
- Cross-reference system between graphs
- Cross-validation of all graphs in network
- Five specialized graph types (Transaction, Identity, Asset, Audit, Custom)
- In-memory caching with RwLock for performance
- Binary serialization with Bincode
- SHA-256 cryptographic hashing
- Complete test suite (unit and integration tests)
- Docker support with multi-stage builds
- Comprehensive documentation (README, Architecture, API, Quick Start)
- Example client implementation
- Development scripts (build, setup, test)
- Clean Architecture with SOLID principles
- Design patterns: Repository, Strategy, Factory, Adapter

### Features
- **Multi-Graph Architecture**: Multiple independent blockchains
- **Cross-Validation**: Graphs verify each other's integrity
- **Persistence**: Data survives restarts with RocksDB
- **High Performance**: Optimized with caching and binary serialization
- **Type Safety**: Rust's type system prevents common errors
- **Async/Await**: Non-blocking operations throughout
- **Configurable**: JSON configuration file
- **Testable**: Comprehensive test coverage

### API Endpoints
- `CreateGraph`: Create new blockchain graph
- `AddBlock`: Add block to graph with optional cross-references
- `GetBlock`: Retrieve block by hash
- `GetLatestBlock`: Get most recent block
- `GetGraphInfo`: Get graph metadata and statistics
- `VerifyGraph`: Validate graph integrity
- `CrossValidateGraphs`: Validate entire network
- `ListGraphs`: List all available graphs
- `GetBlockRange`: Retrieve range of blocks

### Technical Details
- Rust 2021 Edition
- Tonic for gRPC
- RocksDB for persistence
- Tokio for async runtime
- SHA-256 for hashing
- Bincode for serialization

### Documentation
- Complete README in Portuguese
- Architecture documentation with diagrams
- API reference with examples in C# and Python
- Quick start guide
- Contributing guidelines
- Docker deployment guide

[Unreleased]: https://github.com/LucasDiasJorge/Blockchain-GRPC/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/LucasDiasJorge/Blockchain-GRPC/releases/tag/v0.1.0
