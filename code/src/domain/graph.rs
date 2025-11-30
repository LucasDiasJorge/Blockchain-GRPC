use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Block;

/// Types of blockchain graphs for different data responsibilities
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GraphType {
    Transaction, // Financial transactions
    Identity,    // Identity and authentication data
    Asset,       // Asset ownership and transfers
    Audit,       // Audit logs and compliance
    Custom,      // Custom application-specific data
}

impl GraphType {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => GraphType::Transaction,
            1 => GraphType::Identity,
            2 => GraphType::Asset,
            3 => GraphType::Audit,
            _ => GraphType::Custom,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            GraphType::Transaction => 0,
            GraphType::Identity => 1,
            GraphType::Asset => 2,
            GraphType::Audit => 3,
            GraphType::Custom => 4,
        }
    }
}

/// Represents a blockchain graph - a separate blockchain for specific data type
/// Each graph maintains its own chain while cross-referencing others
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainGraph {
    pub id: String,
    pub graph_type: GraphType,
    pub description: String,
    pub created_at: i64,
    pub difficulty: usize,
    #[serde(skip)]
    pub chain: Vec<Block>, // In-memory cache, not persisted
}

impl BlockchainGraph {
    /// Creates a new blockchain graph (Factory Method Pattern)
    pub fn new(id: String, graph_type: GraphType, description: String, difficulty: usize) -> Self {
        let genesis = Block::genesis(id.clone());

        Self {
            id: id.clone(),
            graph_type,
            description,
            created_at: Utc::now().timestamp(),
            difficulty,
            chain: vec![genesis],
        }
    }

    /// Adds a new block to the graph
    pub fn add_block(&mut self, mut block: Block) -> Result<Block, String> {
        // Validate previous hash
        if let Some(last_block) = self.chain.last() {
            if block.previous_hash != last_block.hash {
                return Err("Invalid previous hash".to_string());
            }
            if block.height != last_block.height + 1 {
                return Err("Invalid block height".to_string());
            }
        }

        // Mine the block
        block.mine_block(self.difficulty);

        // Validate the block
        if !block.is_valid() {
            return Err("Invalid block hash".to_string());
        }

        if !block.has_valid_difficulty(self.difficulty) {
            return Err("Block does not meet difficulty requirement".to_string());
        }

        self.chain.push(block.clone());
        Ok(block)
    }

    /// Gets the latest block in the graph
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    /// Validates the entire chain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check if block is valid
            if !current.is_valid() {
                return false;
            }

            // Check if previous hash matches
            if current.previous_hash != previous.hash {
                return false;
            }

            // Check difficulty
            if !current.has_valid_difficulty(self.difficulty) {
                return false;
            }

            // Check height
            if current.height != previous.height + 1 {
                return false;
            }
        }

        true
    }

    /// Loads blocks into the in-memory cache
    pub fn load_blocks(&mut self, blocks: Vec<Block>) {
        self.chain = blocks;
    }

    /// Gets total number of blocks
    pub fn get_chain_length(&self) -> u64 {
        self.chain.len() as u64
    }

    /// Validates cross-references with other graphs
    pub fn validate_cross_references(
        &self,
        other_graphs: &HashMap<String, BlockchainGraph>,
    ) -> Result<(), String> {
        for block in &self.chain {
            for cross_ref in &block.cross_references {
                let mut found = false;

                for (graph_id, graph) in other_graphs {
                    if graph_id == &self.id {
                        continue; // Skip self
                    }

                    if graph.chain.iter().any(|b| &b.hash == cross_ref) {
                        found = true;
                        break;
                    }
                }

                if !found && !cross_ref.is_empty() {
                    return Err(format!(
                        "Cross-reference {} not found in any graph",
                        cross_ref
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = BlockchainGraph::new(
            "test_graph".to_string(),
            GraphType::Transaction,
            "Test blockchain".to_string(),
            2,
        );

        assert_eq!(graph.id, "test_graph");
        assert_eq!(graph.chain.len(), 1); // Genesis block
        assert!(graph.is_valid());
    }

    #[test]
    fn test_add_block() {
        let mut graph = BlockchainGraph::new(
            "test_graph".to_string(),
            GraphType::Transaction,
            "Test".to_string(),
            2,
        );

        let latest = graph.get_latest_block().unwrap();
        let block = Block::new(
            latest.hash.clone(),
            "test data".to_string(),
            "test_graph".to_string(),
            1,
            vec![],
        );

        let result = graph.add_block(block);
        assert!(result.is_ok());
        assert_eq!(graph.chain.len(), 2);
    }

    #[test]
    fn test_invalid_previous_hash() {
        let mut graph = BlockchainGraph::new(
            "test_graph".to_string(),
            GraphType::Transaction,
            "Test".to_string(),
            2,
        );

        let block = Block::new(
            "wrong_hash".to_string(),
            "test data".to_string(),
            "test_graph".to_string(),
            1,
            vec![],
        );

        let result = graph.add_block(block);
        assert!(result.is_err());
    }
}
