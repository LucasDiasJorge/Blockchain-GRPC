# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Multi-graph blockchain service in Rust: several independent chains ("graphs") coexist in one process, each with its own genesis block and Proof-of-Work chain, cross-referencing each other's block hashes. Exposed over gRPC (tonic), persisted in RocksDB.

## Layout & where to run commands

Cargo workspace at the repo root with a single member, `code/`. The crate is `blockchain-grpc`; the library target is `blockchain_grpc`.

**Always run cargo from the repo root, not from `code/`.** `main.rs` loads `config.json` by relative path and the RocksDB directory (`./data/blockchain`) is also relative to the working directory, so running from `code/` creates a second config/database.

## Commands

```powershell
cargo build --bin blockchain-grpc            # build server (see caveat below)
cargo run --release --bin blockchain-grpc    # run gRPC server on 0.0.0.0:50051
cargo test                                   # unit + integration tests
cargo test test_create_and_persist_graph     # single test by name
cargo test --test test_graph_creation        # one integration test file
cargo fmt
cargo clippy -- -D warnings
cargo run --example client_example           # end-to-end gRPC client demo (server must be running)
$env:RUST_LOG="blockchain_grpc=debug"; cargo run --bin blockchain-grpc
```

Helper scripts: [scripts/build.sh](scripts/build.sh) (clean + fmt + clippy + test + release build, bash), [scripts/fast-build.ps1](scripts/fast-build.ps1) (release build with sccache if installed), [scripts/run.ps1](scripts/run.ps1) (launches prebuilt binaries into `logs/`), [scripts/test_grpc.sh](scripts/test_grpc.sh) (grpcurl smoke test of every RPC).

`protoc` must be on PATH — [code/build.rs](code/build.rs) regenerates the tonic client+server on every build.

### Build caveats

- **`cargo build` with no `--bin` fails.** [code/Cargo.toml](code/Cargo.toml) declares a `[[bin]] http_proxy` at `src/bin/http_proxy.rs`, but that file does not exist in the repo. Target `--bin blockchain-grpc` explicitly, or delete the stale `[[bin]]` block.
- `[profile.*]` in [code/Cargo.toml](code/Cargo.toml) is **ignored** (cargo warns: profiles only apply at the workspace root). The effective profiles come from [.cargo/config.toml](.cargo/config.toml) — dev builds use `opt-level = 1` with dependencies at `opt-level = 3`; release uses LTO + `codegen-units = 1`.
- Mining is CPU-bound; a plain debug build can look hung while adding blocks. Use `--release` for anything perf-sensitive.

## Architecture

Layers, strictly one-directional (`infrastructure` → `application` → `domain`):

- [code/src/domain/](code/src/domain/) — pure synchronous logic, no I/O. `Block` (SHA-256 hash over `previous_hash + timestamp + data + nonce + height + graph_id + cross_references`, PoW = leading zeros), `BlockchainGraph` (chain + validation + cross-reference checking), and the trait contracts in [traits.rs](code/src/domain/traits.rs).
- [code/src/application/services/blockchain_service.rs](code/src/application/services/blockchain_service.rs) — `BlockchainServiceImpl`, where **all** request handling lives (`handle_*` methods). Holds `Arc<dyn BlockchainRepository>` plus the authoritative in-memory graph cache `Arc<RwLock<HashMap<String, BlockchainGraph>>>`.
- [code/src/infrastructure/grpc/server.rs](code/src/infrastructure/grpc/server.rs) — the tonic trait impl. Every method is a one-line delegation to a `handle_*`; keep it that way, put new behavior in the service.
- [code/src/infrastructure/persistence/](code/src/infrastructure/persistence/) — `BlockchainRepositoryImpl` (key layout, bincode serialization, its own graph cache) over `RocksDbAdapter` (raw put/get/prefix-scan).
- Generated protobuf lives at `crate::infrastructure::grpc::blockchain` via `tonic::include_proto!` in [grpc/mod.rs](code/src/infrastructure/grpc/mod.rs) — import from that path, never from `OUT_DIR`.

Startup ([code/src/main.rs](code/src/main.rs)): load settings → `RocksDbAdapter` → `BlockchainRepositoryImpl` → `BlockchainServiceImpl` → `service.initialize()` (rehydrates every graph's chain from RocksDB into the cache) → `start_grpc_server`.

### Things that will bite you

- **Two sources of truth per request.** `ListGraphs`, `GetGraphInfo`, `VerifyGraph`, `CrossValidateGraphs` and `AddBlock` read the service's in-memory cache; `GetBlock`, `GetLatestBlock`, `GetBlockRange` go to the repository. A graph absent from the cache yields "Graph not found" even when it is on disk — the cache is filled only by `initialize()` at startup and by `handle_create_graph`.
- **`BlockchainGraph.chain` is `#[serde(skip)]`.** Persisted graph metadata never contains blocks; blocks are stored individually and reassembled via `load_blocks()`. Don't assume a deserialized graph has a chain.
- **Persist the mined block, not the input block.** `BlockchainGraph::add_block` mines in place and *returns* the mined `Block`; saving the pre-mining value silently breaks hash linkage (see [docs/BUG_FIX_HASH_LINKAGE.md](docs/BUG_FIX_HASH_LINKAGE.md)).
- **Never hold a cache lock across an `await`.** `save_graph` previously held the cache write lock while calling `list_graphs()`, which re-entered the same lock and deadlocked. Both `save_graph` and `handle_create_graph` now scope their locks in explicit blocks — preserve that shape ([docs/BUG_FIX_GRAPH_CREATION.md](docs/BUG_FIX_GRAPH_CREATION.md)).
- **Handlers return `Ok(Response)` with `success: false` for domain failures**, not `Err(Status)`. Match the existing convention.
- **Difficulty is hardcoded to `2`** in `handle_create_graph`; `blockchain.default_difficulty` and `max_block_size` in [config.json](config.json) are currently unread. Per-graph difficulty is stored on the graph and used for validation.
- `AddBlockUseCase`, `VerifyGraphsUseCase` and `ValidationService` ([code/src/application/use_cases/](code/src/application/use_cases/)) are **not wired into any RPC** — the gRPC path bypasses them entirely. Changing them affects nothing at runtime; changing `BlockchainServiceImpl` or `BlockchainGraph` does.

### RocksDB key schema

```
block:{graph_id}:{height:020}   → bincode Block
block_hash:{graph_id}:{hash}    → height (u64 LE)
latest:{graph_id}               → height (u64 LE)
graph:{graph_id}                → bincode BlockchainGraph (no chain)
graph_list                      → bincode Vec<String>
```

New prefixes need migration handling; the schema is also documented in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Changing the gRPC contract

Edit [code/proto/blockchain.proto](code/proto/blockchain.proto) (the source of truth), rebuild so `build.rs` regenerates, then update *both* the trait impl in `server.rs` and the corresponding `handle_*` in `blockchain_service.rs`. Domain `GraphType` has its own `from_i32`/`to_i32` that must stay aligned with the proto enum ordering.

## Tests

Unit tests are inline `#[cfg(test)]` modules next to the code (block hashing/mining, graph validation, RocksDB adapter, validation strategies). Integration tests are in [code/tests/](code/tests/) and always use `tempfile::tempdir()` for the database, so there is nothing to clean up between runs.

## Conventions

- Code, comments and log messages are in English; prose docs, tutorials and script output are in Portuguese (pt-BR). Follow whichever the file already uses.
- Errors are `Box<dyn Error>` throughout, despite `thiserror`/`anyhow` being in the dependency list (`anyhow` is only used by the example).
- Logging is `tracing` with emoji-prefixed messages; there is one stray `println!` in `initialize()`.

## Stale references

[.github/copilot-instructions.md](.github/copilot-instructions.md) is useful but points at things that no longer exist: `src/bin/http_proxy.rs` and the Axum HTTP proxy, `tests/integration_tests.rs` (the file is `tests/test_graph_creation.rs`), the C# `Smart-Contract/` bridge, and `docs/QUICKSTART.md` (now [tutorials/QUICKSTART.md](tutorials/QUICKSTART.md)). Paths there are also relative to `code/`, not the repo root. The README and several docs reference the same removed pieces.
