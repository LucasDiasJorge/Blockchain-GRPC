use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Represents a single block in the blockchain
/// Immutable by design (following functional programming principles)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
    pub height: u64,
    pub graph_id: String,
    pub cross_references: Vec<String>, // References to blocks in other graphs
}

impl Block {
    /// Creates a new block (Factory Method Pattern)
    pub fn new(
        previous_hash: String,
        data: String,
        graph_id: String,
        height: u64,
        cross_references: Vec<String>,
    ) -> Self {
        let timestamp = Utc::now().timestamp();
        let nonce = 0;

        let mut block = Self {
            hash: String::new(),
            previous_hash: previous_hash.clone(),
            timestamp,
            data: data.clone(),
            nonce,
            height,
            graph_id: graph_id.clone(),
            cross_references,
        };

        block.hash = block.calculate_hash();
        block
    }

    /// Creates the genesis block (Factory Method Pattern)
    pub fn genesis(graph_id: String) -> Self {
        Self::new(
            "0".to_string(),
            "Genesis Block".to_string(),
            graph_id,
            0,
            vec![],
        )
    }

    /// Calculates the hash of the block
    pub fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}{}{}",
            self.previous_hash,
            self.timestamp,
            self.data,
            self.nonce,
            self.height,
            self.graph_id,
            self.cross_references.join(",")
        );

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Validates if the block's hash is correct
    pub fn is_valid(&self) -> bool {
        self.hash == self.calculate_hash()
    }

    /// Mines the block with given difficulty (Proof of Work)
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    /// Checks if block has required difficulty
    pub fn has_valid_difficulty(&self, difficulty: usize) -> bool {
        let target = "0".repeat(difficulty);
        self.hash.starts_with(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            "previous_hash".to_string(),
            "test data".to_string(),
            "test_graph".to_string(),
            1,
            vec![],
        );

        assert_eq!(block.height, 1);
        assert_eq!(block.graph_id, "test_graph");
        assert!(block.is_valid());
    }

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis("test_graph".to_string());

        assert_eq!(genesis.height, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert!(genesis.is_valid());
    }

    #[test]
    fn test_mining() {
        let mut block = Block::new(
            "prev".to_string(),
            "data".to_string(),
            "graph".to_string(),
            1,
            vec![],
        );

        block.mine_block(2);
        assert!(block.hash.starts_with("00"));
        assert!(block.is_valid());
    }
}
