use serde::{Deserialize, Serialize};
use std::fs;

/// Application settings (Configuration Pattern)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub blockchain: BlockchainSettings,
    pub storage: StorageSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainSettings {
    pub default_difficulty: usize,
    pub max_block_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
    pub data_dir: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: "0.0.0.0".to_string(),
                port: 50051,
            },
            blockchain: BlockchainSettings {
                default_difficulty: 2,
                max_block_size: 1024 * 1024, // 1MB
            },
            storage: StorageSettings {
                data_dir: "./data/blockchain".to_string(),
            },
        }
    }
}

impl Settings {
    /// Loads settings from a file or creates default
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(content) = fs::read_to_string(config_path) {
            let settings: Settings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            let settings = Settings::default();
            settings.save(config_path)?;
            Ok(settings)
        }
    }

    /// Saves settings to a file
    pub fn save(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    /// Gets the server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
