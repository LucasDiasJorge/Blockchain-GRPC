// Integration tests for the blockchain service

use blockchain_grpc::domain::{Block, BlockchainGraph, GraphType};
use blockchain_grpc::infrastructure::persistence::{BlockchainRepositoryImpl, RocksDbAdapter};
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn test_block_creation_and_validation() {
    let mut graph = BlockchainGraph::new(
        "test".to_string(),
        GraphType::Transaction,
        "Test graph".to_string(),
        2,
    );

    // Add first block
    let latest = graph.get_latest_block().unwrap();
    let block1 = Block::new(
        latest.hash.clone(),
        "First block".to_string(),
        "test".to_string(),
        1,
        vec![],
    );

    assert!(graph.add_block(block1).is_ok());
    assert_eq!(graph.get_chain_length(), 2);
    assert!(graph.is_valid());

    // Add second block
    let latest = graph.get_latest_block().unwrap();
    let block2 = Block::new(
        latest.hash.clone(),
        "Second block".to_string(),
        "test".to_string(),
        2,
        vec![],
    );

    assert!(graph.add_block(block2).is_ok());
    assert_eq!(graph.get_chain_length(), 3);
    assert!(graph.is_valid());
}

#[tokio::test]
async fn test_invalid_previous_hash() {
    let mut graph = BlockchainGraph::new(
        "test".to_string(),
        GraphType::Transaction,
        "Test graph".to_string(),
        2,
    );

    // Try to add block with wrong previous hash
    let block = Block::new(
        "invalid_hash".to_string(),
        "Invalid block".to_string(),
        "test".to_string(),
        1,
        vec![],
    );

    assert!(graph.add_block(block).is_err());
}

#[tokio::test]
async fn test_persistence() {
    let dir = tempdir().unwrap();
    let db = Arc::new(RocksDbAdapter::new(dir.path()).unwrap());
    let repo = BlockchainRepositoryImpl::new(db);

    // Create and save graph
    let graph = BlockchainGraph::new(
        "test_persist".to_string(),
        GraphType::Transaction,
        "Persistence test".to_string(),
        2,
    );

    repo.save_graph(&graph).await.unwrap();

    // Save genesis block
    let genesis = graph.get_latest_block().unwrap();
    repo.save_block("test_persist", genesis).await.unwrap();

    // Retrieve graph
    let loaded_graph = repo.get_graph("test_persist").await.unwrap();
    assert!(loaded_graph.is_some());

    let loaded = loaded_graph.unwrap();
    assert_eq!(loaded.id, "test_persist");
    assert_eq!(loaded.get_chain_length(), 1);
}

#[tokio::test]
async fn test_block_retrieval() {
    let dir = tempdir().unwrap();
    let db = Arc::new(RocksDbAdapter::new(dir.path()).unwrap());
    let repo = BlockchainRepositoryImpl::new(db);

    let graph_id = "test_retrieval";
    let block = Block::genesis(graph_id.to_string());
    let hash = block.hash.clone();

    // Save block
    repo.save_block(graph_id, &block).await.unwrap();

    // Retrieve by hash
    let retrieved = repo.get_block(graph_id, &hash).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().hash, hash);

    // Retrieve latest
    let latest = repo.get_latest_block(graph_id).await.unwrap();
    assert!(latest.is_some());
    assert_eq!(latest.unwrap().hash, hash);

    // Retrieve by height
    let by_height = repo.get_block_by_height(graph_id, 0).await.unwrap();
    assert!(by_height.is_some());
    assert_eq!(by_height.unwrap().hash, hash);
}

#[tokio::test]
async fn test_multiple_graphs() {
    let dir = tempdir().unwrap();
    let db = Arc::new(RocksDbAdapter::new(dir.path()).unwrap());
    let repo = BlockchainRepositoryImpl::new(db);

    // Create multiple graphs
    let graph1 = BlockchainGraph::new(
        "graph1".to_string(),
        GraphType::Transaction,
        "First graph".to_string(),
        2,
    );

    let graph2 = BlockchainGraph::new(
        "graph2".to_string(),
        GraphType::Identity,
        "Second graph".to_string(),
        2,
    );

    repo.save_graph(&graph1).await.unwrap();
    repo.save_graph(&graph2).await.unwrap();

    // List graphs
    let graphs = repo.list_graphs().await.unwrap();
    assert_eq!(graphs.len(), 2);

    // Check existence
    assert!(repo.graph_exists("graph1").await.unwrap());
    assert!(repo.graph_exists("graph2").await.unwrap());
    assert!(!repo.graph_exists("nonexistent").await.unwrap());
}

#[tokio::test]
async fn test_cross_references() {
    let mut graph1 = BlockchainGraph::new(
        "graph1".to_string(),
        GraphType::Transaction,
        "Graph 1".to_string(),
        2,
    );

    let mut graph2 = BlockchainGraph::new(
        "graph2".to_string(),
        GraphType::Identity,
        "Graph 2".to_string(),
        2,
    );

    // Add block to graph1
    let latest1 = graph1.get_latest_block().unwrap();
    let block1 = Block::new(
        latest1.hash.clone(),
        "Block in graph1".to_string(),
        "graph1".to_string(),
        1,
        vec![],
    );
    graph1.add_block(block1.clone()).unwrap();

    // Add block to graph2 with cross-reference
    let latest2 = graph2.get_latest_block().unwrap();
    let block2 = Block::new(
        latest2.hash.clone(),
        "Block in graph2".to_string(),
        "graph2".to_string(),
        1,
        vec![block1.hash.clone()], // Cross-reference
    );
    graph2.add_block(block2).unwrap();

    // Validate cross-references
    let mut graphs = std::collections::HashMap::new();
    graphs.insert("graph1".to_string(), graph1);
    graphs.insert("graph2".to_string(), graph2.clone());

    assert!(graph2.validate_cross_references(&graphs).is_ok());
}

#[tokio::test]
async fn test_block_range() {
    let dir = tempdir().unwrap();
    let db = Arc::new(RocksDbAdapter::new(dir.path()).unwrap());
    let repo = BlockchainRepositoryImpl::new(db);

    let graph_id = "test_range";

    // Add multiple blocks
    for i in 0..5 {
        let block = Block::new(
            if i == 0 {
                "0".to_string()
            } else {
                format!("prev_{}", i)
            },
            format!("Block {}", i),
            graph_id.to_string(),
            i,
            vec![],
        );
        repo.save_block(graph_id, &block).await.unwrap();
    }

    // Get range
    let blocks = repo.get_blocks_range(graph_id, 1, 3).await.unwrap();
    assert_eq!(blocks.len(), 3);
    assert_eq!(blocks[0].height, 1);
    assert_eq!(blocks[1].height, 2);
    assert_eq!(blocks[2].height, 3);
}
