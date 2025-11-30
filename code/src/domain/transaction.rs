use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a transaction in the blockchain
/// This is an example domain entity that can be stored in blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: i64,
    pub signature: Option<String>,
    pub metadata: Option<String>,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(from: String, to: String, amount: f64, metadata: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from,
            to,
            amount,
            timestamp: Utc::now().timestamp(),
            signature: None,
            metadata,
        }
    }

    /// Serializes transaction to JSON string for block data
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes transaction from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Validates the transaction
    pub fn is_valid(&self) -> bool {
        !self.from.is_empty() && !self.to.is_empty() && self.amount > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            100.0,
            Some("payment".to_string()),
        );

        assert!(!tx.id.is_empty());
        assert_eq!(tx.from, "alice");
        assert_eq!(tx.to, "bob");
        assert_eq!(tx.amount, 100.0);
        assert!(tx.is_valid());
    }

    #[test]
    fn test_transaction_serialization() {
        let tx = Transaction::new("alice".to_string(), "bob".to_string(), 100.0, None);

        let json = tx.to_json().unwrap();
        let deserialized = Transaction::from_json(&json).unwrap();

        assert_eq!(tx.id, deserialized.id);
        assert_eq!(tx.amount, deserialized.amount);
    }

    #[test]
    fn test_invalid_transaction() {
        let mut tx = Transaction::new("alice".to_string(), "bob".to_string(), 100.0, None);
        tx.amount = -10.0;

        assert!(!tx.is_valid());
    }
}
