use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use crate::application::services::blockchain_service::BlockchainServiceImpl;

// Use the generated protobuf module re-exported at `crate::infrastructure::grpc::blockchain`
use crate::infrastructure::grpc::blockchain::blockchain_service_server::{
    BlockchainService, BlockchainServiceServer,
};
use crate::infrastructure::grpc::blockchain::*;

/// gRPC server implementation
/// Delegates to application service layer
#[tonic::async_trait]
impl BlockchainService for BlockchainServiceImpl {
    async fn add_block(
        &self,
        request: Request<AddBlockRequest>,
    ) -> Result<Response<AddBlockResponse>, Status> {
        let req = request.into_inner();
        self.handle_add_block(req).await
    }

    async fn get_block(
        &self,
        request: Request<GetBlockRequest>,
    ) -> Result<Response<GetBlockResponse>, Status> {
        let req = request.into_inner();
        self.handle_get_block(req).await
    }

    async fn get_latest_block(
        &self,
        request: Request<GetLatestBlockRequest>,
    ) -> Result<Response<GetBlockResponse>, Status> {
        let req = request.into_inner();
        self.handle_get_latest_block(req).await
    }

    async fn get_graph_info(
        &self,
        request: Request<GetGraphInfoRequest>,
    ) -> Result<Response<GetGraphInfoResponse>, Status> {
        let req = request.into_inner();
        self.handle_get_graph_info(req).await
    }

    async fn verify_graph(
        &self,
        request: Request<VerifyGraphRequest>,
    ) -> Result<Response<VerifyGraphResponse>, Status> {
        let req = request.into_inner();
        self.handle_verify_graph(req).await
    }

    async fn cross_validate_graphs(
        &self,
        request: Request<CrossValidateRequest>,
    ) -> Result<Response<CrossValidateResponse>, Status> {
        let _req = request.into_inner();
        self.handle_cross_validate().await
    }

    async fn list_graphs(
        &self,
        request: Request<ListGraphsRequest>,
    ) -> Result<Response<ListGraphsResponse>, Status> {
        let _req = request.into_inner();
        self.handle_list_graphs().await
    }

    async fn create_graph(
        &self,
        request: Request<CreateGraphRequest>,
    ) -> Result<Response<CreateGraphResponse>, Status> {
        let req = request.into_inner();
        self.handle_create_graph(req).await
    }

    async fn get_block_range(
        &self,
        request: Request<GetBlockRangeRequest>,
    ) -> Result<Response<GetBlockRangeResponse>, Status> {
        let req = request.into_inner();
        self.handle_get_block_range(req).await
    }
}

/// Starts the gRPC server
pub async fn start_grpc_server(
    service: Arc<BlockchainServiceImpl>,
    addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;

    tracing::info!("🚀 Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(BlockchainServiceServer::from_arc(service))
        .serve(addr)
        .await?;

    Ok(())
}
