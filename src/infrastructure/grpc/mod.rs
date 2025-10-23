// Re-export the generated protobuf code here so other crates can import
// `crate::infrastructure::grpc::blockchain` directly.
// Re-export generated protobuf module at a stable path
// This allows imports like `crate::infrastructure::grpc::blockchain` across the codebase
pub mod blockchain {
    tonic::include_proto!("blockchain");
}

pub mod server;

pub mod server;
