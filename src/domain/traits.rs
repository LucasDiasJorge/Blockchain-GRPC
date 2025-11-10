use async_trait::async_trait;
use std::error::Error;

use super::{Block, BlockchainGraph};

/// Repository trait for blockchain persistence (Repository Pattern)
/// Following Interface Segregation Principle
#[async_trait]
pub trait BlockchainRepository: Send + Sync {
    async fn save_block(&self, graph_id: &str, block: &Block) -> Result<(), Box<dyn Error>>;
    async fn get_block(&self, graph_id: &str, hash: &str) -> Result<Option<Block>, Box<dyn Error>>;
    async fn get_latest_block(&self, graph_id: &str) -> Result<Option<Block>, Box<dyn Error>>;
    async fn get_block_by_height(&self, graph_id: &str, height: u64,) -> Result<Option<Block>, Box<dyn Error>>;
    async fn get_blocks_range(&self, graph_id: &str, start: u64, end: u64,) -> Result<Vec<Block>, Box<dyn Error>>;
    async fn save_graph(&self, graph: &BlockchainGraph) -> Result<(), Box<dyn Error>>;
    async fn get_graph(&self, graph_id: &str) -> Result<Option<BlockchainGraph>, Box<dyn Error>>;
    async fn list_graphs(&self) -> Result<Vec<BlockchainGraph>, Box<dyn Error>>;
    async fn graph_exists(&self, graph_id: &str) -> Result<bool, Box<dyn Error>>;
}

/// Validation strategy trait (Strategy Pattern)
#[async_trait]
pub trait ValidationStrategy: Send + Sync {
    async fn validate(&self, graph: &BlockchainGraph) -> Result<bool, Box<dyn Error>>;
}

/// Hash calculator trait (Strategy Pattern)
pub trait HashCalculator: Send + Sync {
    fn calculate_hash(
        &self,
        previous_hash: &str,
        timestamp: i64,
        data: &str,
        nonce: u64,
    ) -> String;
}

/// Proof of Work trait
#[async_trait]
pub trait ProofOfWork: Send + Sync {
    async fn mine(&self, block: &mut Block, difficulty: usize) -> Result<(), Box<dyn Error>>;
}
