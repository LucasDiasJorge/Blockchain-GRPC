# Blockchain Multi-Graph com gRPC

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![gRPC](https://img.shields.io/badge/gRPC-4285F4?style=for-the-badge&logo=google&logoColor=white)

Uma implementação de blockchain enterprise em Rust com arquitetura de múltiplos grafos, interface gRPC e persistência completa de dados. Projetado para alta performance e integração com sistemas externos via API.

## 🎯 Características Principais

### Arquitetura Multi-Graph Blockchain
- **Múltiplos Grafos Independentes**: Cada grafo é uma blockchain separada com responsabilidade específica
- **Tipos de Grafos Especializados**:
  - `Transaction`: Transações financeiras
  - `Identity`: Dados de identidade e autenticação
  - `Asset`: Propriedade e transferência de ativos
  - `Audit`: Logs de auditoria e compliance
  - `Custom`: Dados específicos da aplicação

### Validação Cruzada (Cross-Validation)
- Cada grafo pode referenciar blocos de outros grafos
- Sistema de validação cruzada para integridade da rede
- Grafos funcionam de forma independente mas se verificam mutuamente

### Performance e Persistência
- **Persistência com RocksDB**: Dados nunca são perdidos, mesmo após reinicialização
- **Cache em Memória**: Performance otimizada com cache inteligente
- **Proof of Work**: Mineração de blocos com dificuldade configurável
- **Serialização Binária**: Bincode para máxima eficiência

### Arquitetura Clean Code & SOLID
- **Domain-Driven Design**: Separação clara entre domínio, aplicação e infraestrutura
- **Repository Pattern**: Abstração da camada de persistência
- **Strategy Pattern**: Múltiplas estratégias de validação
- **Factory Pattern**: Criação elegante de blocos e grafos
- **Dependency Injection**: Baixo acoplamento entre componentes

## 🏗️ Arquitetura

```
blockchain-grpc/
├── domain/              # Entidades de domínio e regras de negócio
│   ├── block.rs         # Estrutura de bloco
│   ├── graph.rs         # Blockchain graph
│   ├── transaction.rs   # Transações
│   └── traits.rs        # Interfaces (Repository, Validation, etc)
│
├── application/         # Casos de uso e serviços de aplicação
│   ├── services/        # Serviços de aplicação
│   └── use_cases/       # Casos de uso específicos
│
├── infrastructure/      # Implementações técnicas
│   ├── persistence/     # RocksDB e Repository
│   └── grpc/           # Servidor gRPC
│
└── config/             # Configurações da aplicação
```

### Princípios SOLID Aplicados

1. **Single Responsibility**: Cada módulo tem uma responsabilidade única e bem definida
2. **Open/Closed**: Extensível através de traits, fechado para modificação
3. **Liskov Substitution**: Todas as implementações de traits são substituíveis
4. **Interface Segregation**: Traits pequenas e focadas
5. **Dependency Inversion**: Dependências através de abstrações (traits)

## 🚀 Instalação e Uso

### Pré-requisitos

- Rust 1.70+ (instale via [rustup](https://rustup.rs/))
- Protocol Buffers Compiler (`protoc`)
- Linux/WSL (otimizado para ambientes Unix)

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y protobuf-compiler libprotobuf-dev

# Fedora
sudo dnf install protobuf-compiler protobuf-devel
```

### Build

```bash
# Clone o repositório
git clone https://github.com/LucasDiasJorge/Blockchain-GRPC.git
cd Blockchain-GRPC

# Build do projeto
cargo build --release

# Executar testes
cargo test

# Executar com logs detalhados
RUST_LOG=info cargo run
```

### Configuração

O projeto cria automaticamente um arquivo `config.json` com configurações padrão:

```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 50051
  },
  "blockchain": {
    "default_difficulty": 2,
    "max_block_size": 1048576
  },
  "storage": {
    "data_dir": "./data/blockchain"
  }
}
```

### Executando o Servidor

```bash
cargo run --release
```

O servidor gRPC estará disponível em `0.0.0.0:50051`

## 📡 API gRPC

### Criar um Novo Grafo

```protobuf
rpc CreateGraph(CreateGraphRequest) returns (CreateGraphResponse);

message CreateGraphRequest {
    string graph_id = 1;
    GraphType graph_type = 2;
    string description = 3;
}
```

### Adicionar Bloco

```protobuf
rpc AddBlock(AddBlockRequest) returns (AddBlockResponse);

message AddBlockRequest {
    string graph_id = 1;
    string data = 2;
    repeated string cross_references = 3;
}
```

### Obter Informações do Grafo

```protobuf
rpc GetGraphInfo(GetGraphInfoRequest) returns (GetGraphInfoResponse);
```

### Validar Grafo

```protobuf
rpc VerifyGraph(VerifyGraphRequest) returns (VerifyGraphResponse);
```

### Validação Cruzada de Todos os Grafos

```protobuf
rpc CrossValidateGraphs(CrossValidateRequest) returns (CrossValidateResponse);
```

### Listar Todos os Grafos

```protobuf
rpc ListGraphs(ListGraphsRequest) returns (ListGraphsResponse);
```

## 🔗 Integração com C#

Este projeto foi projetado para funcionar como a camada de **Repository/Infrastructure** em uma arquitetura onde uma API C# atua como **Domain/Presentation** (Commander).

### Exemplo de Cliente C#

```csharp
using Grpc.Net.Client;
using Blockchain;

// Conectar ao servidor Rust
var channel = GrpcChannel.ForAddress("http://localhost:50051");
var client = new BlockchainService.BlockchainServiceClient(channel);

// Criar um grafo
var createResponse = await client.CreateGraphAsync(new CreateGraphRequest
{
    GraphId = "transactions",
    GraphType = GraphType.Transaction,
    Description = "Financial transactions"
});

// Adicionar um bloco
var addResponse = await client.AddBlockAsync(new AddBlockRequest
{
    GraphId = "transactions",
    Data = JsonSerializer.Serialize(new Transaction
    {
        From = "Alice",
        To = "Bob",
        Amount = 100.0
    })
});

// Validar toda a rede
var validateResponse = await client.CrossValidateGraphsAsync(
    new CrossValidateRequest()
);
```

### Padrão de Integração Recomendado

```
┌─────────────────────────────────────┐
│         API C# (Commander)          │
│  - Controllers                      │
│  - Domain Logic                     │
│  - Business Rules                   │
│  - Presentation Layer               │
└──────────────┬──────────────────────┘
               │ gRPC
               │
┌──────────────▼──────────────────────┐
│    Rust Blockchain (Repository)    │
│  - Data Persistence                 │
│  - Blockchain Logic                 │
│  - Cross-Validation                 │
│  - Infrastructure Layer             │
└─────────────────────────────────────┘
```

## 🧪 Testes

O projeto inclui testes unitários e de integração:

```bash
# Rodar todos os testes
cargo test

# Testes com output detalhado
cargo test -- --nocapture

# Testes de um módulo específico
cargo test domain::block
```

## 📊 Performance

### Benchmarks (em ambiente WSL2 - Ryzen 5 5600X)

- **Mineração de Bloco** (dificuldade 2): ~1-5ms
- **Validação de Chain** (1000 blocos): ~50ms
- **Persistência RocksDB**: ~0.5ms por bloco
- **Throughput gRPC**: ~10,000 requisições/s

### Otimizações Aplicadas

- Serialização binária com Bincode
- Cache em memória com RwLock
- Índices de hash para busca O(1)
- Compilação com LTO e otimizações máximas

## 🔒 Segurança

- **SHA-256**: Hash criptográfico para blocos
- **Proof of Work**: Proteção contra alterações maliciosas
- **Validação Cruzada**: Múltiplos grafos se verificam mutuamente
- **Imutabilidade**: Estruturas de dados imutáveis por design

⚠️ **Nota**: Autenticação será implementada em versões futuras

## 🛠️ Desenvolvimento

### Estrutura de Commits

- `feat:` Nova funcionalidade
- `fix:` Correção de bug
- `refactor:` Refatoração de código
- `test:` Adição de testes
- `docs:` Documentação

### Roadmap

- [x] Implementação base da blockchain
- [x] Sistema de múltiplos grafos
- [x] Persistência com RocksDB
- [x] Interface gRPC
- [x] Validação cruzada de grafos
- [ ] Autenticação e autorização
- [ ] API REST complementar
- [ ] Métricas e monitoring (Prometheus)
- [ ] Smart contracts básicos
- [ ] Consenso distribuído (Raft/PBFT)
- [ ] Cliente CLI para testes
- [ ] Dashboard web para visualização

## 📝 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes

## 👤 Autor

**Lucas Jorge**

- GitHub: [@LucasDiasJorge](https://github.com/LucasDiasJorge)

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se livre para abrir issues e pull requests.

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

---

⭐ Se este projeto foi útil, considere dar uma estrela!