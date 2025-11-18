# AI working guide for this repo

Use this sheet to get productive quickly inside the multi-graph Rust blockchain service that exposes gRPC plus optional REST bridges.

## Architecture at a glance
- Multi-graph blockchain: each graph (`Transaction`, `Identity`, etc.) runs its own chain and cross-references peers via `Block::cross_references` (`src/domain/block.rs`).
- Layers stay clean: domain (`src/domain/*`) is pure sync logic, application (`src/application/services/*.rs`) orchestrates with `Arc<RwLock<HashMap>>`, infrastructure (`src/infrastructure/*`) hosts gRPC + RocksDB adapters, and `src/config/*` loads JSON settings.
- Data flow: gRPC handler (`src/infrastructure/grpc/server.rs`) → `BlockchainServiceImpl::{handle_add_block,...}` → domain validation/mining → persistence via `BlockchainRepositoryImpl` (RocksDB keys like `block:{graph}:{height:020}` from `docs/ARCHITECTURE.md`).
- Startup (`src/main.rs`): load `config.json`, wire `RocksDbAdapter` → `BlockchainRepositoryImpl` → `BlockchainServiceImpl`, then call `service.initialize().await` to hydrate the in-memory cache before serving traffic.

## Critical workflows
- Build/run server: `cargo run --release` (LTO + POW difficulty expect release-grade perf) opens gRPC on `0.0.0.0:50051`.
- HTTP proxy: `cargo run --bin http_proxy` respects `GRPC_ADDR`/`HTTP_ADDR` env vars and forwards to tonic server (`src/bin/http_proxy.rs`).
- Tests: `cargo test` covers unit + `tests/integration_tests.rs`; RocksDB fixtures always use `tempfile::tempdir()` so no manual cleanup.
- Proto edits: update `proto/blockchain.proto`, let `build.rs` regenerate via `tonic_build`, then update both the gRPC adapter and `BlockchainServiceImpl`; rebuild the C# bridge under `Smart-Contract/` if those messages changed.
- Windows helpers live in `scripts/*.ps1`; prefer them for local dev if PowerShell environment is pre-configured.

## Conventions & patterns
- Keep gRPC handlers thin—never add business logic to `server.rs`; implement new behaviors inside `BlockchainServiceImpl` or dedicated use cases (`src/application/use_cases/*`).
- Validation lives in `ValidationService` and strategy traits under `src/application/services/validation_service.rs`; extend those traits instead of sprinkling ad-hoc checks.
- Use provided helpers: `BlockchainServiceImpl::block_to_proto` for response mapping, repository trait (`src/domain/traits.rs`) for persistence access, `BlockchainGraph::load_blocks` when warming caches.
- Serialization choices are deliberate: configs via `serde_json`, RocksDB payloads via `bincode`; keep new data formats consistent so `BlockchainRepositoryImpl` stays coherent.
- Stick with `Arc<RwLock<_>>` for shared state—`graphs` cache lives there, so cloning the `Arc` is cheap and write locks stay minimal.

## Integration points
- gRPC contract is the source of truth (`proto/blockchain.proto`); both the Axum proxy and C# REST bridge consume the same generated module (`crate::infrastructure::grpc::blockchain`).
- REST options: lightweight Axum proxy (`src/bin/http_proxy.rs`) or the full ASP.NET bridge in `Smart-Contract/` (configure `Grpc:Endpoint` in its `appsettings.json`, run via `dotnet run -c Release`).
- Example client at `examples/client_example.rs` demonstrates constructing realistic tonic requests—reuse those payload shapes for manual tests.

## Pitfalls & troubleshooting
- Missing `protoc` or outdated Rust toolchains cause build failures during `tonic_build`; follow `docs/QUICKSTART.md` / `docs/BUILD_TROUBLESHOOTING.md` for setup fixes.
- Always regenerate and re-run after touching the proto; stale generated code desyncs both Rust and C# clients.
- POW difficulty assumes release builds—debug binaries will appear “hung” while mining; switch back to `--release` if blocks don’t finalize.
- Respect storage schema documented in `docs/ARCHITECTURE.md`; introducing new RocksDB prefixes requires migration logic before deploy.

## Quick references
- Core service: `src/application/services/blockchain_service.rs`
- Domain entities: `src/domain/{block.rs,graph.rs,traits.rs}`
- Persistence: `src/infrastructure/persistence/{repository.rs,rocksdb_adapter.rs}`
- Entrypoint: `src/main.rs`
- Tests: `tests/integration_tests.rs`
