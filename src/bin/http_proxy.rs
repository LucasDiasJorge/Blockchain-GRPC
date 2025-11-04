use std::net::SocketAddr;

use axum::{extract::Path, extract::Query, routing::{get, post}, Json, Router};
use serde::Deserialize;
use tonic::transport::Channel;

use blockchain_grpc::infrastructure::grpc::blockchain::blockchain_service_client::BlockchainServiceClient;
use blockchain_grpc::infrastructure::grpc::blockchain::*;

#[derive(Clone)]
struct AppState {
    grpc_addr: String,
}

// Request DTOs for HTTP JSON proxy
#[derive(Debug, Deserialize)]
struct CreateGraphDto {
    graph_id: String,
    graph_type: i32,
    description: String,
}

#[derive(Debug, Deserialize)]
struct AddBlockDto {
    data: String,
    #[serde(default)]
    cross_references: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RangeParams {
    start_height: u64,
    end_height: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // HTTP proxy listens on 8080 by default
    let http_addr: SocketAddr = std::env::var("HTTP_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()?;

    // Where the gRPC server is running
    let grpc_addr = std::env::var("GRPC_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let state = AppState { grpc_addr };

    let app = Router::new()
        // Graphs
        .route("/graphs", post(create_graph).get(list_graphs))
        .route("/graphs/verify", post(cross_validate))
        .route("/graphs/:graph_id", get(get_graph_info))
        .route("/graphs/:graph_id/verify", post(verify_graph))
        // Blocks
        .route("/graphs/:graph_id/blocks", post(add_block).get(get_block_range))
        .route("/graphs/:graph_id/blocks/latest", get(get_latest_block))
        // Health
        .route("/health", get(health_check))
        .with_state(state);

    tracing::info!("HTTP JSON proxy listening on http://{}", http_addr);
    
    let listener = tokio::net::TcpListener::bind(&http_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn grpc_client(state: &AppState) -> Result<BlockchainServiceClient<Channel>, tonic::Status> {
    BlockchainServiceClient::connect(state.grpc_addr.clone()).await.map_err(|e| {
        tracing::error!(?e, "Failed to connect to gRPC server");
        tonic::Status::unavailable("gRPC server unavailable")
    })
}

async fn create_graph(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(dto): Json<CreateGraphDto>,
) -> Result<Json<CreateGraphResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    let req = CreateGraphRequest { graph_id: dto.graph_id, graph_type: dto.graph_type, description: dto.description };
    client
        .create_graph(req)
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn list_graphs(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<ListGraphsResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    client
        .list_graphs(ListGraphsRequest {})
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn get_graph_info(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(graph_id): Path<String>,
) -> Result<Json<GetGraphInfoResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    client
        .get_graph_info(GetGraphInfoRequest { graph_id })
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn health_check(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<HealthCheckResponse>, (axum::http::StatusCode, String)> { 
    let mut client = grpc_client(&state).await.map_err(internal)?;
    client
        .health_check(HealthCheckRequest {})
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn verify_graph(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(graph_id): Path<String>,
) -> Result<Json<VerifyGraphResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    client
        .verify_graph(VerifyGraphRequest { graph_id })
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn cross_validate(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<CrossValidateResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    client
        .cross_validate_graphs(CrossValidateRequest {})
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn add_block(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(graph_id): Path<String>,
    Json(dto): Json<AddBlockDto>,
) -> Result<Json<AddBlockResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    let req = AddBlockRequest { graph_id, data: dto.data, cross_references: dto.cross_references };
    client
        .add_block(req)
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn get_latest_block(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(graph_id): Path<String>,
) -> Result<Json<GetBlockResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    client
        .get_latest_block(GetLatestBlockRequest { graph_id })
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

async fn get_block_range(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(graph_id): Path<String>,
    Query(params): Query<RangeParams>,
) -> Result<Json<GetBlockRangeResponse>, (axum::http::StatusCode, String)> {
    let mut client = grpc_client(&state).await.map_err(internal)?;
    let req = GetBlockRangeRequest { graph_id, start_height: params.start_height, end_height: params.end_height };
    client
        .get_block_range(req)
        .await
        .map(|r| Json(r.into_inner()))
        .map_err(map_grpc_err)
}

fn map_grpc_err(err: tonic::Status) -> (axum::http::StatusCode, String) {
    use axum::http::StatusCode;
    let code = match err.code() {
        tonic::Code::InvalidArgument => StatusCode::BAD_REQUEST,
        tonic::Code::NotFound => StatusCode::NOT_FOUND,
        tonic::Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (code, err.message().to_string())
}

fn internal<E: std::fmt::Display>(e: E) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
