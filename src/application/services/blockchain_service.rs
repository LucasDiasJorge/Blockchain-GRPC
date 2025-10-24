use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Response, Status};

use crate::domain::traits::BlockchainRepository;
use crate::domain::{Block, BlockchainGraph, GraphType};
use crate::infrastructure::grpc::blockchain::*;

/// Main blockchain service (Application Service Layer)
/// Orchestrates business logic and coordinates between layers
/// Follows Single Responsibility Principle
pub struct BlockchainServiceImpl {
    repository: Arc<dyn BlockchainRepository>,
    graphs: Arc<RwLock<HashMap<String, BlockchainGraph>>>,
}

impl BlockchainServiceImpl {
    pub fn new(repository: Arc<dyn BlockchainRepository>) -> Self {
        Self {
            repository,
            graphs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initializes the service by loading existing graphs from storage
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Initializing blockchain service...");

        let graphs = self.repository.list_graphs().await?;
        let mut graph_map = self.graphs.write().await;

        for mut graph in graphs {
            tracing::info!(
                "Loading graph: {} ({} blocks)",
                graph.id,
                graph.get_chain_length()
            );

            // Load all blocks for this graph
            if let Some(latest) = self.repository.get_latest_block(&graph.id).await? {
                let blocks = self
                    .repository
                    .get_blocks_range(&graph.id, 0, latest.height)
                    .await?;
                graph.load_blocks(blocks);
            }

            graph_map.insert(graph.id.clone(), graph);
        }

        tracing::info!("✅ Loaded {} graphs", graph_map.len());
        Ok(())
    }

    /// Handles adding a new block to a graph
    pub async fn handle_add_block(
        &self,
        request: AddBlockRequest,
    ) -> Result<Response<AddBlockResponse>, Status> {
        let graph_id = request.graph_id.clone();

        // Get or create graph
        let mut graphs = self.graphs.write().await;
        let graph = match graphs.get_mut(&graph_id) {
            Some(g) => g,
            None => {
                return Ok(Response::new(AddBlockResponse {
                    success: false,
                    message: format!("Graph '{}' not found", graph_id),
                    block: None,
                }));
            }
        };

        // Get previous block info
        let (previous_hash, height) = match graph.get_latest_block() {
            Some(block) => (block.hash.clone(), block.height + 1),
            None => ("0".to_string(), 0),
        };

        // Create new block
        let block = Block::new(
            previous_hash,
            request.data,
            graph_id.clone(),
            height,
            request.cross_references,
        );

        // Add block to graph
        if let Err(e) = graph.add_block(block.clone()) {
            return Ok(Response::new(AddBlockResponse {
                success: false,
                message: format!("Failed to add block: {}", e),
                block: None,
            }));
        }

        // Persist block
        if let Err(e) = self.repository.save_block(&graph_id, &block).await {
            return Ok(Response::new(AddBlockResponse {
                success: false,
                message: format!("Failed to persist block: {}", e),
                block: None,
            }));
        }

        // Convert to proto block
        let proto_block = self.block_to_proto(&block);

        Ok(Response::new(AddBlockResponse {
            success: true,
            message: "Block added successfully".to_string(),
            block: Some(proto_block),
        }))
    }

    /// Handles getting a block by hash
    pub async fn handle_get_block(
        &self,
        request: GetBlockRequest,
    ) -> Result<Response<GetBlockResponse>, Status> {
        match self
            .repository
            .get_block(&request.graph_id, &request.hash)
            .await
        {
            Ok(Some(block)) => Ok(Response::new(GetBlockResponse {
                success: true,
                message: "Block found".to_string(),
                block: Some(self.block_to_proto(&block)),
            })),
            Ok(None) => Ok(Response::new(GetBlockResponse {
                success: false,
                message: "Block not found".to_string(),
                block: None,
            })),
            Err(e) => Ok(Response::new(GetBlockResponse {
                success: false,
                message: format!("Error: {}", e),
                block: None,
            })),
        }
    }

    /// Handles getting the latest block
    pub async fn handle_get_latest_block(
        &self,
        request: GetLatestBlockRequest,
    ) -> Result<Response<GetBlockResponse>, Status> {
        match self.repository.get_latest_block(&request.graph_id).await {
            Ok(Some(block)) => Ok(Response::new(GetBlockResponse {
                success: true,
                message: "Latest block found".to_string(),
                block: Some(self.block_to_proto(&block)),
            })),
            Ok(None) => Ok(Response::new(GetBlockResponse {
                success: false,
                message: "No blocks in graph".to_string(),
                block: None,
            })),
            Err(e) => Ok(Response::new(GetBlockResponse {
                success: false,
                message: format!("Error: {}", e),
                block: None,
            })),
        }
    }

    /// Handles getting graph information
    pub async fn handle_get_graph_info(
        &self,
        request: GetGraphInfoRequest,
    ) -> Result<Response<GetGraphInfoResponse>, Status> {
        let graphs = self.graphs.read().await;

        match graphs.get(&request.graph_id) {
            Some(graph) => {
                let latest_hash = graph
                    .get_latest_block()
                    .map(|b| b.hash.clone())
                    .unwrap_or_default();

                Ok(Response::new(GetGraphInfoResponse {
                    success: true,
                    graph_id: graph.id.clone(),
                    graph_type: graph.graph_type.to_i32(),
                    total_blocks: graph.get_chain_length(),
                    latest_hash,
                    created_at: graph.created_at,
                    is_valid: graph.is_valid(),
                }))
            }
            None => Ok(Response::new(GetGraphInfoResponse {
                success: false,
                graph_id: request.graph_id,
                graph_type: 0,
                total_blocks: 0,
                latest_hash: String::new(),
                created_at: 0,
                is_valid: false,
            })),
        }
    }

    /// Handles graph verification
    pub async fn handle_verify_graph(
        &self,
        request: VerifyGraphRequest,
    ) -> Result<Response<VerifyGraphResponse>, Status> {
        let graphs = self.graphs.read().await;

        match graphs.get(&request.graph_id) {
            Some(graph) => {
                let is_valid = graph.is_valid();
                let message = if is_valid {
                    "Graph is valid".to_string()
                } else {
                    "Graph integrity check failed".to_string()
                };

                Ok(Response::new(VerifyGraphResponse {
                    success: true,
                    is_valid,
                    message,
                    errors: vec![],
                }))
            }
            None => Ok(Response::new(VerifyGraphResponse {
                success: false,
                is_valid: false,
                message: format!("Graph '{}' not found", request.graph_id),
                errors: vec![],
            })),
        }
    }

    /// Handles cross-validation of all graphs
    pub async fn handle_cross_validate(
        &self,
    ) -> Result<Response<CrossValidateResponse>, Status> {
        let graphs = self.graphs.read().await;
        let mut statuses = HashMap::new();
        let mut all_valid = true;

        for (id, graph) in graphs.iter() {
            let is_valid = graph.is_valid();
            statuses.insert(id.clone(), is_valid);

            if !is_valid {
                all_valid = false;
            }
        }

        // Validate cross-references
        for (_, graph) in graphs.iter() {
            if let Err(_e) = graph.validate_cross_references(&graphs) {
                all_valid = false;
            }
        }

        Ok(Response::new(CrossValidateResponse {
            success: true,
            all_valid,
            message: if all_valid {
                "All graphs are valid".to_string()
            } else {
                "Some graphs have validation errors".to_string()
            },
            graph_statuses: statuses,
        }))
    }

    /// Handles listing all graphs
    pub async fn handle_list_graphs(
        &self,
    ) -> Result<Response<ListGraphsResponse>, Status> {
        let graphs = self.graphs.read().await;

        let graph_infos: Vec<GraphInfo> = graphs
            .values()
            .map(|g| GraphInfo {
                graph_id: g.id.clone(),
                graph_type: g.graph_type.to_i32(),
                total_blocks: g.get_chain_length(),
                description: g.description.clone(),
            })
            .collect();

        Ok(Response::new(ListGraphsResponse {
            graphs: graph_infos,
        }))
    }

    /// Handles creating a new graph
    pub async fn handle_create_graph(
        &self,
        request: CreateGraphRequest,
    ) -> Result<Response<CreateGraphResponse>, Status> {
        let graph_id = request.graph_id.clone();

        // Check if graph already exists
        if self
            .repository
            .graph_exists(&graph_id)
            .await
            .unwrap_or(false)
        {
            return Ok(Response::new(CreateGraphResponse {
                success: false,
                message: format!("Graph '{}' already exists", graph_id),
                graph_info: None,
            }));
        }

        // Create new graph
        let graph_type = GraphType::from_i32(request.graph_type);
        let graph = BlockchainGraph::new(graph_id.clone(), graph_type, request.description, 2);

        // Persist genesis block
        if let Some(genesis) = graph.get_latest_block() {
            if let Err(e) = self.repository.save_block(&graph_id, genesis).await {
                return Ok(Response::new(CreateGraphResponse {
                    success: false,
                    message: format!("Failed to persist genesis block: {}", e),
                    graph_info: None,
                }));
            }
        }

        // Save graph metadata
        if let Err(e) = self.repository.save_graph(&graph).await {
            return Ok(Response::new(CreateGraphResponse {
                success: false,
                message: format!("Failed to save graph: {}", e),
                graph_info: None,
            }));
        }

        // Add to in-memory cache
        let mut graphs = self.graphs.write().await;
        let graph_info = GraphInfo {
            graph_id: graph.id.clone(),
            graph_type: graph.graph_type.to_i32(),
            total_blocks: graph.get_chain_length(),
            description: graph.description.clone(),
        };

        graphs.insert(graph_id, graph);

        Ok(Response::new(CreateGraphResponse {
            success: true,
            message: "Graph created successfully".to_string(),
            graph_info: Some(graph_info),
        }))
    }

    /// Handles getting a range of blocks
    pub async fn handle_get_block_range(
        &self,
        request: GetBlockRangeRequest,
    ) -> Result<Response<GetBlockRangeResponse>, Status> {
        match self
            .repository
            .get_blocks_range(&request.graph_id, request.start_height, request.end_height)
            .await
        {
            Ok(blocks) => {
                let proto_blocks: Vec<crate::infrastructure::grpc::blockchain::Block> =
                    blocks.iter().map(|b| self.block_to_proto(b)).collect();

                Ok(Response::new(GetBlockRangeResponse {
                    success: true,
                    blocks: proto_blocks,
                }))
            }
            Err(_e) => Ok(Response::new(GetBlockRangeResponse {
                success: false,
                blocks: vec![],
            })),
        }
    }

    /// Converts domain Block to proto Block
    fn block_to_proto(&self, block: &Block) -> crate::infrastructure::grpc::blockchain::Block {
        crate::infrastructure::grpc::blockchain::Block {
            hash: block.hash.clone(),
            previous_hash: block.previous_hash.clone(),
            timestamp: block.timestamp,
            nonce: block.nonce,
            data: block.data.clone(),
            height: block.height,
            graph_id: block.graph_id.clone(),
            cross_references: block.cross_references.clone(),
        }
    }
}
