# Explicação do arquivo `src/main.rs`

Este documento explica, de forma detalhada, o que acontece durante a execução do binário principal do serviço (função `main`). O objetivo é deixar claro o fluxo de inicialização, os componentes envolvidos e o porquê de cada etapa.

## Visão geral

A `main` faz o boot do serviço gRPC de blockchain. Em linhas gerais, ela:

- Inicializa o sistema de logs/telemetria (tracing) com suporte a `RUST_LOG`.
- Carrega as configurações a partir de um arquivo JSON (`config.json`).
- Garante o diretório de dados, abre o RocksDB e instancia o repositório.
- Constrói o serviço de aplicação e executa sua rotina de inicialização.
- Sobe o servidor gRPC escutando no endereço configurado.

Tudo isso é assíncrono e roda sobre o runtime do Tokio.

## Código comentado (passo a passo)

Trechos de código referenciados abaixo correspondem ao conteúdo de `src/main.rs`:

### 1) Imports e alias

- `use std::sync::Arc;` — `Arc` é um ponteiro de contagem de referências thread-safe. Usamos para compartilhar instâncias (repositório/serviço) entre tasks assíncronas do servidor gRPC sem cópia.
- `use blockchain_grpc::{BlockchainServiceImpl, Settings};` — expõe tipos do crate (camadas de aplicação e config).
- `use blockchain_grpc::infrastructure::persistence::{BlockchainRepositoryImpl, RocksDbAdapter};` — componentes de infraestrutura para persistência em RocksDB.

### 2) Runtime assíncrono

- `#[tokio::main]` — macro que cria o runtime do Tokio e transforma `main` em uma função `async`. Todo o servidor gRPC e I/O assíncrono dependem deste runtime.

### 3) Inicialização de logs (tracing)

```
tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
    )
    .init();
```

- Configura o subscriber do `tracing` (logs estruturados).
- Se a variável de ambiente `RUST_LOG` estiver definida (por exemplo, `RUST_LOG=debug`), ela dita o nível de log. Caso contrário, usa o nível `info`.
- Exemplos de valores úteis: `RUST_LOG=debug`, `RUST_LOG=blockchain_grpc=trace,tokio=info`.

### 4) Mensagem de boot

```
tracing::info!("🔗 Starting Blockchain gRPC Service");
```

- Apenas sinaliza início do processo para facilitar troubleshooting.

### 5) Carregar configurações

```
let settings = Settings::load("config.json")?;
tracing::info!("⚙️  Configuration loaded");
```

- Lê o arquivo `config.json` na raiz do projeto (ou diretório de execução) e materializa a struct `Settings`.
- Entre outros campos, as configurações definem o diretório de dados e o endereço/porta do servidor.
- O operador `?` propaga erros (se o arquivo não existir ou estiver inválido, a execução falha com mensagem clara).

### 6) Preparar armazenamento (RocksDB)

```
std::fs::create_dir_all(&settings.storage.data_dir)?;
let db = Arc::new(RocksDbAdapter::new(&settings.storage.data_dir)?);
tracing::info!("💾 Storage initialized at {}", settings.storage.data_dir);
```

- Garante que o diretório de dados exista (`create_dir_all`).
- Abre/instancia o `RocksDbAdapter` apontando para o caminho configurado.
- Envolve o adaptador em `Arc` para compartilhamento seguro entre múltiplas tasks.

### 7) Repositório de blockchain

```
let repository = Arc::new(BlockchainRepositoryImpl::new(db));
```

- Cria o repositório que implementa as operações de leitura/gravação de blocos e grafos sobre o RocksDB.
- Também é compartilhado via `Arc`.

### 8) Serviço de aplicação

```
let service = Arc::new(BlockchainServiceImpl::new(repository));
service.initialize().await?;
tracing::info!("✅ Service initialized successfully");
```

- Constrói o serviço principal da aplicação (regras de negócio, orquestração de casos de uso).
- `initialize()` permite preparar estado inicial (ex.: criar grafos padrão, aquecer caches, migrar índices, etc.). É assíncrono e pode acessar o repositório.

### 9) Subir o servidor gRPC

```
let addr = settings.server_address();
blockchain_grpc::start_grpc_server(service, addr).await?;
```

- Obtém o endereço de escuta a partir das configurações (por exemplo, `127.0.0.1:50051`).
- Inicia o servidor gRPC, registrando as implementações dos métodos e começando a aceitar conexões.
- A chamada é assíncrona e normalmente só retorna se o servidor encerrar (erro ou shutdown).

### 10) Tratamento de erros

```
async fn main() -> Result<(), Box<dyn std::error::Error>> { ... }
```

- A assinatura retorna `Result`. Usando o operador `?` ao longo do fluxo, qualquer falha interrompe a execução e propaga uma mensagem de erro adequada.

## Por que usar `Arc`?

- O servidor gRPC (Tonic) atende múltiplas requisições em paralelo. Para compartilhar o mesmo serviço/repositório sem copiar estado, usamos `Arc`.
- `Arc` garante contagem de referências thread-safe; quando o último clone é descartado, o recurso é liberado.

## Variáveis de ambiente úteis

- `RUST_LOG`: controla o nível/filtragem de logs (ex.: `RUST_LOG=info` ou `RUST_LOG=blockchain_grpc=debug`).

## Execução (exemplo)

- Compilar:

```bash
cargo build --release
```

- Executar o servidor gRPC (assegure que `config.json` está presente):

```bash
cargo run --release
```

- Se quiser logs mais verbosos (Linux/WSL/powershell adaptam-se ao ambiente):

```bash
RUST_LOG=debug cargo run --release
```

No Windows PowerShell, você pode usar:

```powershell
$env:RUST_LOG = "debug"
cargo run --release
```

## Resumo do fluxo

1. Configura logs (com suporte a `RUST_LOG`).
2. Carrega `config.json` em `Settings`.
3. Garante o diretório de dados e abre o RocksDB.
4. Instancia o repositório e o serviço da aplicação.
5. Roda a inicialização do serviço (assíncrona).
6. Sobe o servidor gRPC no endereço configurado.

Esse pipeline segue princípios de Clean Architecture: a `main` apenas compõe e conecta camadas (config → infraestrutura → repositório → aplicação → interface gRPC), mantendo responsabilidades bem separadas e fáceis de testar.