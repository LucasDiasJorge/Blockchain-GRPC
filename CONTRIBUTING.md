# Contributing to Blockchain-GRPC

First off, thank you for considering contributing to Blockchain-GRPC! 🎉

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful and constructive in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **System information** (OS, Rust version, etc.)
- **Relevant logs**

Example:
```markdown
### Bug: Server crashes on invalid graph ID

**Steps to reproduce:**
1. Start server with `cargo run`
2. Call AddBlock with non-existent graph_id
3. Server panics

**Expected:** Error response returned
**Actual:** Server crashes with panic

**Environment:**
- OS: Ubuntu 22.04
- Rust: 1.75
- Version: 0.1.0
```

### Suggesting Enhancements

Enhancement suggestions are welcome! Please provide:

- **Clear use case**
- **Expected behavior**
- **Why it benefits the project**
- **Possible implementation approach**

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/AmazingFeature`)
3. **Make your changes**
4. **Add tests** for new functionality
5. **Ensure all tests pass** (`cargo test`)
6. **Format your code** (`cargo fmt`)
7. **Run clippy** (`cargo clippy`)
8. **Commit with clear messages**
9. **Push to your fork**
10. **Open a Pull Request**

## Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install protoc
sudo apt install protobuf-compiler libprotobuf-dev  # Ubuntu/Debian
```

### Clone and Build

```bash
git clone https://github.com/LucasDiasJorge/Blockchain-GRPC.git
cd Blockchain-GRPC
cargo build
cargo test
```

### Recommended Tools

```bash
# Code formatting
cargo install cargo-fmt

# Linting
cargo install cargo-clippy

# Auto-reload for development
cargo install cargo-watch

# Test coverage
cargo install cargo-tarpaulin
```

## Coding Standards

### Rust Style Guide

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/). Use `cargo fmt` to automatically format code.

### Naming Conventions

- **Functions/Methods**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

```rust
// Good
pub struct BlockchainGraph { }
pub fn add_block(&self) { }
const DEFAULT_DIFFICULTY: usize = 2;

// Bad
pub struct blockchain_graph { }
pub fn AddBlock(&self) { }
const default_difficulty: usize = 2;
```

### Documentation

All public APIs must be documented:

```rust
/// Creates a new blockchain graph.
///
/// # Arguments
///
/// * `id` - Unique identifier for the graph
/// * `graph_type` - Type of graph (Transaction, Identity, etc.)
/// * `description` - Human-readable description
/// * `difficulty` - Proof of work difficulty level
///
/// # Examples
///
/// ```
/// let graph = BlockchainGraph::new(
///     "transactions".to_string(),
///     GraphType::Transaction,
///     "Financial transactions".to_string(),
///     2
/// );
/// ```
pub fn new(id: String, graph_type: GraphType, description: String, difficulty: usize) -> Self {
    // implementation
}
```

### Error Handling

Use `Result` types and provide meaningful error messages:

```rust
// Good
pub async fn add_block(&self, block: Block) -> Result<(), String> {
    if !block.is_valid() {
        return Err("Block validation failed: invalid hash".to_string());
    }
    Ok(())
}

// Bad
pub async fn add_block(&self, block: Block) {
    assert!(block.is_valid()); // Don't use assertions for business logic
}
```

### SOLID Principles

Follow SOLID principles in your contributions:

- **Single Responsibility**: Each struct/module should have one reason to change
- **Open/Closed**: Extend functionality via traits, not modifications
- **Liskov Substitution**: Implementations should be substitutable
- **Interface Segregation**: Small, focused traits
- **Dependency Inversion**: Depend on abstractions, not concretions

### Design Patterns

Prefer these patterns when applicable:

- **Repository Pattern** for data access
- **Strategy Pattern** for algorithms
- **Factory Pattern** for object creation
- **Builder Pattern** for complex initialization

## Commit Guidelines

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or modifying tests
- `chore`: Build process or auxiliary tool changes

### Examples

```
feat(grpc): add endpoint for batch block addition

Implements BatchAddBlock RPC method that allows adding multiple
blocks in a single request for improved performance.

Closes #123
```

```
fix(persistence): resolve race condition in block save

Fixed a race condition where concurrent block saves could
result in incorrect height indexing.

Fixes #456
```

```
docs(api): update API documentation with new endpoints

Added documentation for BatchAddBlock and improved examples
for existing endpoints.
```

## Pull Request Process

### Before Submitting

1. **Update documentation** if you changed APIs
2. **Add tests** for new functionality
3. **Run full test suite**: `cargo test`
4. **Format code**: `cargo fmt`
5. **Check lints**: `cargo clippy -- -D warnings`
6. **Update CHANGELOG.md** if applicable

### PR Description Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe the tests you ran

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] No new warnings
```

### Review Process

1. At least one maintainer must review
2. All comments must be addressed
3. CI must pass
4. No merge conflicts

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration_tests
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(/* ... */);
        assert!(block.is_valid());
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = some_async_function().await;
        assert!(result.is_ok());
    }
}
```

### Test Coverage

Aim for:
- **Domain logic**: 90%+ coverage
- **Application services**: 80%+ coverage
- **Infrastructure**: 70%+ coverage

```bash
# Generate coverage report
cargo tarpaulin --out Html
```

## Architecture Guidelines

### Layer Boundaries

Respect architectural layers:

```
Presentation (gRPC)
    ↓
Application (Services, Use Cases)
    ↓
Domain (Entities, Business Logic)
    ↓
Infrastructure (Persistence, External)
```

- **Domain** should not depend on infrastructure
- **Application** coordinates between layers
- **Infrastructure** implements interfaces defined in domain

### Adding New Features

1. **Define domain entities** in `src/domain/`
2. **Create traits** if needed in `src/domain/traits.rs`
3. **Implement use cases** in `src/application/use_cases/`
4. **Add infrastructure** in `src/infrastructure/`
5. **Expose via gRPC** in `proto/` and `src/infrastructure/grpc/`
6. **Write tests** at each layer

## Questions?

Feel free to:
- Open an issue for questions
- Join discussions
- Reach out to maintainers

Thank you for contributing! 🙏
