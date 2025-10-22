// Re-export the generated protobuf code here so other crates can import
// `crate::infrastructure::grpc::blockchain` directly.
pub mod blockchain {
	tonic::include_proto!("blockchain");
}

pub mod server;
