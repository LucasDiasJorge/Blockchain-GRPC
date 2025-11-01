# Multi-stage build with aggressive caching using cargo-chef
FROM rust:1.75-slim AS chef

# Install build dependencies once and reuse across stages
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    protobuf-compiler \
    libprotobuf-dev \
    binutils \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked

WORKDIR /app

# --- Planner stage: compute dependency graph for caching ---
FROM chef AS planner

COPY Cargo.toml Cargo.lock build.rs ./
COPY proto ./proto
COPY src ./src
COPY examples ./examples

RUN cargo chef prepare --recipe-path recipe.json

# --- Builder stage: build dependencies and application ---
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --locked
RUN strip target/release/blockchain-grpc

# --- Health check utility stage ---
FROM debian:bookworm-slim AS healthcheck

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*

ARG GRPC_HEALTH_PROBE_VERSION=v0.4.25
RUN wget -qO /usr/local/bin/grpc_health_probe "https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64" \
    && chmod +x /usr/local/bin/grpc_health_probe

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    gosu \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/blockchain-grpc /usr/local/bin/blockchain-grpc
COPY --from=healthcheck /usr/local/bin/grpc_health_probe /usr/local/bin/grpc_health_probe
COPY config.example.json /app/config.example.json
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh

RUN chmod +x /usr/local/bin/entrypoint.sh

ENV CONFIG_PATH=/app/config.json \
    DATA_DIR=/app/data/blockchain \
    RUST_LOG=info

RUN useradd --system --uid 10001 --home /app blockchain \
    && mkdir -p ${DATA_DIR} \
    && chown -R blockchain:blockchain /app

VOLUME ["/app/data/blockchain"]

EXPOSE 50051

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["blockchain-grpc"]
