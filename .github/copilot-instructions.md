# AI working guide for this repo

This repository implements a multi-graph blockchain in Rust exposed over gRPC, with an optional HTTP JSON proxy (Axum) and a C# REST bridge. Use this playbook to be productive quickly.

## Big picture
- **Protocol-first**: gRPC contracts in `proto/blockchain.proto` drive the API. Code is generated at build time (`build.rs` via `tonic-build`).
- **Clean architecture layering**:
  - **Domain** (`src/domain/*`): Pure business logic - `Block`, `BlockchainGraph`, traits (no I/O, no async except traits).
  - **Application** (`src/application/*`): Orchestrates workflows in `services/blockchain_service.rs` using `Arc<RwLock<HashMap>>` for in-memory graph cache.
  - **Infrastructure** (`src/infrastructure/*`): gRPC server (`grpc/server.rs`), RocksDB persistence (`persistence/*`).
  - **Config** (`src/config/*`): JSON-based settings loaded in `main.rs` via `Settings::load()`.
- **Service boundary**: `BlockchainServiceImpl` (application) implements business workflows; `grpc/server.rs` only adapts gRPC requests/responses.
- **Multi-graph architecture**: Multiple independent blockchains (Transaction, Identity, Asset, Audit, Custom) can cross-reference each other for validation.

## Build/run workflows
- **Prerequisites**: Rust toolchain 1.70+, `protoc` ≥ 3.17. On Windows, use PowerShell or WSL2; on Linux/WSL install `protobuf-compiler` and C toolchain (see `docs/QUICKSTART.md` and `docs/BUILD_TROUBLESHOOTING.md`).
- **Build & run server**:
  - `cargo run --release` → gRPC server at `0.0.0.0:50051`
  - **Important**: Always use `--release` for performance (LTO enabled, opt-level=3)
  - Binary name is `blockchain-grpc` (from package name in Cargo.toml)
- **Run HTTP proxy**: `cargo run --bin http_proxy` → REST at `0.0.0.0:8080` (forwards to gRPC)
- **Tests**:
  - `cargo test` (unit + integration in `tests/integration_tests.rs`)
  - Integration tests use `tempfile::tempdir()` for isolated RocksDB instances
- **Docker**: `docker build -t blockchain-grpc .`; `docker-compose up -d`
- **Windows-specific**: PowerShell scripts in `scripts/` directory (`.ps1` extensions)

## gRPC surface (source of truth)
- Contracts: `proto/blockchain.proto` (CreateGraph, AddBlock, GetBlock/GetLatestBlock, VerifyGraph, CrossValidateGraphs, ListGraphs, GetBlockRange).
- Codegen: handled by `build.rs` using `tonic_build`; generated module is re-exported at `crate::infrastructure::grpc::blockchain`.
- Server implementation: `src/infrastructure/grpc/server.rs` forwards to methods on `BlockchainServiceImpl` (e.g., `handle_add_block`).

## Persistence and data flow
- Repository pattern (`src/domain/traits.rs`) abstracts storage; `src/infrastructure/persistence/*` provides RocksDB adapters and repository impl.
- Storage keys are documented in `docs/ARCHITECTURE.md` (e.g., `block:{graph_id}:{height:020}`, `latest:{graph_id}`, `graph_list`).
- **Key serialization pattern**: Binary via `bincode` for blocks; JSON for configs via `serde_json`.
- **In-memory cache**: `BlockchainServiceImpl` holds `Arc<RwLock<HashMap<String, BlockchainGraph>>>` for fast graph access; initialized via `service.initialize().await`.
- Typical flow (AddBlock): gRPC → Application service → Domain (mine/validate) → Repository → Response mapping.

## HTTP JSON options
- Rust proxy bin: `src/bin/http_proxy.rs` exposes REST that forwards to gRPC.
  - Config via env: `GRPC_ADDR` (default `http://127.0.0.1:50051`), `HTTP_ADDR` (default `0.0.0.0:8080`).
  - Run: `cargo run --bin http_proxy`. Endpoints documented in `docs/USAGE.md`.
- C# REST bridge (full Web API): `Smart-Contract/*` (.NET 8), generates a gRPC client from `../proto/blockchain.proto` and exposes REST controllers.
  - Run: `dotnet run -c Release` in `Smart-Contract`. Config: `Smart-Contract/appsettings.json` → `Grpc:Endpoint`.

## Conventions and patterns
- Do not put business logic in gRPC handlers; add/modify methods in `application/services/blockchain_service.rs` and keep handlers thin.
- **New RPC method workflow**:
  1) Add to `proto/blockchain.proto`.
  2) Implement mapping in `src/infrastructure/grpc/server.rs` (thin adapter, just call service method).
  3) Add corresponding method in `BlockchainServiceImpl` (and repository/domain changes as needed).
- **Persistence changes**: update `src/infrastructure/persistence/*` and keep keys consistent with `docs/ARCHITECTURE.md`.
- **Data serialization**: binary via bincode; blocks contain `cross_references` for cross-graph validation.
- **Dependency injection pattern**: `main.rs` constructs dependencies (`RocksDbAdapter` → `BlockchainRepositoryImpl` → `BlockchainServiceImpl`) and wires them via `Arc`.
- **Async boundaries**: Domain is sync; async starts at repository trait (`#[async_trait]`) and continues through application/infrastructure.

## Tips for agents
- Use existing helpers: `BlockchainServiceImpl::block_to_proto` for response mapping; repository has `save_block`, `get_block`, `get_blocks_range`, `save_graph`, `graph_exists`.
- When touching Axum proxy, this repo targets Axum 0.7 APIs (use `tokio::net::TcpListener` + `axum::serve`).
- Config file `config.json` is read by `Settings` (`src/config/settings.rs`); ensure new options are plumbed through `Settings` and used at startup (`src/main.rs`).
- Example client: `examples/client_example.rs` shows realistic end-to-end calls; use it as a reference for request payloads.
- **Error handling pattern**: Use `Box<dyn Error>` for simplicity; future evolution toward `thiserror` or `anyhow` is on roadmap.
- **Logging with tracing**: Use `tracing::info!`, `tracing::error!` for structured logging; set `RUST_LOG=debug` env var for verbose output.
- **Testing RocksDB**: Always use `tempfile::tempdir()` for test DBs to ensure isolation and cleanup.

## Common pitfalls
- Missing `protoc`/toolchain causes build failures during tonic codegen; follow `docs/QUICKSTART.md` setup section.
- Keep the separation: domain (pure), application (coordination), infrastructure (I/O). Avoid crossing layers.
- If you change `proto`, rebuild the workspace; both Rust server and the C# REST bridge depend on it.

## Pointers to start
- gRPC server entry: `src/infrastructure/grpc/server.rs`
- Core logic: `src/application/services/blockchain_service.rs`
- Domain entities: `src/domain/{block.rs,graph.rs,transaction.rs}`
- Storage: `src/infrastructure/persistence/{repository.rs,rocksdb_adapter.rs}`
- REST proxies: Rust `src/bin/http_proxy.rs`, C# `Smart-Contract/Controllers/*`
