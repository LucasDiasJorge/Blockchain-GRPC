use anyhow::Result;
use blockchain_grpc::infrastructure::grpc::blockchain::blockchain_service_client::BlockchainServiceClient;
use blockchain_grpc::infrastructure::grpc::blockchain::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔗 Blockchain gRPC Client Example\n");

    // Connect to server
    let mut client = BlockchainServiceClient::connect("http://127.0.0.1:50051").await?;
    println!("✅ Connected to server\n");

    // 1. Create graphs
    println!("📊 Creating graphs...");
    
    let graphs = vec![
        ("transactions", GraphType::Transaction as i32, "Financial transactions"),
        ("identity", GraphType::Identity as i32, "User identity data"),
        ("assets", GraphType::Asset as i32, "Asset ownership"),
    ];

    for (id, graph_type, desc) in graphs {
        let request = CreateGraphRequest {
            graph_id: id.to_string(),
            graph_type,
            description: desc.to_string(),
        };

        match client.create_graph(request).await {
            Ok(response) => {
                let res = response.into_inner();
                if res.success {
                    println!("  ✓ Created graph: {}", id);
                } else {
                    println!("  ⚠ Graph '{}' already exists", id);
                }
            }
            Err(e) => println!("  ✗ Error creating graph: {}", e),
        }
    }

    // 2. List graphs
    println!("📋 Listing all graphs...");
    let list_response = client.list_graphs(ListGraphsRequest {}).await?;
    let graphs_list = list_response.into_inner().graphs;
    
    for graph in &graphs_list {
        println!("  • {} ({:?}) - {} blocks", 
            graph.graph_id, 
            GraphType::from_i32(graph.graph_type),
            graph.total_blocks
        );
    }

    println!();

    // 3. Add blocks to transaction graph
    println!("📦 Adding blocks to 'transactions' graph...");
    
    let transactions = vec![
        r#"{"from":"Alice","to":"Bob","amount":100.0}"#,
        r#"{"from":"Bob","to":"Charlie","amount":50.0}"#,
        r#"{"from":"Charlie","to":"Alice","amount":25.0}"#,
    ];

    let mut last_hash = String::new();

    for (i, tx) in transactions.iter().enumerate() {
        let request = AddBlockRequest {
            graph_id: "transactions".to_string(),
            data: tx.to_string(),
            cross_references: vec![],
        };

        let response = client.add_block(request).await?;
        let res = response.into_inner();

        if res.success {
            if let Some(block) = res.block {
                println!("  ✓ Block {} added - Hash: {}...", i + 1, &block.hash[..16]);
                last_hash = block.hash;
            }
        }
    }

    println!();

    // 4. Add blocks with cross-references
    println!("🔗 Adding blocks with cross-references...");
    
    let identity_request = AddBlockRequest {
        graph_id: "identity".to_string(),
        data: r#"{"user":"Alice","verified":true}"#.to_string(),
        cross_references: vec![last_hash.clone()], // Reference transaction block
    };

    let response = client.add_block(identity_request).await?;
    let res = response.into_inner();

    if res.success {
        if let Some(block) = res.block {
            println!("  ✓ Identity block added with cross-reference");
            println!("    References: {}", last_hash[..16].to_string());
        }
    }

    println!();

    // 5. Get latest block
    println!("📥 Getting latest block from 'transactions'...");
    let latest_request = GetLatestBlockRequest {
        graph_id: "transactions".to_string(),
    };

    let response = client.get_latest_block(latest_request).await?;
    let res = response.into_inner();

    if res.success {
        if let Some(block) = res.block {
            println!("  Height: {}", block.height);
            println!("  Hash: {}...", &block.hash[..16]);
            println!("  Data: {}", block.data);
        }
    }

    println!();

    // 6. Get graph info
    println!("ℹ️  Getting graph information...");
    let info_request = GetGraphInfoRequest {
        graph_id: "transactions".to_string(),
    };

    let response = client.get_graph_info(info_request).await?;
    let info = response.into_inner();

    if info.success {
        println!("  Graph ID: {}", info.graph_id);
        println!("  Type: {:?}", GraphType::from_i32(info.graph_type));
        println!("  Total Blocks: {}", info.total_blocks);
        println!("  Is Valid: {}", info.is_valid);
    }

    println!();

    // 7. Verify graph integrity
    println!("🔍 Verifying graph integrity...");
    let verify_request = VerifyGraphRequest {
        graph_id: "transactions".to_string(),
    };

    let response = client.verify_graph(verify_request).await?;
    let verification = response.into_inner();

    if verification.is_valid {
        println!("  ✅ Graph is valid!");
    } else {
        println!("  ❌ Graph validation failed!");
        for error in verification.errors {
            println!("    - {}", error);
        }
    }

    println!();

    // 8. Cross-validate all graphs
    println!("🌐 Cross-validating all graphs...");
    let cross_validate_request = CrossValidateRequest {};

    let response = client.cross_validate_graphs(cross_validate_request).await?;
    let validation = response.into_inner();

    println!("  All Valid: {}", validation.all_valid);
    println!("  Graph Statuses:");
    for (graph_id, is_valid) in validation.graph_statuses {
        let status = if is_valid { "✓" } else { "✗" };
        println!("    {} {}", status, graph_id);
    }

    println!();

    // 9. Get block range
    println!("📊 Getting block range (0-2) from 'transactions'...");
    let range_request = GetBlockRangeRequest {
        graph_id: "transactions".to_string(),
        start_height: 0,
        end_height: 2,
    };

    let response = client.get_block_range(range_request).await?;
    let range_res = response.into_inner();

    if range_res.success {
        println!("  Retrieved {} blocks:", range_res.blocks.len());
        for block in range_res.blocks {
            println!("    • Height {}: {}...", block.height, &block.hash[..16]);
        }
    }

    println!("\n✅ All operations completed successfully!");

    Ok(())
}
