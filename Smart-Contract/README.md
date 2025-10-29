# Smart-Contract REST → gRPC Bridge (.NET 8)

REST API em ASP.NET Core que consome a API gRPC definida em `proto/blockchain.proto` e expõe endpoints HTTP equivalentes.

## Requisitos
- .NET SDK 8+
- Servidor gRPC externo disponível (ex.: `http://127.0.0.1:50051`)

## Configuração
Edite `appsettings.json`:
```json
{
  "Grpc": {
    "Endpoint": "http://127.0.0.1:50051"
  }
}
```

## Rodando
```powershell
# Na pasta Smart-Contract
dotnet restore
dotnet build -c Release
dotnet run -c Release
```
A API inicia em `http://localhost:5199` (porta dinâmica). Acesse Swagger em `/swagger`.

## Endpoints Principais
- POST `/api/graphs` → CreateGraph
- GET `/api/graphs` → ListGraphs
- GET `/api/graphs/{graphId}` → GetGraphInfo
- POST `/api/graphs/{graphId}/verify` → VerifyGraph
- POST `/api/graphs/verify` → CrossValidateGraphs
- POST `/api/graphs/{graphId}/blocks` → AddBlock
- GET `/api/graphs/{graphId}/blocks` → GetBlockRange (`startHeight`, `endHeight`)
- GET `/api/graphs/{graphId}/blocks/latest` → GetLatestBlock
- GET `/api/graphs/{graphId}/blocks/{hash}` → GetBlock

## Exemplos (PowerShell)
```powershell
# Criar graph
Invoke-RestMethod -Method Post -Uri "http://localhost:5199/api/graphs" -ContentType 'application/json' -Body '{
  "graphId":"g1",
  "graphType":0,
  "description":"transacoes"
}'

# Listar graphs
Invoke-RestMethod -Method Get -Uri "http://localhost:5199/api/graphs"

# Adicionar bloco
Invoke-RestMethod -Method Post -Uri "http://localhost:5199/api/graphs/g1/blocks" -ContentType 'application/json' -Body '{
  "data":"payload",
  "crossReferences":["abc","def"]
}'

# Bloco mais recente
Invoke-RestMethod -Method Get -Uri "http://localhost:5199/api/graphs/g1/blocks/latest"
```

## Observações
- O client gRPC é gerado diretamente a partir de `../proto/blockchain.proto` via `Grpc.Tools`.
- Tratamento de erros: `RpcException` é traduzida para códigos HTTP equivalentes (400/404/503/500).
- Código modular com DI e serviços separados.
