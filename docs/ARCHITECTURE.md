# Architecture Documentation

## Overview

This project implements an enterprise-grade blockchain system using a **multi-graph architecture**, where multiple independent blockchains coexist and cross-validate each other.

## Core Concepts

### 1. Multi-Graph Architecture

Instead of a single blockchain, the system maintains multiple specialized blockchains (graphs), each responsible for different types of data:

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Transaction    │────▶│    Identity     │────▶│     Asset       │
│     Graph       │     │      Graph      │     │     Graph       │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        │                       │                        │
        │                       │                        │
        └───────────────────────┼────────────────────────┘
                                │
                        Cross-Validation
```

Each graph:
- Maintains its own chain of blocks
- Has a specific responsibility (Transaction, Identity, Asset, Audit, Custom)
- Can reference blocks from other graphs
- Validates independently but participates in cross-validation

### 2. Cross-Reference System

Blocks can include `cross_references` to blocks in other graphs, creating a network of interconnected blockchains:

```rust
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    // ... other fields
    pub cross_references: Vec<String>, // References to other graphs
}
```

This enables:
- **Data Correlation**: Link related data across different graphs
- **Audit Trail**: Track dependencies between different data types
- **Network Integrity**: Tampering with one graph affects cross-references

### 3. Clean Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│                    Presentation Layer                    │
│                    (gRPC Server)                         │
└───────────────────────┬─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│                  Application Layer                       │
│              (Services & Use Cases)                      │
│  - BlockchainServiceImpl                                 │
│  - ValidationService                                     │
└───────────────────────┬─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│                    Domain Layer                          │
│              (Business Logic & Entities)                 │
│  - Block                                                 │
│  - BlockchainGraph                                       │
│  - Traits (Repository, Validation)                       │
└───────────────────────┬─────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────┐
│                 Infrastructure Layer                     │
│            (External Implementations)                    │
│  - RocksDB Adapter                                       │
│  - Repository Implementation                             │
└─────────────────────────────────────────────────────────┘
```

## Design Patterns

### 1. Repository Pattern

Abstracts data persistence:

```rust
#[async_trait]
pub trait BlockchainRepository: Send + Sync {
    async fn save_block(&self, graph_id: &str, block: &Block) 
        -> Result<(), Box<dyn Error>>;
    async fn get_block(&self, graph_id: &str, hash: &str) 
        -> Result<Option<Block>, Box<dyn Error>>;
    // ... more methods
}
```

**Benefits**:
- Easy to swap storage backends
- Testable with mock implementations
- Single source of truth for data access

### 2. Strategy Pattern

Multiple validation strategies:

```rust
#[async_trait]
pub trait ValidationStrategy: Send + Sync {
    async fn validate(&self, graph: &BlockchainGraph) 
        -> Result<bool, Box<dyn Error>>;
}

// Implementations
struct ChainIntegrityValidator;
struct BlockHashValidator;
struct DifficultyValidator;
```

**Benefits**:
- Add new validation rules without modifying existing code
- Each validator has single responsibility
- Composable validations

### 3. Factory Pattern

Block creation:

```rust
impl Block {
    pub fn new(/* params */) -> Self { /* ... */ }
    pub fn genesis(graph_id: String) -> Self { /* ... */ }
}
```

### 4. Adapter Pattern

RocksDB wrapper:

```rust
pub struct RocksDbAdapter {
    db: Arc<DB>,
}

impl RocksDbAdapter {
    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn Error>>
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>>
}
```

## SOLID Principles

### Single Responsibility Principle (SRP)
- `Block`: Represents a single block
- `BlockchainGraph`: Manages a single blockchain
- `BlockchainRepository`: Handles persistence only
- `ValidationService`: Only validates

### Open/Closed Principle (OCP)
- Extensible through traits (`ValidationStrategy`, `BlockchainRepository`)
- Closed for modification (core logic doesn't change)

### Liskov Substitution Principle (LSP)
- Any `ValidationStrategy` implementation can be used
- Any `BlockchainRepository` implementation works

### Interface Segregation Principle (ISP)
- Small, focused traits
- Clients depend only on what they use

### Dependency Inversion Principle (DIP)
- High-level modules depend on abstractions (traits)
- `BlockchainServiceImpl` depends on `BlockchainRepository` trait, not concrete implementation

## Data Flow

### Adding a Block

```
Client Request
     │
     ▼
gRPC Server (Infrastructure)
     │
     ▼
BlockchainServiceImpl (Application)
     │
     ├──▶ Get Graph from Cache/Repository
     │
     ├──▶ Create new Block (Domain)
     │
     ├──▶ Validate & Mine Block (Domain)
     │
     ├──▶ Add to Graph (Domain)
     │
     └──▶ Persist via Repository (Infrastructure)
     │
     ▼
RocksDB
```

### Cross-Validation

```
Validation Request
     │
     ▼
Load All Graphs
     │
     ├──▶ Validate Each Graph Individually
     │    (Chain integrity, hashes, difficulty)
     │
     └──▶ Validate Cross-References
          │
          └──▶ Check if referenced blocks exist
               in other graphs
```

## Storage Schema

### RocksDB Keys

```
block:{graph_id}:{height:020}              # Block by height
block_hash:{graph_id}:{hash}               # Hash → Height index
latest:{graph_id}                          # Latest block height
graph:{graph_id}                           # Graph metadata
graph_list                                 # List of all graph IDs
```

### Example

```
block:transactions:00000000000000000001   → [Block Binary Data]
block_hash:transactions:abc123...         → [Height: 1]
latest:transactions                       → [Height: 5]
graph:transactions                        → [Graph Metadata]
graph_list                                → ["transactions", "identity", "assets"]
```

## Performance Optimizations

1. **Binary Serialization**: Using Bincode instead of JSON
2. **In-Memory Cache**: RwLock-protected HashMap for hot graphs
3. **Indexed Lookups**: Hash-based indexing for O(1) block retrieval
4. **Async I/O**: Non-blocking operations throughout
5. **LTO**: Link-Time Optimization in release builds

## Security Considerations

1. **SHA-256 Hashing**: Cryptographically secure block hashes
2. **Proof of Work**: Computational cost to add blocks
3. **Cross-Validation**: Multiple graphs verify each other
4. **Immutability**: Blocks cannot be modified once added
5. **Chain Validation**: Previous hash linking prevents tampering

## Integration Pattern with C#

The Rust service acts as the **Infrastructure/Repository** layer, while the C# API provides the **Domain/Application** layer:

```
┌───────────────────────────────────────────────────────┐
│                    C# API (ASP.NET)                   │
│                                                        │
│  Controllers → Domain Services → gRPC Client          │
│  (Business Logic, Validation, DTOs)                   │
└────────────────────────┬──────────────────────────────┘
                         │ gRPC
                         │
┌────────────────────────▼──────────────────────────────┐
│                 Rust Blockchain Service               │
│                                                        │
│  gRPC Server → Application → Domain → RocksDB         │
│  (Data Persistence, Blockchain Logic)                 │
└───────────────────────────────────────────────────────┘
```

The C# API:
- Receives HTTP requests
- Validates business rules
- Transforms DTOs
- Calls Rust service via gRPC
- Returns responses to clients

The Rust service:
- Handles blockchain operations
- Manages data persistence
- Ensures data integrity
- Provides high-performance storage
