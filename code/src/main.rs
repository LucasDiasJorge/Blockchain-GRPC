use std::sync::Arc;

use blockchain_grpc::{BlockchainServiceImpl, Settings};
use blockchain_grpc::infrastructure::persistence::{BlockchainRepositoryImpl, RocksDbAdapter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("🔗 Starting Blockchain gRPC Service");

    // Load settings
    let settings = Settings::load("config.json")?;
    tracing::info!("⚙️  Configuration loaded");

    // Initialize storage
    std::fs::create_dir_all(&settings.storage.data_dir)?;
    let db = Arc::new(RocksDbAdapter::new(&settings.storage.data_dir)?);
    tracing::info!("💾 Storage initialized at {}", settings.storage.data_dir);

    // Initialize repository
    let repository = Arc::new(BlockchainRepositoryImpl::new(db));

    // Initialize service
    let service = Arc::new(BlockchainServiceImpl::new(repository));
    service.initialize().await?;

    tracing::info!("✅ Service initialized successfully");

    // Start gRPC server
    let addr = settings.server_address();
    blockchain_grpc::start_grpc_server(service, addr).await?;

    Ok(())
}
