use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::traits::BlockchainRepository;
use crate::domain::{Block, BlockchainGraph};

use super::rocksdb_adapter::RocksDbAdapter;

/// Repository implementation using RocksDB (Repository Pattern)
/// Provides abstraction over data storage
pub struct BlockchainRepositoryImpl {
    db: Arc<RocksDbAdapter>,
    cache: Arc<RwLock<std::collections::HashMap<String, BlockchainGraph>>>,
}

impl BlockchainRepositoryImpl {
    pub fn new(db: Arc<RocksDbAdapter>) -> Self {
        Self {
            db,
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Generates key for block storage
    fn block_key(graph_id: &str, height: u64) -> String {
        format!("block:{}:{:020}", graph_id, height)
    }

    /// Generates key for block hash index
    fn block_hash_key(graph_id: &str, hash: &str) -> String {
        format!("block_hash:{}:{}", graph_id, hash)
    }

    /// Generates key for latest block pointer
    fn latest_block_key(graph_id: &str) -> String {
        format!("latest:{}", graph_id)
    }

    /// Generates key for graph metadata
    fn graph_key(graph_id: &str) -> String {
        format!("graph:{}", graph_id)
    }

    /// Generates key for graph list
    fn graph_list_key() -> String {
        "graph_list".to_string()
    }
}

#[async_trait]
impl BlockchainRepository for BlockchainRepositoryImpl {
    async fn save_block(&self, graph_id: &str, block: &Block) -> Result<(), Box<dyn Error>> {
        let serialized = bincode::serialize(block)?;

        // Save block by height
        let block_key = Self::block_key(graph_id, block.height);
        self.db.put(&block_key, &serialized)?;

        // Save hash index for quick lookup
        let hash_key = Self::block_hash_key(graph_id, &block.hash);
        self.db.put(&hash_key, &block.height.to_le_bytes())?;

        // Update latest block pointer
        let latest_key = Self::latest_block_key(graph_id);
        self.db.put(&latest_key, &block.height.to_le_bytes())?;

        Ok(())
    }

    async fn get_block(&self, graph_id: &str, hash: &str) -> Result<Option<Block>, Box<dyn Error>> {
        // Get height from hash index
        let hash_key = Self::block_hash_key(graph_id, hash);
        let height_bytes = match self.db.get(&hash_key)? {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let height = u64::from_le_bytes(height_bytes.try_into().map_err(|_| "Invalid height")?);

        // Get block by height
        self.get_block_by_height(graph_id, height).await
    }

    async fn get_latest_block(&self, graph_id: &str) -> Result<Option<Block>, Box<dyn Error>> {
        let latest_key = Self::latest_block_key(graph_id);
        let height_bytes = match self.db.get(&latest_key)? {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let height = u64::from_le_bytes(height_bytes.try_into().map_err(|_| "Invalid height")?);
        self.get_block_by_height(graph_id, height).await
    }

    async fn get_block_by_height(
        &self,
        graph_id: &str,
        height: u64,
    ) -> Result<Option<Block>, Box<dyn Error>> {
        let block_key = Self::block_key(graph_id, height);
        let data = match self.db.get(&block_key)? {
            Some(data) => data,
            None => return Ok(None),
        };

        let block: Block = bincode::deserialize(&data)?;
        Ok(Some(block))
    }

    async fn get_blocks_range(
        &self,
        graph_id: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<Block>, Box<dyn Error>> {
        let mut blocks = Vec::new();

        for height in start..=end {
            if let Some(block) = self.get_block_by_height(graph_id, height).await? {
                blocks.push(block);
            }
        }

        Ok(blocks)
    }

    async fn save_graph(&self, graph: &BlockchainGraph) -> Result<(), Box<dyn Error>> {
        tracing::info!("Starting save_graph for graph_id: {}", graph.id);

        let serialized = bincode::serialize(graph)?;
        tracing::info!("Serialized graph metadata for {}", graph.id);

        let graph_key = Self::graph_key(&graph.id);
        self.db.put(&graph_key, &serialized)?;
        tracing::info!("Saved graph metadata to DB for {}", graph.id);

        // Update cache in a short scope to avoid holding the lock across await points
        {
            let mut cache = self.cache.write().await;
            cache.insert(graph.id.clone(), graph.clone());
            tracing::info!("Updated in-memory cache for {}", graph.id);
        } // lock released here

        // Update graph list directly using DB to avoid re-entrancy on cache/list_graphs
        let list_key = Self::graph_list_key();
        tracing::info!("Loading graph list from DB");
        let current = self.db.get(&list_key)?;
        let mut graph_ids: Vec<String> = if let Some(bytes) = current {
            match bincode::deserialize(&bytes) {
                Ok(ids) => ids,
                Err(e) => {
                    tracing::error!("Failed to deserialize graph list, resetting list. Error: {}", e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        if !graph_ids.iter().any(|id| id == &graph.id) {
            tracing::info!("Graph {} not in list, adding", graph.id);
            graph_ids.push(graph.id.clone());
            let serialized_list = bincode::serialize(&graph_ids)?;
            self.db.put(&list_key, &serialized_list)?;
            tracing::info!("Updated graph list in DB");
        } else {
            tracing::info!("Graph {} already in list, skipping update", graph.id);
        }

        tracing::info!("Successfully saved graph {}", graph.id);
        Ok(())
    }

    async fn get_graph(&self, graph_id: &str) -> Result<Option<BlockchainGraph>, Box<dyn Error>> {
        // Check cache first
        tracing::trace!("get_graph: checking cache for {}", graph_id);
        {
            let cache = self.cache.read().await;
            if let Some(graph) = cache.get(graph_id) {
                tracing::debug!("get_graph: cache hit for {}", graph_id);
                return Ok(Some(graph.clone()));
            }
        }

        // Load from database
        let graph_key = Self::graph_key(graph_id);
        tracing::trace!("get_graph: loading from DB with key {}", graph_key);
        let data = match self.db.get(&graph_key)? {
            Some(data) => data,
            None => return Ok(None),
        };

        let mut graph: BlockchainGraph = bincode::deserialize(&data)?;

        // Load all blocks into graph
        let latest_block = self.get_latest_block(graph_id).await?;
        if let Some(latest) = latest_block {
            tracing::trace!("get_graph: loading blocks 0..={} for {}", latest.height, graph_id);
            let blocks = self.get_blocks_range(graph_id, 0, latest.height).await?;
            graph.load_blocks(blocks);
        }

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(graph_id.to_string(), graph.clone());
        tracing::trace!("get_graph: inserted {} into cache", graph_id);

        Ok(Some(graph))
    }

    async fn list_graphs(&self) -> Result<Vec<BlockchainGraph>, Box<dyn Error>> {
        let list_key = Self::graph_list_key();
        tracing::trace!("list_graphs: reading graph list from {}", list_key);
        let data = match self.db.get(&list_key)? {
            Some(data) => data,
            None => return Ok(Vec::new()),
        };

        let graph_ids: Vec<String> = match bincode::deserialize(&data) {
            Ok(ids) => ids,
            Err(e) => {
                tracing::error!("list_graphs: failed to deserialize list: {}", e);
                Vec::new()
            }
        };
        tracing::debug!("list_graphs: found {} ids: {:?}", graph_ids.len(), graph_ids);
        let mut graphs = Vec::new();

        for id in graph_ids {
            if let Some(graph) = self.get_graph(&id).await? {
                graphs.push(graph);
            } else {
                tracing::warn!("list_graphs: id {} present in list but graph not found in DB", id);
            }
        }

        Ok(graphs)
    }

    async fn graph_exists(&self, graph_id: &str) -> Result<bool, Box<dyn Error>> {
        let graph_key = Self::graph_key(graph_id);
        let exists = self.db.exists(&graph_key)?;
        tracing::debug!("graph_exists: key {} exists={} ", graph_key, exists);
        Ok(exists)
    }
}
