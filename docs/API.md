# API Reference - gRPC Endpoints

## Service: BlockchainService

### CreateGraph

Creates a new blockchain graph with specified type and configuration.

**Request:**
```protobuf
message CreateGraphRequest {
    string graph_id = 1;        // Unique identifier for the graph
    GraphType graph_type = 2;   // Type: TRANSACTION, IDENTITY, ASSET, AUDIT, CUSTOM
    string description = 3;     // Human-readable description
}
```

**Response:**
```protobuf
message CreateGraphResponse {
    bool success = 1;
    string message = 2;
    GraphInfo graph_info = 3;
}
```

**Example (grpcurl):**
```bash
grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "graph_type": 0,
  "description": "Financial transactions blockchain"
}' localhost:50051 blockchain.BlockchainService/CreateGraph
```

---

### AddBlock

Adds a new block to a specific graph with optional cross-references.

**Request:**
```protobuf
message AddBlockRequest {
    string graph_id = 1;                    // Target graph ID
    string data = 2;                        // Block data (JSON, binary, etc)
    repeated string cross_references = 3;   // Hashes from other graphs
}
```

**Response:**
```protobuf
message AddBlockResponse {
    bool success = 1;
    string message = 2;
    Block block = 3;    // The newly created block
}
```

**Example:**
```bash
grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "data": "{\"from\":\"Alice\",\"to\":\"Bob\",\"amount\":100.0}",
  "cross_references": []
}' localhost:50051 blockchain.BlockchainService/AddBlock
```

---

### GetBlock

Retrieves a specific block by its hash.

**Request:**
```protobuf
message GetBlockRequest {
    string graph_id = 1;
    string hash = 2;
}
```

**Response:**
```protobuf
message GetBlockResponse {
    bool success = 1;
    string message = 2;
    Block block = 3;
}
```

---

### GetLatestBlock

Gets the most recent block in a graph.

**Request:**
```protobuf
message GetLatestBlockRequest {
    string graph_id = 1;
}
```

**Response:**
```protobuf
message GetBlockResponse {
    bool success = 1;
    string message = 2;
    Block block = 3;
}
```

---

### GetGraphInfo

Returns detailed information about a specific graph.

**Request:**
```protobuf
message GetGraphInfoRequest {
    string graph_id = 1;
}
```

**Response:**
```protobuf
message GetGraphInfoResponse {
    bool success = 1;
    string graph_id = 2;
    GraphType graph_type = 3;
    uint64 total_blocks = 4;
    string latest_hash = 5;
    int64 created_at = 6;
    bool is_valid = 7;
}
```

---

### VerifyGraph

Validates the integrity of a specific graph.

**Request:**
```protobuf
message VerifyGraphRequest {
    string graph_id = 1;
}
```

**Response:**
```protobuf
message VerifyGraphResponse {
    bool success = 1;
    bool is_valid = 2;
    string message = 3;
    repeated string errors = 4;
}
```

**Validation Checks:**
- Chain integrity (previous hash linking)
- Block hash validity
- Proof of work difficulty
- Height sequence

---

### CrossValidateGraphs

Performs cross-validation across all graphs in the network.

**Request:**
```protobuf
message CrossValidateRequest {}
```

**Response:**
```protobuf
message CrossValidateResponse {
    bool success = 1;
    bool all_valid = 2;
    string message = 3;
    map<string, bool> graph_statuses = 4;  // Graph ID -> Valid/Invalid
}
```

**Validation Process:**
1. Validates each graph individually
2. Checks cross-references between graphs
3. Ensures referenced blocks exist

---

### ListGraphs

Lists all available graphs in the system.

**Request:**
```protobuf
message ListGraphsRequest {}
```

**Response:**
```protobuf
message ListGraphsResponse {
    repeated GraphInfo graphs = 1;
}

message GraphInfo {
    string graph_id = 1;
    GraphType graph_type = 2;
    uint64 total_blocks = 3;
    string description = 4;
}
```

---

### GetBlockRange

Retrieves a range of blocks by height.

**Request:**
```protobuf
message GetBlockRangeRequest {
    string graph_id = 1;
    uint64 start_height = 2;
    uint64 end_height = 3;
}
```

**Response:**
```protobuf
message GetBlockRangeResponse {
    bool success = 1;
    repeated Block blocks = 2;
}
```

---

## Data Types

### Block

```protobuf
message Block {
    string hash = 1;                        // SHA-256 hash
    string previous_hash = 2;               // Link to previous block
    int64 timestamp = 3;                    // Unix timestamp
    uint64 nonce = 4;                       // Proof of work nonce
    string data = 5;                        // Payload (JSON, binary, etc)
    uint64 height = 6;                      // Block position in chain
    string graph_id = 7;                    // Parent graph
    repeated string cross_references = 8;   // References to other graphs
}
```

### GraphType

```protobuf
enum GraphType {
    TRANSACTION = 0;  // Financial transactions
    IDENTITY = 1;     // Identity/authentication data
    ASSET = 2;        // Asset ownership
    AUDIT = 3;        // Audit logs
    CUSTOM = 4;       // Custom application data
}
```

---

## Error Handling

All RPC methods return responses with `success` boolean. Check this field:

```rust
if response.success {
    // Operation succeeded
} else {
    // Handle error, check response.message
}
```

Common error scenarios:
- **Graph not found**: Graph ID doesn't exist
- **Invalid previous hash**: Block chain broken
- **Invalid difficulty**: Proof of work not satisfied
- **Persistence failure**: Database error

---

## C# Client Example

```csharp
using Grpc.Net.Client;
using Blockchain;

public class BlockchainClient
{
    private readonly BlockchainService.BlockchainServiceClient _client;

    public BlockchainClient(string address)
    {
        var channel = GrpcChannel.ForAddress(address);
        _client = new BlockchainService.BlockchainServiceClient(channel);
    }

    public async Task<CreateGraphResponse> CreateGraphAsync(
        string graphId, 
        GraphType type, 
        string description)
    {
        var request = new CreateGraphRequest
        {
            GraphId = graphId,
            GraphType = type,
            Description = description
        };

        return await _client.CreateGraphAsync(request);
    }

    public async Task<AddBlockResponse> AddBlockAsync(
        string graphId, 
        string data, 
        params string[] crossReferences)
    {
        var request = new AddBlockRequest
        {
            GraphId = graphId,
            Data = data
        };
        request.CrossReferences.AddRange(crossReferences);

        return await _client.AddBlockAsync(request);
    }

    public async Task<GetGraphInfoResponse> GetGraphInfoAsync(string graphId)
    {
        var request = new GetGraphInfoRequest { GraphId = graphId };
        return await _client.GetGraphInfoAsync(request);
    }

    public async Task<CrossValidateResponse> CrossValidateAsync()
    {
        var request = new CrossValidateRequest();
        return await _client.CrossValidateGraphsAsync(request);
    }
}

// Usage
var client = new BlockchainClient("http://localhost:50051");

// Create graph
await client.CreateGraphAsync(
    "transactions", 
    GraphType.Transaction, 
    "Financial transactions"
);

// Add block
var response = await client.AddBlockAsync(
    "transactions",
    JsonSerializer.Serialize(new { from = "Alice", to = "Bob", amount = 100 })
);

// Validate
var validation = await client.CrossValidateAsync();
Console.WriteLine($"All graphs valid: {validation.AllValid}");
```

---

## Python Client Example

```python
import grpc
from blockchain_pb2 import *
from blockchain_pb2_grpc import BlockchainServiceStub

class BlockchainClient:
    def __init__(self, address='localhost:50051'):
        self.channel = grpc.insecure_channel(address)
        self.stub = BlockchainServiceStub(self.channel)
    
    def create_graph(self, graph_id, graph_type, description):
        request = CreateGraphRequest(
            graph_id=graph_id,
            graph_type=graph_type,
            description=description
        )
        return self.stub.CreateGraph(request)
    
    def add_block(self, graph_id, data, cross_refs=None):
        request = AddBlockRequest(
            graph_id=graph_id,
            data=data,
            cross_references=cross_refs or []
        )
        return self.stub.AddBlock(request)
    
    def verify_graph(self, graph_id):
        request = VerifyGraphRequest(graph_id=graph_id)
        return self.stub.VerifyGraph(request)

# Usage
client = BlockchainClient()

# Create graph
response = client.create_graph(
    'transactions',
    GraphType.TRANSACTION,
    'Financial transactions'
)

# Add block
import json
data = json.dumps({'from': 'Alice', 'to': 'Bob', 'amount': 100})
block_response = client.add_block('transactions', data)

print(f"Block hash: {block_response.block.hash}")
```
