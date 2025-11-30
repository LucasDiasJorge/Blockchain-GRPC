use async_trait::async_trait;
use std::error::Error;

use crate::domain::traits::ValidationStrategy;
use crate::domain::BlockchainGraph;

/// Service for validating blockchain graphs (Strategy Pattern)
pub struct ValidationService {
    strategies: Vec<Box<dyn ValidationStrategy>>,
}

impl ValidationService {
    pub fn new() -> Self {
        Self {
            strategies: vec![
                Box::new(ChainIntegrityValidator),
                Box::new(BlockHashValidator),
                Box::new(DifficultyValidator),
            ],
        }
    }

    /// Validates a graph using all registered strategies
    pub async fn validate_graph(&self, graph: &BlockchainGraph) -> Result<bool, Box<dyn Error>> {
        for strategy in &self.strategies {
            if !strategy.validate(graph).await? {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Validates chain integrity (previous hash linking)
struct ChainIntegrityValidator;

#[async_trait]
impl ValidationStrategy for ChainIntegrityValidator {
    async fn validate(&self, graph: &BlockchainGraph) -> Result<bool, Box<dyn Error>> {
        for i in 1..graph.chain.len() {
            let current = &graph.chain[i];
            let previous = &graph.chain[i - 1];

            if current.previous_hash != previous.hash {
                return Ok(false);
            }

            if current.height != previous.height + 1 {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Validates block hashes
struct BlockHashValidator;

#[async_trait]
impl ValidationStrategy for BlockHashValidator {
    async fn validate(&self, graph: &BlockchainGraph) -> Result<bool, Box<dyn Error>> {
        for block in &graph.chain {
            if !block.is_valid() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Validates proof of work difficulty
struct DifficultyValidator;

#[async_trait]
impl ValidationStrategy for DifficultyValidator {
    async fn validate(&self, graph: &BlockchainGraph) -> Result<bool, Box<dyn Error>> {
        for block in &graph.chain {
            if !block.has_valid_difficulty(graph.difficulty) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Block, GraphType};

    #[tokio::test]
    async fn test_validation_service() {
        let mut graph = BlockchainGraph::new(
            "test".to_string(),
            GraphType::Transaction,
            "Test".to_string(),
            2,
        );

        let service = ValidationService::new();
        let result = service.validate_graph(&graph).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
