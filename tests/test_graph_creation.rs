// Test specifically for graph creation and persistence

use blockchain_grpc::domain::{BlockchainGraph, GraphType};
use blockchain_grpc::infrastructure::persistence::{BlockchainRepositoryImpl, RocksDbAdapter};
use blockchain_grpc::domain::traits::BlockchainRepository;
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn test_create_and_persist_graph() {
    // Setup temporary database
    let temp_dir = tempdir().unwrap();
    let db = Arc::new(RocksDbAdapter::new(temp_dir.path()).unwrap());
    let repository = Arc::new(BlockchainRepositoryImpl::new(db));

    // Create a new graph
    let graph_id = "test_graph".to_string();
    let graph = BlockchainGraph::new(
        graph_id.clone(),
        GraphType::Transaction,
        "Test Graph Description".to_string(),
        2,
    );

    println!("Created graph: {:?}", graph.id);
    println!("Genesis block hash: {:?}", graph.get_latest_block().map(|b| b.hash.clone()));

    // Save the genesis block
    if let Some(genesis) = graph.get_latest_block() {
        println!("Saving genesis block...");
        repository.save_block(&graph_id, genesis).await.expect("Failed to save genesis block");
        println!("Genesis block saved!");
    }

    // Save the graph metadata
    println!("Saving graph metadata...");
    repository.save_graph(&graph).await.expect("Failed to save graph");
    println!("Graph metadata saved!");

    // Verify graph exists
    let exists = repository.graph_exists(&graph_id).await.expect("Failed to check existence");
    println!("Graph exists check: {}", exists);
    assert!(exists, "Graph should exist after saving");

    // Retrieve the graph
    println!("Retrieving graph...");
    let retrieved_graph = repository.get_graph(&graph_id).await.expect("Failed to retrieve graph");
    println!("Retrieved graph: {:?}", retrieved_graph.is_some());
    
    assert!(retrieved_graph.is_some(), "Should retrieve the saved graph");
    
    let retrieved = retrieved_graph.unwrap();
    assert_eq!(retrieved.id, graph_id);
    assert_eq!(retrieved.get_chain_length(), 1); // Should have genesis block
    
    // Verify the genesis block can be retrieved
    println!("Checking genesis block...");
    let genesis_retrieved = repository.get_latest_block(&graph_id).await.expect("Failed to get latest block");
    assert!(genesis_retrieved.is_some(), "Should retrieve genesis block");
    
    println!("✅ Test passed!");
}

#[tokio::test]
async fn test_list_graphs() {
    // Setup temporary database
    let temp_dir = tempdir().unwrap();
    let db = Arc::new(RocksDbAdapter::new(temp_dir.path()).unwrap());
    let repository = Arc::new(BlockchainRepositoryImpl::new(db));

    // Create multiple graphs
    for i in 0..3 {
        let graph_id = format!("graph_{}", i);
        let graph = BlockchainGraph::new(
            graph_id.clone(),
            GraphType::Transaction,
            format!("Graph {}", i),
            2,
        );

        if let Some(genesis) = graph.get_latest_block() {
            repository.save_block(&graph_id, genesis).await.unwrap();
        }
        
        repository.save_graph(&graph).await.unwrap();
    }

    // List all graphs
    let graphs = repository.list_graphs().await.expect("Failed to list graphs");
    println!("Found {} graphs", graphs.len());
    
    assert_eq!(graphs.len(), 3, "Should have 3 graphs");
    
    println!("✅ Test passed!");
}
