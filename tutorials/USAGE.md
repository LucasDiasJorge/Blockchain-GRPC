# USAGE: cURL, Postman e gRPC

Este documento mostra como interagir com o serviço usando:
- gRPC (via grpcurl)
- HTTP/JSON (via cURL/Postman) — usando o proxy `http_proxy` incluído no projeto

> O servidor principal roda gRPC em `0.0.0.0:50051`. Para usar cURL/Postman sem gRPC, rode o proxy HTTP (`http_proxy`) que converte JSON → gRPC.

---

## 1) gRPC diretamente (grpcurl)

Instalação no Ubuntu/WSL:
```bash
sudo apt update
sudo apt install -y grpcurl
```

Exemplos:

- Listar serviços:
```bash
grpcurl -plaintext localhost:50051 list
```

- Listar métodos do serviço:
```bash
grpcurl -plaintext localhost:50051 list blockchain.BlockchainService
```

- Listar grafos:
```bash
grpcurl -plaintext -d '{}' localhost:50051 blockchain.BlockchainService/ListGraphs
```

- Info de um grafo:
```bash
grpcurl -plaintext -d '{"graph_id":"transactions"}' localhost:50051 blockchain.BlockchainService/GetGraphInfo
```

- Último bloco:
```bash
grpcurl -plaintext -d '{"graph_id":"transactions"}' localhost:50051 blockchain.BlockchainService/GetLatestBlock
```

- Intervalo de blocos:
```bash
grpcurl -plaintext -d '{"graph_id":"transactions","start_height":0,"end_height":5}' \
  localhost:50051 blockchain.BlockchainService/GetBlockRange
```

- Adicionar bloco:
```bash
grpcurl -plaintext -d '{
  "graph_id": "transactions",
  "data": "{\"from\":\"Alice\",\"to\":\"Bob\",\"amount\":100}",
  "cross_references": []
}' localhost:50051 blockchain.BlockchainService/AddBlock
```

---

## 2) HTTP/JSON com cURL e Postman (via proxy)

O binário `http_proxy` expõe endpoints HTTP que encaminham as chamadas para o servidor gRPC. Assim, você pode usar cURL e Postman com JSON puro.

### 2.1 Rodando o proxy

Em um terminal, com o servidor gRPC rodando (porta 50051), rode:
```bash
# Opcional: customizar endereços
export GRPC_ADDR=http://127.0.0.1:50051
export HTTP_ADDR=0.0.0.0:8080

# Rodar o proxy
cargo run --bin http_proxy
```

Agora, você tem HTTP em `http://localhost:8080` apontando para gRPC `http://127.0.0.1:50051`.

### 2.2 Endpoints HTTP

- Criar grafo (POST)
```
POST /graphs
Content-Type: application/json
{
  "graph_id": "transactions",
  "graph_type": 0,
  "description": "Financial transactions"
}
```
Exemplo cURL:
```bash
curl -sS -X POST http://localhost:8080/graphs \
  -H 'Content-Type: application/json' \
  -d '{"graph_id":"transactions","graph_type":0,"description":"Financial transactions"}' | jq
```

- Listar grafos (GET)
```
GET /graphs
```
Exemplo cURL:
```bash
curl -sS http://localhost:8080/graphs | jq
```

- Info do grafo (GET)
```
GET /graphs/{graph_id}
```
Exemplo cURL:
```bash
curl -sS http://localhost:8080/graphs/transactions | jq
```

- Verificar grafo (POST)
```
POST /graphs/{graph_id}/verify
```
Exemplo cURL:
```bash
curl -sS -X POST http://localhost:8080/graphs/transactions/verify | jq
```

- Validação cruzada (POST)
```
POST /graphs/verify
```
Exemplo cURL:
```bash
curl -sS -X POST http://localhost:8080/graphs/verify | jq
```

- Adicionar bloco (POST)
```
POST /graphs/{graph_id}/blocks
Content-Type: application/json
{
  "data": "{\"from\":\"Alice\",\"to\":\"Bob\",\"amount\":100}",
  "cross_references": []
}
```
Exemplo cURL:
```bash
curl -sS -X POST http://localhost:8080/graphs/transactions/blocks \
  -H 'Content-Type: application/json' \
  -d '{"data":"{\"from\":\"Alice\",\"to\":\"Bob\",\"amount\":100}","cross_references":[]}' | jq
```

- Último bloco (GET)
```
GET /graphs/{graph_id}/blocks/latest
```
Exemplo cURL:
```bash
curl -sS http://localhost:8080/graphs/transactions/blocks/latest | jq
```

- Intervalo de blocos (GET com query)
```
GET /graphs/{graph_id}/blocks?start_height=0&end_height=5
```
Exemplo cURL:
```bash
curl -sS "http://localhost:8080/graphs/transactions/blocks?start_height=0&end_height=5" | jq
```

> Observação: o campo `data` nos blocos é uma string; se quiser enviar JSON como payload, envie-o como string escapada (exemplos acima).

---

## 3) Brief da Implementação

- Projeto em Rust com **Tonic (gRPC)**, **RocksDB** para persistência e **arquitetura multi-graph**.
- Cada grafo é uma blockchain separada com propósito específico (Transaction, Identity, Asset, Audit, Custom).
- Blocos têm **hash SHA-256**, **proof-of-work** (dificuldade configurável), vínculo `previous_hash` e **cross-references** entre grafos.
- **Repository Pattern** para acesso a dados (RocksDB), **Strategy Pattern** para validações, **Factory Pattern** para criação de blocos/grafos, e **Adapter Pattern** para a camada de persistência.
- `http_proxy` (Axum) é um binário opcional que encaminha JSON/HTTP → gRPC usando o cliente gerado pelo Tonic, permitindo uso de cURL/Postman.

Arquivos-chave:
- `proto/blockchain.proto` — definições gRPC
- `src/infrastructure/grpc/server.rs` — servidor gRPC
- `src/infrastructure/persistence/` — RocksDB + Repository
- `src/application/services/` — orquestração e casos de uso
- `src/domain/` — entidades e regras de negócio
- `src/bin/http_proxy.rs` — proxy HTTP/JSON opcional

---

## 4) Dicas

- Para produção, considere TLS no gRPC e autenticação (roadmap futuro).
- Use `grpcurl` para debugar rapidamente; use `http_proxy` se preferir Postman/REST.
- Logs: defina `RUST_LOG=info` para ver eventos do servidor/proxy.