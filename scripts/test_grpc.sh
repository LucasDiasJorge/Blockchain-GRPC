# Scripts de exemplo para testar o serviço blockchain

# Criar grafos
echo "Creating graphs..."

grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "graph_type": 0,
  "description": "Transaction blockchain"
}' localhost:50051 blockchain.BlockchainService/CreateGraph

grpcurl -plaintext -d '{
  "graph_id": "identity",
  "graph_type": 1,
  "description": "Identity blockchain"
}' localhost:50051 blockchain.BlockchainService/CreateGraph

grpcurl -plaintext -d '{
  "graph_id": "assets",
  "graph_type": 2,
  "description": "Asset blockchain"
}' localhost:50051 blockchain.BlockchainService/CreateGraph

# Listar grafos
echo "Listing graphs..."
grpcurl -plaintext -d '{}' localhost:50051 blockchain.BlockchainService/ListGraphs

# Adicionar blocos
echo "Adding blocks..."

grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "data": "{\"from\":\"Alice\",\"to\":\"Bob\",\"amount\":100.0}"
}' localhost:50051 blockchain.BlockchainService/AddBlock

grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "data": "{\"from\":\"Bob\",\"to\":\"Charlie\",\"amount\":50.0}"
}' localhost:50051 blockchain.BlockchainService/AddBlock

# Verificar grafo
echo "Verifying graph..."
grpcurl -plaintext -d '{
  "graph_id": "transactions"
}' localhost:50051 blockchain.BlockchainService/VerifyGraph

# Validação cruzada
echo "Cross-validating all graphs..."
grpcurl -plaintext -d '{}' localhost:50051 blockchain.BlockchainService/CrossValidateGraphs

# Informações do grafo
echo "Getting graph info..."
grpcurl -plaintext -d '{
  "graph_id": "transactions"
}' localhost:50051 blockchain.BlockchainService/GetGraphInfo
