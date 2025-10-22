# Multi-stage build for optimal image size
FROM rust:1.75 AS builder

# Install protobuf compiler
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY proto ./proto
COPY build.rs ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/blockchain-grpc /app/blockchain-grpc

# Create data directory
RUN mkdir -p /app/data/blockchain

# Expose gRPC port
EXPOSE 50051

# Run the binary
CMD ["/app/blockchain-grpc"]
