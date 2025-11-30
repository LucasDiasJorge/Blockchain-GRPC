use std::sync::Arc;

use crate::domain::traits::BlockchainRepository;
use crate::domain::Block;

/// Use case for adding a block to a graph (Use Case Pattern)
/// Encapsulates business logic for a specific operation
pub struct AddBlockUseCase {
    repository: Arc<dyn BlockchainRepository>,
}

impl AddBlockUseCase {
    pub fn new(repository: Arc<dyn BlockchainRepository>) -> Self {
        Self { repository }
    }

    /// Executes the use case
    pub async fn execute(
        &self,
        graph_id: String,
        data: String,
        cross_references: Vec<String>,
    ) -> Result<Block, Box<dyn std::error::Error>> {
        // Get the latest block
        let latest = self.repository.get_latest_block(&graph_id).await?;

        let (previous_hash, height) = match latest {
            Some(block) => (block.hash, block.height + 1),
            None => ("0".to_string(), 0),
        };

        // Create new block
        let block = Block::new(previous_hash, data, graph_id.clone(), height, cross_references);

        // Save block
        self.repository.save_block(&graph_id, &block).await?;

        Ok(block)
    }
}
