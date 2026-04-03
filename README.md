# Blockchain gRPC

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![gRPC](https://img.shields.io/badge/gRPC-4285F4?style=for-the-badge&logo=google&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=tokio&logoColor=white)
![RocksDB](https://img.shields.io/badge/RocksDB-1D4ED8?style=for-the-badge)
![Cargo](https://img.shields.io/badge/Cargo-%23dea584.svg?style=for-the-badge&logo=rust&logoColor=white)

> README definitivo para dominar Rust em camadas: comece pelos fundamentos, avance para arquitetura profissional e aplique tudo no projeto `Blockchain-GRPC`.

## 🧭 Índice

- [Visão Geral do Projeto](#-visão-geral-do-projeto)
- [Requisitos do Ambiente](#-requisitos-do-ambiente)
- [Arquitetura do Repositório](#-arquitetura-do-repositório)
- [Mapa Intensivo de Estudos](#-mapa-intensivo-de-estudos)
- [Fundamentos Essenciais de Rust](#-fundamentos-essenciais-de-rust)
- [Ownership, Borrowing e Lifetimes](#-ownership-borrowing-e-lifetimes)
- [Tipos Compostos, Iteradores e Closures](#-tipos-compostos-iteradores-e-closures)
- [Traits, Generics e Padrões de Projeto](#-traits-generics-e-padrões-de-projeto)
- [Concorrência, Async e gRPC](#-concorrência-async-e-grpc)
- [Persistência, Serialização e RocksDB](#-persistência-serialização-e-rocksdb)
- [Fluxo gRPC ponta a ponta](#-fluxo-grpc-ponta-a-ponta)
- [Ferramentas Cargo, Build e Observabilidade](#-ferramentas-cargo-build-e-observabilidade)
- [Testes, Integração e Qualidade](#-testes-integração-e-qualidade)
- [Laboratórios Guiados](#-laboratórios-guiados)
- [Diagnóstico e Debug](#-diagnóstico-e-debug)
- [Checklist de Estudos](#-checklist-de-estudos)
- [Perguntas Frequentes](#-perguntas-frequentes)
- [Glossário de Rust](#-glossário-de-rust)
- [Recursos Complementares](#-recursos-complementares)

## 🚀 Visão Geral do Projeto

- Blockchain multi-grafo escrita em Rust, com interface gRPC e arquitetura de domínio explícita.
- Modulagem em camadas: `domain` (regras puras), `application` (orquestração), `infrastructure` (gRPC + storage), `config` (configurações).
- Persistência durável com RocksDB, serialização binária e validação cruzada entre grafos.
- Servidor assíncrono com `tokio`/`tonic`, prontos para workloads de alta concorrência.
- Build script `build.rs` integra geração de código protobuf automaticamente durante `cargo build`.

```rust
// src/main.rs — ponto de entrada com DI simples e runtime assíncrona
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt()
    .with_env_filter(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
    )
    .init();

  tracing::info!("🔗 Starting Blockchain gRPC Service");

  let settings = Settings::load("config.json")?;
  let db = Arc::new(RocksDbAdapter::new(&settings.storage.data_dir)?);
  let repository = Arc::new(BlockchainRepositoryImpl::new(db));
  let service = Arc::new(BlockchainServiceImpl::new(repository));
  service.initialize().await?;
  blockchain_grpc::start_grpc_server(service, settings.server_address()).await?;
  Ok(())
}
```

## 🧰 Requisitos do Ambiente

- Rust 1.70+ (`rustup update stable` garante uma toolchain moderna).
- `protoc` ≥ 3.17 para gerar código gRPC via `tonic-build`.
- Windows, Linux ou macOS; recomendado WSL2 ou Linux para scripts shell.
- Ferramentas extras úteis: `just`, `make`, Docker, VS Code com extensão Rust Analyzer.

### Instalação rápida (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev protobuf-compiler
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🗃️ Arquitetura do Repositório

```
.
├── code/                 # Crate Rust principal (blockchain-grpc)
│   ├── src/              # Código de domínio, aplicação, infraestrutura e config
│   ├── tests/            # Testes de integração async
│   ├── examples/         # Exemplos (ex.: client gRPC)
│   ├── proto/            # Definições protobuf do serviço gRPC
│   ├── build.rs          # Geração automática de código gRPC
│   └── Cargo.toml        # Manifesto do crate Rust
│
├── docs/                 # Documentação de arquitetura, API, bugfixes, etc.
├── tutorials/            # Guias de estudo, quickstarts e material didático
├── scripts/              # Scripts de build, setup, execução e testes gRPC
├── Cargo.toml            # Workspace Cargo (agrupa o crate em code/)
├── config.json           # Configuração padrão da aplicação
└── README.md             # Este guia principal
```

### Roteiro de estudo da estrutura

1. Abra `src/lib.rs` e observe os `pub use` que definem a API pública.
2. Percorra `src/domain` para entender modelos (`Block`, `BlockchainGraph`) e contratos (`BlockchainRepository`).
3. Estude `src/application/services/blockchain_service.rs` para ver orquestração com `RwLock`.
4. Explore `src/infrastructure/grpc/server.rs` e o código gerado em `src/infrastructure/grpc/blockchain`.
5. Inspecione `build.rs` para entender geração de código a partir do `.proto`.

## 🧠 Mapa Intensivo de Estudos

- **Fundamentos**: sintaxe, tipos primitivos, controle de fluxo, ownership, erro.
- **Intermediário**: structs, enums, traits, pattern matching, módulos e organização.
- **Avançado**: async/await, concorrência, `Arc`/`RwLock`, streaming gRPC.
- **Especialização**: persistência com RocksDB, integração gRPC, interoperabilidade C#.
- **Mastery**: otimização, profiling, tracing distribuído, customizações do build.

Use cada seção do README como aula. Os laboratórios no final consolidam os conhecimentos.

## 🧱 Fundamentos Essenciais de Rust

### 1. `main` assíncrono e retorno de `Result`
- Funções `main` podem devolver `Result` para propagação de erro limpa.
- A macro `#[tokio::main]` cria um runtime; evite blocos `tokio::runtime::Runtime::new()` manuais.

### 2. Variáveis, mutabilidade e inferência
- Variáveis são imutáveis por padrão (`let x = 5`).
- Use `let mut` para permitir mudanças locais, como o `nonce` ao minerar blocos.

```rust
let mut nonce: u64 = 0;
while !self.hash.starts_with(&target) {
  nonce += 1;
}
```

### 3. Sombras e escopos
- `let hash = hash.to_uppercase();` cria nova binding com mesmo nome, útil para pipelines.
- Escopos são delimitados por `{}` e garantem drop determinístico de recursos.

### 4. Ownership na prática (`Block::new`)
- Cada valor tem um dono; ao passar `String`, movemos a posse.
- Clonagem acontece apenas quando necessário para manter dados originais.

```rust
pub fn new(previous_hash: String, data: String, graph_id: String, height: u64, cross_references: Vec<String>) -> Self {
  let mut block = Self {
    hash: String::new(),
    previous_hash: previous_hash.clone(),
    timestamp: Utc::now().timestamp(),
    data: data.clone(),
    nonce: 0,
    height,
    graph_id: graph_id.clone(),
    cross_references,
  };
  block.hash = block.calculate_hash();
  block
}
```

### 5. Borrowing e referências
- `&T` permite leitura concorrente; `&mut T` garante exclusividade de escrita.
- `BlockchainGraph::get_latest_block` devolve `Option<&Block>`, evitando cópias.

### 6. `Option` e `Result`
- Use `match`, `if let` ou os métodos `map`, `unwrap_or` para manipular.
- `?` propaga erros automaticamente, reduzindo boilerplate em `async fn`.

### 7. Pattern matching exaustivo
- `match` exige cobrir todos os casos, reduzindo bugs em tempo de compilação.
- `GraphType::from_i32` mantém compatibilidade com enums gerados pelo protobuf.

### 8. Enums ricos
- Enums podem conter dados (`enum Message { Text(String), Binary(Vec<u8>) }`).
- Métodos `impl` auxiliam na conversão e comportamento (ver `GraphType::to_i32`).

### 9. Structs e métodos
- `impl_struct` agrupa comportamento com dados.
- `Block::mine_block` demonstra método que modifica estado interno com `&mut self`.

### 10. Traits como interfaces
- `BlockchainRepository` usa `async_trait` garantindo assinatura assíncrona uniforme.
- Traits podem herdar `Send + Sync` para segurança em threads.

### 11. Generics e tipos associados
- Traits como `HashCalculator` permitem diferentes algoritmos de hash.
- Favor padrões `dyn Trait` quando objeto concreto não importa em tempo de compilação.

### 12. Iteradores, adapters e coleções
- Métodos como `.iter().map(...).collect::<Vec<_>>()` tornam pipelines declarativos.
- `graph.validate_cross_references` usa `any` e `for` combinados para clareza.

### 13. Lifetimes implícitas
- O compilador infere lifetimes na maioria dos casos.
- Em APIs complexas, anote como `fn foo<'a>(x: &'a str) -> &'a str` para expressar relações.

### 14. `if let`, `while let` e destructuring
- Sintaxes que reduzem verbosidade ao lidar com enums.
- Exemplos em `application/services` ao extrair mensagens das requests gRPC.

### 15. Erros com `thiserror`/`anyhow` (extensões sugeridas)
- Atual projeto usa `Box<dyn Error>` para simplicidade.
- Evolua para `thiserror` (erros estruturados) ou `anyhow` (erros dinâmicos com contexto) conforme crescer.

### 16. Logging com `tracing`
- Spans e eventos estruturados (`tracing::info!`, `tracing::error!`).
- Integre com `tracing-subscriber` e exportadores OpenTelemetry para observabilidade distribuída.

### 17. Serialização com `serde`
- `#[derive(Serialize, Deserialize)]` em `Block`, `BlockchainGraph`, `Settings`.
- `serde_json` carrega/salva configs; `bincode` pode ser usado para persistir dados binários compactos.

### 18. Crates externos e versionamento
- `Cargo.toml` lista dependências com versões (`tokio = { version = "1", features = ["full"] }`).
- `cargo update` sincroniza lockfile; `cargo tree` inspeciona dependências.

### 19. Testes unitários em linha
- Módulos `#[cfg(test)]` próximos ao código facilitam manutenção.
- Use `cargo test domain::block` para filtrar.

### 20. Workspaces e reexport com `pub use`
- `src/lib.rs` reexporta símbolos-chave, simplificando consumo por binários externos.
- Estrutura ideal para transformar o projeto em crate reutilizável.

### 21. Closures e funções de ordem superior
- Use closures em iteradores ou handlers (ex.: `blocks.iter().map(|b| ...)`).
- Closures capturam ambiente por referência ou movimento (`move ||`), útil ao spawnar tasks em `tokio`.

### 22. Arrays, slices e fat pointers
- Slices `&[u8]` são onipresentes (como nos métodos `RocksDbAdapter::put`).
- Entenda que `&[u8]` inclui ponteiro + tamanho, evitando `strlen` em runtime.

### 23. Coleções standard (`Vec`, `HashMap`, `BTreeMap`)
- `HashMap<String, BlockchainGraph>` armazena grafos carregados em memória.
- Use `entry` API para inserções condicionais e `retain` para limpeza eficiente.

### 24. Guards e pattern matching avançado
- `match x { 0..=10 => ..., _ if cond => ... }` cria filtros legíveis.
- Pratique convertendo validações complexas em pattern matching.

### 25. Macros declarativas
- `println!`, `format!`, `tracing::info!` são macros.
- Crie macros customizadas para padronizar logs (ex.: `macro_rules! audit { ... }`).

### 26. Módulos, privacidade e `pub(crate)`
- Controle exportações com `pub(crate)` para expor apenas dentro da crate.
- Arquivos `mod.rs` permitem agrupar submódulos (`application/services/mod.rs`).

### 27. Build script `build.rs`
- Executa antes da compilação; aqui gera código gRPC com `tonic_build`.

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .compile(&["proto/blockchain.proto"], &["proto"])?;
  Ok(())
}
```

### 28. Feature flags e `cfg` (expansão futura)
- Utilize `[features]` no `Cargo.toml` para habilitar partes opcionais (ex.: `rocksdb` vs. `sled`).
- `#[cfg(feature = "experimental")]` permite código condicional.

### 29. Condicional de plataforma
- `#[cfg(target_os = "windows")]` habilita integrações específicas (ex.: caminhos para PowerShell).
- Útil para suportar múltiplos ambientes sem código duplicado.

### 30. Documentação com `rustdoc`
- Comentários `///` geram docs navegáveis (`cargo doc --open`).
- Inclua exemplos executáveis nos comentários para garantir que a doc compila.

## 🔐 Ownership, Borrowing e Lifetimes

- `Arc<T>` permite múltiplos donos compartilharem dados; clone barato (incrementa contador).
- `tokio::sync::RwLock` oferece múltiplas leituras simultâneas e escrita exclusiva.
- Lifetimes são implícitas nas estruturas do projeto, mas compreender regras é essencial para evoluir.
- `Send`/`Sync` certificam que tipos podem ser movidos ou compartilhados entre threads; `Arc<RwLock<...>>` implementa ambos.

## 🧬 Tipos Compostos, Iteradores e Closures

- `Vec<T>` é coleção básica; usada para armazenar cadeia de blocos em memória (`Vec<Block>`).
- `HashMap` guarda grafos carregados (`HashMap<String, BlockchainGraph>` em `BlockchainServiceImpl`).
- Iteradores (`iter`, `map`, `filter`, `collect`) deixam código declarativo e eficiente.
- Closures com `move` são úteis ao criar tasks: `tokio::spawn(async move { ... })`.

## 🧩 Traits, Generics e Padrões de Projeto

- **Repository Pattern**: `BlockchainRepository` abstrai detalhes de persistência.
- **Strategy Pattern**: `ValidationStrategy` (extensível) para validações alternativas.
- **Factory Method**: `Block::new` e `BlockchainGraph::new` retornam objetos prontos.
- **Adapter Pattern**: `RocksDbAdapter` encapsula bibliotecas externas.
- **Dependency Injection**: passada por `Arc<dyn Trait>` no construtor de `BlockchainServiceImpl`.

```rust
pub struct BlockchainServiceImpl {
  repository: Arc<dyn BlockchainRepository>,
  graphs: Arc<RwLock<HashMap<String, BlockchainGraph>>>,
}

impl BlockchainServiceImpl {
  pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
    let graphs = self.repository.list_graphs().await?;
    let mut graph_map = self.graphs.write().await;
    for mut graph in graphs {
      if let Some(latest) = self.repository.get_latest_block(&graph.id).await? {
        let blocks = self.repository.get_blocks_range(&graph.id, 0, latest.height).await?;
        graph.load_blocks(blocks);
      }
      graph_map.insert(graph.id.clone(), graph);
    }
    Ok(())
  }
}
```

## ⚙️ Concorrência, Async e gRPC

- `tokio` fornece runtime cooperativo; tasks são agendadas no mesmo thread pool.
- `tonic` gera server/client gRPC; código gerado fica em `src/infrastructure/grpc/blockchain`.
- `#[tonic::async_trait]` simplifica implementação de interfaces gRPC.
- `Arc<RwLock<_>>` gerencia estado compartilhado entre requisições (`graphs` em memória).
- Fluxo típico: request gRPC → `BlockchainServiceImpl` → camada de domínio → persistência.

```rust
#[tonic::async_trait]
impl BlockchainService for BlockchainServiceImpl {
  async fn add_block(&self, request: Request<AddBlockRequest>) -> Result<Response<AddBlockResponse>, Status> {
    let req = request.into_inner();
    self.handle_add_block(req).await
  }

  async fn cross_validate_graphs(&self, _request: Request<CrossValidateRequest>) -> Result<Response<CrossValidateResponse>, Status> {
    self.handle_cross_validate().await
  }
}
```

## 💾 Persistência, Serialização e RocksDB

- `RocksDbAdapter` encapsula operações de baixo nível (`put`, `get`, `get_keys_with_prefix`).
- Armazenamento organizado por prefixos (`graph:{id}`, `block:{graph}:{height}`) em `BlockchainRepositoryImpl`.
- Serialização típica com `bincode` (opcional) e `serde_json` para configs.
- Batch writes (`WriteBatch`) garantem atomicidade para múltiplos registros.

```rust
pub fn get_keys_with_prefix(&self, prefix: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let mut keys = Vec::new();
  for item in self.db.iterator(IteratorMode::Start) {
    let (key, _) = item?;
    let key_str = String::from_utf8(key.to_vec())?;
    if key_str.starts_with(prefix) {
      keys.push(key_str);
    }
  }
  Ok(keys)
}
```

## 🔁 Fluxo gRPC ponta a ponta

1. **Protobuf** (`proto/blockchain.proto`): define mensagens e serviços.
2. **build.rs**: executa `tonic_build::compile`, gerando código client/server.
3. **Servidor** (`infrastructure/grpc/server.rs`): implementa trait gRPC gerado.
4. **Service Layer** (`application/services/blockchain_service.rs`): orquestra operações.
5. **Domain**: `Block`, `BlockchainGraph`, validações e regras de negócio.
6. **Persistence**: `BlockchainRepositoryImpl` salva e busca dados em RocksDB.

Fluxo `AddBlock` resumido:

```
client gRPC → AddBlockRequest → BlockchainServiceImpl::handle_add_block
→ carrega último bloco → cria novo Block → minera → salva em RocksDB → retorna AddBlockResponse
```

## 🛠️ Ferramentas Cargo, Build e Observabilidade

- `cargo build --release`: compila com otimizações; binário em `target/release`.
- `cargo fmt` e `cargo clippy -- -D warnings` mantêm estilo e padrão idiomático.
- `cargo watch -x run`: recompile automático em desenvolvimento (necessita `cargo-watch`).
- `tracing` + `RUST_LOG=debug` revelam detalhes de execução.
- Integrar `tracing-opentelemetry` + Jaeger/Tempo para observabilidade distribuída.
- Métricas futuras: adicione `metrics` crate ou `prometheus`.
- **Build troubleshooting**: consulte [BUILD_TROUBLESHOOTING.md](docs/BUILD_TROUBLESHOOTING.md) para resolver erros comuns de compilação.

## 🧪 Testes, Integração e Qualidade

- **Unitários**: dentro dos arquivos (`mod tests`), focados em regras pontuais.
- **Integração** (`tests/integration_tests.rs`): exercitam fluxo completo com Tokio.
- **Testes de persistência**: usam `tempfile::tempdir()` para criar DBs temporários.
- **Cobertura**: `cargo llvm-cov` (instalar `cargo-llvm-cov`) para relatório de cobertura.
- **Lint**: `cargo clippy` com opções `-W clippy::pedantic` para maior rigor.

```rust
#[tokio::test]
async fn test_block_creation_and_validation() {
  let mut graph = BlockchainGraph::new("test".into(), GraphType::Transaction, "Test graph".into(), 2);
  let block1 = Block::new(graph.get_latest_block().unwrap().hash.clone(), "First block".into(), "test".into(), 1, vec![]);
  graph.add_block(block1).unwrap();
  assert!(graph.is_valid());
}
```

## 🧪 Laboratórios Guiados

1. **Structs enriquecidas**: troque `String` por tipos mais específicos (ex.: `serde_json::Value`) em `Block::data` e ajuste serialização.
2. **Validação customizada**: implemente `ValidationStrategy` garantindo diferença mínima de timestamp entre blocos.
3. **Repository in-memory**: crie implementações de teste para `BlockchainRepository` usando `DashMap` e injete em `BlockchainServiceImpl`.
4. **Batch gRPC**: adicione `rpc AddBlocks(AddBlocksRequest)` no `.proto`, gere código e implemente fluxo completo.
5. **Observabilidade**: adicione `tracing::instrument` às funções críticas e exporte spans para visualizar em Jaeger.
6. **Proof of Work adaptativa**: implemente trait `ProofOfWork` com ajuste dinâmico de dificuldade baseado em tempo médio de mineração.
7. **Integração C#**: utilize `Smart-Contract/` para criar API REST que chama gRPC, validando interoperabilidade cross-language.
8. **Feature flags**: crie feature `in-memory-db` que troca RocksDB por `HashMap` para ambientes de teste.
9. **Benchmarks**: use `criterion` para comparar tempos de validação com 100, 1.000 e 10.000 blocos.
10. **Streaming**: experimente adicionar RPC streaming (`server streaming`) para enviar blocos continuamente ao cliente.

## 🧭 Diagnóstico e Debug

- Habilite logs detalhados: `RUST_LOG=blockchain_grpc=debug cargo run`.
- Use `cargo expand` para inspecionar macros (`cargo install cargo-expand`).
- Profiler: `perf` (Linux) ou `Instruments` (macOS) para analisar uso de CPU.
- Debug com `lldb`/`gdb`: `rust-gdb target/debug/blockchain-grpc`.
- Inspecione RocksDB com `rocksdb::Iterator` ou ferramentas CLI (`ldb` bundled com RocksDB).
- **Build errors**: consulte [BUILD_TROUBLESHOOTING.md](docs/BUILD_TROUBLESHOOTING.md) para soluções de problemas de compilação comuns.

## ✅ Checklist de Estudos

- [ ] Ler `src/domain/block.rs` e `graph.rs`, entendendo regras de validação.
- [ ] Acompanhar fluxo `AddBlock` ponta a ponta (gRPC → service → domain → persistence).
- [ ] Compilar o projeto após modificar o `.proto` e observar regeneração via `build.rs`.
- [ ] Rodar `cargo test` (unitários) e `cargo test --test integration_tests` (integração).
- [ ] Experimentar `RUST_LOG=debug` e mapear mensagens emitidas.
- [ ] Implementar pelo menos um laboratório guiado e documentar resultado.
- [ ] Gerar documentação (`cargo doc --open`) e navegar nos módulos.
- [ ] Criar binário customizado em `src/bin/` que consuma `BlockchainServiceImpl` diretamente.

## ❓ Perguntas Frequentes

- **Preciso entender lifetimes explícitos agora?** Não imediatamente; este projeto usa principalmente lifetimes implícitas. Ao editar APIs que retornam referências, revise o capítulo 10 do Rust Book.
- **Posso usar `Arc<Mutex<T>>` em vez de `RwLock`?** Sim, mas `RwLock` oferece ganho quando há muito mais leituras que escritas, como neste caso.
- **Como regenerar o código gRPC?** Basta rodar `cargo build` ou `cargo check`; `build.rs` cuida da compilação do protobuf automaticamente.
- **Onde ficam os dados físicos?** Por padrão em `./data/blockchain` (configurável via `config.json`).
- **Como criar um cliente Rust gRPC?** Ative `build_client(true)` (já habilitado) e use os tipos gerados em `infrastructure::grpc::blockchain::blockchain_service_client`.
- **Posso executar no Windows?** Sim; use PowerShell, configure `protoc` e compile normalmente (`cargo run`). Para scripts shell, utilize WSL2.

## 📗 Glossário de Rust

- `crate`: unidade de compilação; pode ser binário ou biblioteca.
- `module (mod)`: subdivisão de uma crate; controla visibilidade e organização.
- `trait`: contrato de comportamento que tipos podem implementar.
- `impl`: bloco de implementação de métodos ou traits para um tipo específico.
- `Arc`: contagem de referência atômica para compartilhar dados entre threads.
- `async/await`: sintaxe para código assíncrono cooperativo.
- `tonic`: framework gRPC idiomático em Rust.
- `serde`: biblioteca de serialização/desserialização.
- `bincode`: formato binário rápido e compacto.
- `RwLock`: lock de leitura/escrita permitindo concorrência otimista.

## 📚 Recursos Complementares

- Documentação oficial: [https://doc.rust-lang.org](https://doc.rust-lang.org)
- The Rust Book (PT-BR): [https://rust-br.github.io/rust-book-pt-br/](https://rust-br.github.io/rust-book-pt-br/)
- Rustlings (exercícios práticos): [https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
- Exercism Rust: [https://exercism.org/tracks/rust](https://exercism.org/tracks/rust)
- Guia Tokio: [https://tokio.rs/tokio/tutorial](https://tokio.rs/tokio/tutorial)
- gRPC com tonic: [https://github.com/hyperium/tonic](https://github.com/hyperium/tonic)
- RocksDB para Rust: [https://github.com/rust-rocksdb/rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb)
- Rust Async Book: [https://rust-lang.github.io/async-book/](https://rust-lang.github.io/async-book/)
- Tracing + Observabilidade: [https://docs.rs/tracing](https://docs.rs/tracing)

---