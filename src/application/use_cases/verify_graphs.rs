use std::collections::HashMap;
use std::sync::Arc;

use crate::application::services::validation_service::ValidationService;
use crate::domain::traits::BlockchainRepository;
use crate::domain::BlockchainGraph;

/// Use case for verifying all graphs and their cross-references
pub struct VerifyGraphsUseCase {
    repository: Arc<dyn BlockchainRepository>,
    validation_service: ValidationService,
}

impl VerifyGraphsUseCase {
    pub fn new(repository: Arc<dyn BlockchainRepository>) -> Self {
        Self {
            repository,
            validation_service: ValidationService::new(),
        }
    }

    /// Executes cross-validation of all graphs
    pub async fn execute(&self) -> Result<HashMap<String, bool>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();

        // Load all graphs
        let graphs = self.repository.list_graphs().await?;

        // Validate each graph internally
        for graph in &graphs {
            let is_valid = self.validation_service.validate_graph(graph).await?;
            results.insert(graph.id.clone(), is_valid);
        }

        // Validate cross-references
        let graph_map: HashMap<String, BlockchainGraph> =
            graphs.into_iter().map(|g| (g.id.clone(), g)).collect();

        for (id, graph) in &graph_map {
            if let Err(_) = graph.validate_cross_references(&graph_map) {
                results.insert(id.clone(), false);
            }
        }

        Ok(results)
    }
}
