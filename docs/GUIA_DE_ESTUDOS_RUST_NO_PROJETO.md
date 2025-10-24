# Guia de Estudos: Rust aplicado ao projeto Blockchain-GRPC

Este guia foi feito para te levar à fluência neste projeto Rust com gRPC e RocksDB. Está organizado por tópicos, com referências de estudo, paralelos com C# e C, e trilha prática baseada no próprio código do repositório.


## Como usar este guia

- Para cada tópico: 1) leia os conceitos, 2) confira os arquivos apontados no projeto, 3) execute exercícios curtos, e 4) consulte as referências rápidas.
- Você já domina C# e C, então trago analogias diretas para acelerar seu entendimento.

Arquivos do projeto citados como referência:
- `src/main.rs` (fluxo de boot)
- `src/config/settings.rs` (config)
- `src/domain/*.rs` (entidades e regras)
- `src/application/services/*.rs` (serviços de aplicação/validação)
- `src/infrastructure/grpc/*.rs` e `proto/blockchain.proto` (gRPC)
- `src/infrastructure/persistence/*.rs` (RocksDB)
- `src/bin/http_proxy.rs` (proxy HTTP para Postman/curl)
- `tests/integration_tests.rs` e testes nos módulos


## 1) Propriedade, Empréstimo e Lifetimes (ownership/borrowing/lifetimes)

O que é e por que importa: Em Rust, não há GC (como no C#). Em vez disso, o compilador garante segurança de memória através de regras de propriedade. Isso elimina classes de bugs comuns em C (dangling pointers, double free) sem coletor de lixo.

- Propriedade: Cada valor tem um dono; quando o dono sai de escopo, o valor é dropado.
- Empréstimo: Você pode emprestar referências imutáveis (`&T`) ou mutáveis (`&mut T`) com regras estritas.
- Lifetimes: Anotações que garantem que referências nunca apontem para dados inválidos (normalmente inferidas pelo compilador).

Onde ver no projeto:
- `src/main.rs`: uso de `Arc<T>` para compartilhar instâncias entre tasks sem copiar dados.
- `src/infrastructure/persistence/repository.rs`: referências e retornos por valor/clone onde apropriado.
- `src/domain/*`: APIs que evitam cópias desnecessárias e usam referências de forma explícita.

Comparando com C# e C:
- C#: O GC cuida de liberar memória; em Rust, o dono libera ao sair do escopo (RAII).
- C: É como se o compilador “checasse” `free()`s automaticamente e proibisse usos após free.
- `Arc<T>` ≈ referência compartilhada com contagem atômica (similar a `std::shared_ptr` em C++); não confundir com GC.

Exercício:
- Abra `block.rs` e identifique onde dados são movidos vs. emprestados. Tente alterar uma função para aceitar `&str` ao invés de `String` e veja o impacto.

Referências:
- The Rust Book: Ownership, Borrowing, Lifetimes
  - https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
  - https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
- Rust by Example: Ownership
  - https://doc.rust-lang.org/rust-by-example/scope/ownership.html


## 2) Traits, Tipos e Polimorfismo

O que é: Traits definem comportamentos (como interfaces em C#), mas são resolvidos de forma estática (monomorfismo) ou dinâmica (`dyn Trait`) quando necessário.

- `trait` ≈ `interface` do C#, mas com poder extra (default methods, blanket impls).
- Genéricos com `impl<T: Trait>` são otimizados em compile-time (sem overhead de vtable). `Box<dyn Trait>` usa dispatch dinâmico similar à interface em C#.

Onde ver no projeto:
- `src/domain/traits.rs`: contratos de repositório e validação.
- `src/application/services/validation_service.rs`: estratégias de validação (Strategy Pattern) como traits concretas.

Comparando com C# e C:
- C#: `interface IFoo { ... }` ≈ `trait Foo { ... }`. `Trait + Generics` ≈ interfaces genéricas com JIT; em Rust é AOT e monomorfizado.
- C: Não há interfaces; você usaria ponteiros de função/structs — traits são uma forma segura e expressiva dessa ideia.

Exercício:
- Adicione uma nova `ValidationStrategy` (ex.: `TimestampMonotonicValidator`) e plugue-a no `ValidationService`.

Referências:
- The Rust Book: Traits & Generics
  - https://doc.rust-lang.org/book/ch10-00-generics.html
  - https://doc.rust-lang.org/book/ch10-02-traits.html
- Rust by Example: Trait
  - https://doc.rust-lang.org/rust-by-example/trait.html


## 3) Erros: Result, thiserror e anyhow

O que é: Em vez de exceptions, Rust usa `Result<T, E>`. Você lida com erros explicitamente, ganhando previsibilidade e performance.

- `?` propaga o erro. `thiserror` facilita criar tipos de erro; `anyhow` é ótimo para bordas e binários (erros dinâmicos com contextos).

Onde ver no projeto:
- `src/main.rs` retorna `Result<(), Box<dyn Error>>` e usa `?` em todo o fluxo.
- Repositório e serviços retornam `Result` para operações I/O e de domínio.

Comparando com C# e C:
- C#: try/catch em tempo de execução; Rust levanta erros no tipo — o compilador força tratamento.
- C: Códigos de erro + `errno` — Rust padroniza isso com tipos fortes e ergonomia (`?`).

Exercício:
- Envolva uma chamada do repositório com contexto usando `anyhow::Context` e logue o erro com `tracing`.

Referências:
- The Rust Book: Error Handling
  - https://doc.rust-lang.org/book/ch09-00-error-handling.html
- thiserror
  - https://docs.rs/thiserror
- anyhow
  - https://docs.rs/anyhow


## 4) Módulos, Visibilidade e Organização

O que é: O sistema de módulos controla namespaces e visibilidade (`pub`).

- Re-export com `pub use` simplifica caminhos. Integração gRPC usa `tonic::include_proto!` re-exportado.

Onde ver no projeto:
- `src/infrastructure/grpc/mod.rs`: reexporta `pub mod blockchain { tonic::include_proto!("blockchain") }`.
- `src/lib.rs`: reúne módulos públicos consumidos por `main`.

Comparando com C# e C:
- C#: namespaces; Rust é mais explícito com arquivos e pastas mapeando módulos.
- C: múltiplos arquivos `.c`/`.h` — módulos são equivalentes com visibilidade controlada.

Exercício:
- Reexporte um tipo de domínio em `src/lib.rs` para encurtar imports na `main`.

Referências:
- The Rust Book: Modules
  - https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html


## 5) Assíncrono: Tokio, async/await, async_trait

O que é: O runtime do Tokio executa tasks assíncronas; `async fn` define corrotinas; `await` suspende.

Onde ver no projeto:
- `#[tokio::main]` em `src/main.rs`.
- Servidor gRPC: handlers `async` nos serviços.
- `async_trait` quando precisamos de `async` em traits.

Comparando com C# e C:
- C#: muito similar a `async/await`. Diferenças: Rust não tem GC; o borrow checker atua também no async. Evite capturar referências de vida curta em futures.
- C: seria equivalente a usar state machines + callbacks manualmente, mas aqui o compilador gera a máquina de estados.

Exercício:
- Transforme um método síncrono do repositório em `async` e propague `await` (em ambiente controlado).

Referências:
- Tokio
  - https://tokio.rs/  |  https://docs.rs/tokio
- async-trait
  - https://docs.rs/async-trait


## 6) gRPC com Tonic/Prost

O que é: `prost` gera tipos a partir de `.proto`; `tonic` cria o servidor/cliente gRPC.

Onde ver no projeto:
- `proto/blockchain.proto`, `build.rs` (gera código gRPC).
- `src/infrastructure/grpc/server.rs` (servidor), `examples/client_example.rs` (cliente).
- Conversão domínio ↔ protobuf em `src/application/services/blockchain_service.rs`.

Comparando com C# e C:
- C#: `Grpc.AspNetCore` e `protobuf-net.Grpc` têm o mesmo papel. As mensagens geradas são parecidas; a diferença é a integração ao ecossistema Rust.
- C: gRPC C-core + wrappers manuais; em Rust o `tonic` encapsula de forma ergonômica e segura.

Exercício:
- Adicione um novo RPC simples (ex.: `GetGraphCount`) ao `.proto` e implemente no servidor e serviço.

Referências:
- Tonic
  - https://github.com/hyperium/tonic  |  https://docs.rs/tonic
- Prost
  - https://github.com/tokio-rs/prost  |  https://docs.rs/prost
- Protocol Buffers
  - https://developers.google.com/protocol-buffers


## 7) Persistência com RocksDB, bincode e índices

O que é: RocksDB é um KV-store embutido. Este projeto usa um esquema de chaves (`prefixos`) e serialização binária eficiente (`bincode`).

Onde ver no projeto:
- `src/infrastructure/persistence/rocksdb_adapter.rs` (baixo nível, prefix scan, batch).
- `src/infrastructure/persistence/repository.rs` (mapeamento de domínios → chaves e índices: `block:{graph}:{height}`, `block_hash:{graph}:{hash}`, `latest:{graph}`, `graph:{graph_id}`).

Comparando com C# e C:
- C#: Não é EF Core; pense em usar `Dictionary` persistente em disco com ordenação por chave e iteradores/seek.
- C: Similar a usar LevelDB/RocksDB direto via C API, porém com segurança de tipos e ergonomia do Rust.

Exercício:
- Adicione um índice auxiliar (ex.: por `timestamp`) e crie um método para consulta por intervalo de tempo.

Referências:
- rocksdb crate
  - https://docs.rs/rocksdb
- bincode
  - https://docs.rs/bincode


## 8) Mecânicas de Blockchain: Hash, PoW e Grafos cruzados

O que é: Cada bloco contém `previous_hash`, `nonce`, `height` e `graph_id`. PoW força custo computacional ao minerar (hash com prefixo de zeros).

Onde ver no projeto:
- `src/domain/block.rs`: `calculate_hash`, `mine_block`, validações.
- `src/domain/graph.rs`: validação do chain e referências cruzadas entre grafos.

Comparando com C# e C:
- C#: Sem diferença conceitual; a implementação em Rust enfatiza imutabilidade e zero-cost abstractions.
- C: Implementação semelhante em termos de hashing e loops; Rust adiciona segurança de memória e tipos fortes.

Exercício:
- Altere a dificuldade padrão de um grafo e avalie o tempo de mineração e logs.

Referências:
- sha2
  - https://docs.rs/sha2


## 9) Arquitetura (Clean) e Padrões

O que é: Separação clara dos layers:
- Domain (entidades/regra pura)
- Application (casos de uso/serviços/estratégias)
- Infrastructure (adapters gRPC, DB, config)

Onde ver no projeto:
- Pastas `src/domain`, `src/application`, `src/infrastructure`, `src/config`.
- Padrões: Repository (persistence), Strategy (validação), Adapter (RocksDB/gRPC), Mapper (domínio↔protobuf).

Comparando com C# e C:
- C#: Se parece com uma solução em `.NET` com camadas `Domain`, `Application`, `Infrastructure` e `Web/Grpc`.
- C: Você teria de impor disciplina de pastas e headers; em Rust, módulos reforçam a estrutura.

Exercício:
- Escreva um diagrama simples (ASCII ou PlantUML) da interação gRPC → Service → Repository → RocksDB e cole em `docs/`.

Referências:
- Clean Architecture (Martin)
  - https://8thlight.com/blog/uncle-bob/2012/08/13/the-clean-architecture.html


## 10) Observabilidade: tracing

O que é: Logs estruturados com filtros por nível e por módulo via `EnvFilter`.

Onde ver no projeto:
- `src/main.rs`: configuração do subscriber.
- Pontos de log em serviços e repositório.

Comparando com C# e C:
- C#: `ILogger` com filtros; aqui usamos `tracing` + `tracing-subscriber`.
- C: `printf`/`syslog`; `tracing` oferece spans e campos estruturados.

Exercício:
- Adicione `tracing::instrument` em um método crítico e observe os campos no log.

Referências:
- tracing
  - https://docs.rs/tracing
- tracing-subscriber
  - https://docs.rs/tracing-subscriber


## 11) Testes: unitários e integração

O que é: Testes por módulo (`#[cfg(test)]`) e pasta `tests/` para integração. Úteis para provar as invariantes do blockchain.

Onde ver no projeto:
- Testes em `src/domain/*` e `tests/integration_tests.rs`.

Comparando com C# e C:
- C#: xUnit/NUnit; o runner é o próprio `cargo test`.
- C: frameworks de teste manuais; Rust integra ça no cargo com isolamento.

Exercício:
- Crie um teste que falha quando a dificuldade é violada.

Referências:
- The Rust Book: Testing
  - https://doc.rust-lang.org/book/ch11-00-testing.html


## 12) Build, Cargo, build.rs e perfis

O que é: `Cargo.toml` define dependências e perfis; `build.rs` roda antes da compilação para gerar código (gRPC).

Onde ver no projeto:
- `Cargo.toml` (tonic/prost/tokio/rocksdb/etc.).
- `build.rs` (tonic-build para `proto/`).

Comparando com C# e C:
- C#: `csproj` + MSBuild Tasks; `build.rs` lembra um Target custom.
- C: `Makefile` + scripts pré-compilação.

Exercício:
- Ajuste o perfil `release` (ex.: `lto = true`, `codegen-units = 1`) e avalie o tempo/tamanho.

Referências:
- Cargo Book
  - https://doc.rust-lang.org/cargo/
- build scripts
  - https://doc.rust-lang.org/cargo/reference/build-scripts.html


## 13) Proxy HTTP (Axum) para Postman/curl

O que é: Um binário opcional que converte HTTP/JSON em gRPC, ideal para testar via Postman/curl.

Onde ver no projeto:
- `src/bin/http_proxy.rs` (rotas HTTP chamam cliente gRPC).
- `docs/USAGE.md` (exemplos curl/grpcurl).

Comparando com C# e C:
- C#: `Minimal APIs` no ASP.NET chamando um client gRPC.
- C: você faria um microservidor HTTP e chamaria gRPC C-core — bem mais verboso.

Exercício:
- Adicione uma rota `/health` que consulta o serviço gRPC e retorna status JSON.

Referências:
- Axum
  - https://docs.rs/axum


## 14) Windows/WSL, toolchain nativo e build estável

Pontos práticos:
- Para compilar RocksDB (via `rocksdb` crate), são necessários headers/libraries nativos (clang/zlib). No Windows puro, use `MSVC` toolchain e instale o que for pedido; no WSL, instale `build-essential`, `clang`, `libclang-dev`, `pkg-config`, `zlib1g-dev`, `libssl-dev`.
- Protoc (`protoc`) precisa estar instalado ou será provido por dependências do tonic-build (dependendo da config).

Referências:
- Instalação Rust (Windows/MSVC)
  - https://rustup.rs/
- WSL Build deps
  - Documentação em `docs/QUICKSTART.md` (do projeto) e guias do Ubuntu/Debian.


---

## Roadmap prático (7 dias, 45–90 min/dia)

Dia 1 — Fundamentos no código
- Ler `src/main.rs` + `docs/EXPLICACAO_MAIN.md`.
- Rodar o servidor gRPC e chamar 2 RPCs com `grpcurl`.

Dia 2 — Domínio e PoW
- Ler `src/domain/block.rs` e `src/domain/graph.rs`.
- Ajustar dificuldade e observar logs/tempo.

Dia 3 — gRPC ponta a ponta
- Ler `proto/blockchain.proto` → `server.rs` → `blockchain_service.rs`.
- Adicionar um RPC simples de leitura e testar.

Dia 4 — Persistência
- Estudar `rocksdb_adapter.rs` e `repository.rs`.
- Adicionar um novo índice ou consulta por faixa de timestamp.

Dia 5 — Assíncrono e validação
- Rever `async_trait` e validações em `validation_service.rs`.
- Criar uma nova `ValidationStrategy` e testá-la.

Dia 6 — Observabilidade e testes
- Instrumentar 1–2 métodos com `tracing::instrument`.
- Criar um teste de integração cobrindo um fluxo completo (criar grafo → minerar bloco → verificar grafo).

Dia 7 — Proxy HTTP e ergonomia
- Explorar `src/bin/http_proxy.rs`, adicionar `/health`.
- Documentar o fluxo com um diagrama em `docs/`.


## Checklist de fluência (autoavaliação)

- [ ] Consigo explicar ownership/borrowing e quando usar `Arc`, `Clone`, `&T` e `&mut T`.
- [ ] Implemento e uso `traits` e entendo a diferença entre `impl Trait` e `dyn Trait`.
- [ ] Trato erros com `Result` e `?`, e sei quando usar `thiserror`/`anyhow`.
- [ ] Navego e organizo módulos, entendo `pub` e reexports.
- [ ] Escrevo/entendo código `async` com Tokio e `async_trait`.
- [ ] Consigo criar/alterar RPCs com Tonic/Prost.
- [ ] Entendo o esquema de chaves no RocksDB e consigo criar/consultar índices.
- [ ] Compreendo PoW, hashing e validações cruzadas entre grafos.
- [ ] Instrumento logs com `tracing` e escrevo testes unitários/integrados.
- [ ] Ajusto `Cargo.toml`/`build.rs` e consigo otimizar o build.


## Apêndice: Mapa mental rápido Rust → C# / C

- Memória
  - Rust: Ownership/borrowing, RAII, sem GC → mais próximo do C/C++ com verificações em compile-time.
  - C#: GC; finalize/IDisposable para recursos.
  - C: `malloc/free`, erros comuns evitados por Rust.
- Polimorfismo
  - Rust: Traits (estático por default) e `dyn Trait` (dinâmico quando necessário).
  - C#: Interfaces com dispatch dinâmico; genéricos com JIT.
  - C: Funções e structs com ponteiros de função.
- Assíncrono
  - Rust: `async/await` gera state machines sem alocação extra se bem escrito.
  - C#: `async/await` com GC; similar em semântica de alto nível.
  - C: Callbacks e threads manuais.
- Erros
  - Rust: `Result`/`Option` + pattern matching.
  - C#: Exceptions, `TryParse`/`Nullable`.
  - C: códigos de erro, `errno`.

Se quiser, posso transformar este guia em um curso rápido com exercícios verificáveis (testes que você roda com `cargo test`) para cada dia do roadmap.


---

## Pitfalls comuns neste repositório (e como evitar)

- Capturar referências de vida curta em `async`/futures
  - Sintoma: erros de lifetime ao `await` ou mover closures.
  - Dica: Prefira mover valores de curta duração para dentro da future (use `clone()` quando for leve). Evite manter `&T` além do escopo.
  - Onde ver: Handlers gRPC e serviços que fazem I/O.

- Uso excessivo de `clone()` em estruturas grandes
  - Sintoma: alocação desnecessária, queda de performance.
  - Dica: Empreste (`&T`) quando possível; use `Arc<T>` para compartilhar entre tasks; serialize/deserializar só quando necessário.

- Conversão Domínio ↔ Protobuf
  - Sintoma: divergência entre tipos (ex.: `u64` ↔ `i64`, `String` ↔ `bytes`).
  - Dica: Centralize mapeamentos em funções/helpers no serviço de aplicação; documente convenções (ex.: timestamps em `i64` UNIX epoch).

- Iterações por prefixo no RocksDB
  - Sintoma: esquecer de delimitar corretamente a faixa; retorno de registros de outros grafos.
  - Dica: Siga um padrão rígido de chaves (`block:{graph}:{height}`), e valide prefixos antes de iterar.

- Dificuldade de PoW muito alta na dev
  - Sintoma: mineração lenta, testes demorados.
  - Dica: Reduza dificuldade nos ambientes de desenvolvimento.

- Logs verbosos sem filtro
  - Sintoma: Ruído em produção/dev.
  - Dica: Use `RUST_LOG` para filtrar módulos, ex.: `RUST_LOG=info,blockchain_grpc=debug`.


## Cheat sheet de comandos (PowerShell no Windows)

```powershell
# Build (debug) e rodar
cargo build
cargo run

# Build otimizado
cargo build --release

# Testes
cargo test

# Lint e formatação
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -D warnings

# Definir log em PowerShell
$env:RUST_LOG = "blockchain_grpc=debug,tokio=info"; cargo run

# Rodar binário do proxy HTTP
cargo run --bin http_proxy
```

Observação: Caso use WSL/Ubuntu, os comandos equivalentes funcionam em Bash (variáveis: `RUST_LOG=... cargo run`).


## Mapa Rust ↔ C# ↔ C com exemplos curtos

- Propriedade e escopo
  - C#: `using var stream = ...;` — gerencia recursos; GC lida com memória.
  - C: `malloc/free` manual.
  - Rust:
    ```rust
    {
        let s = String::from("data"); // s é dono
        // uso de s
    } // drop automático de s aqui (RAII)
    ```

- Empréstimo imutável/mutável
  - C#: `ref`/`in`/`out` têm semânticas diferentes; imutabilidade não é padrão.
  - C: ponteiros const vs não-const.
  - Rust:
    ```rust
    fn len_ref(s: &String) -> usize { s.len() }
    fn push_ref(s: &mut String, c: char) { s.push(c) }
    ```

- Traits vs Interfaces
  - C#: `interface IFoo { void Do(); }`
  - C: função via ponteiro dentro de struct.
  - Rust:
    ```rust
    trait Doer { fn do_it(&self); }
    struct A;
    impl Doer for A { fn do_it(&self) { /* ... */ } }
    fn run<T: Doer>(x: T) { x.do_it(); }
    ```

- Erros (Result) vs Exceptions
  - C#: `try { ... } catch(Exception e) { ... }`
  - C: retorna `int`/`errno`.
  - Rust:
    ```rust
    fn may_fail() -> Result<u32, std::io::Error> { Ok(42) }
    fn caller() -> Result<(), std::io::Error> { let _ = may_fail()?; Ok(0.into()) }
    ```

- Pattern Matching
  - C#: `switch` moderno com patterns.
  - C: `switch` de inteiros.
  - Rust:
    ```rust
    enum GraphType { A, B(u32) }
    fn f(g: GraphType) {
        match g {
            GraphType::A => {}
            GraphType::B(v) if v > 10 => {}
            GraphType::B(_) => {}
        }
    }
    ```


## Laboratórios guiados (hands-on)

1) Novo RPC de leitura
- Objetivo: adicionar `GetGraphCount` ao `.proto` e implementar ponta a ponta.
  - Passos:
    1. Editar `proto/blockchain.proto` e adicionar a mensagem/resposta e o RPC.
    2. Rodar `cargo build` para gerar o código com `tonic-build`.
    3. Implementar no `server.rs` delegando ao `BlockchainServiceImpl`.
    4. Implementar no `blockchain_service.rs` a contagem consultando o repositório.
    5. Testar via `grpcurl` (ou via proxy HTTP criando rota temporária).
  - Critérios de aceitação: Retornar o número correto de grafos criados.

2) Índice por timestamp no RocksDB
- Objetivo: listar blocos por faixa de tempo.
  - Passos:
    1. Definir chave `block_ts:{graph}:{timestamp}:{height}`.
    2. Gravar o índice ao persistir um bloco.
    3. Implementar uma consulta que faz `prefix_scan` em `block_ts:{graph}:{start_ts}` até `end_ts`.
    4. Expor um RPC `GetBlocksByTimeRange`.
  - Critérios: retornar blocos ordenados por tempo e dentro do intervalo.

3) Nova estratégia de validação
- Objetivo: rejeitar blocos com `timestamp` muito no futuro.
  - Passos: criar `FutureTimestampValidator` em `validation_service.rs`, configurar no serviço.
  - Critérios: testes cobrindo aceitação/rejeição.

4) Observabilidade
- Objetivo: instrumentar mineração de bloco.
  - Passos: adicionar `#[tracing::instrument]` em `mine_block` e logs com campos (`height`, `nonce`).
  - Critérios: logs mostram spans e campos ao minerar.


## Debugging e ferramentas

- rust-analyzer (VS Code)
  - Navegação de símbolos, inferência de tipos, “go to definition”.
- clippy
  - Detecta code smells e más práticas: `cargo clippy -- -D warnings`.
- rustfmt
  - Formatação consistente: `cargo fmt --all`.
- Logging estruturado
  - Use `tracing::instrument` e campos (`info!(height = ..., "msg")`).
- Perf/Profiling (avançado)
  - `cargo flamegraph` (Linux) ou `pprof-rs`/`dhat-rs`.


## Qualidade de código: checks rápidos

- Build: `cargo build` (PASS/FAIL)
- Lint: `cargo clippy --all-targets --all-features -D warnings` (PASS/FAIL)
- Formatação: `cargo fmt --all -- --check` (PASS/FAIL)
- Testes: `cargo test` (PASS/FAIL)

Integre esses quatro como “gates” antes de comitar mudanças públicas.


## Desempenho e alocação (notas práticas)

- Evite `clone()` em hot paths; prefira `&[u8]`, `&str` quando possível.
- Use `Vec::with_capacity` quando souber o tamanho aproximado.
- Serialize com `bincode` para tráfego/armazenamento binário (rápido); use `serde_json` apenas em fronteiras HTTP para legibilidade.
- Logue menos no caminho crítico (use `debug`/`trace` somente quando necessário).
- Em PoW, prefira prealocar buffers de hash e reaproveitar estruturas locais.


## Glossário mínimo

- Ownership: modelo de propriedade que define quem libera um recurso.
- Borrow: empréstimo temporário (`&T` / `&mut T`) sem transferir propriedade.
- Trait: contrato de comportamento implementado por tipos.
- Future: computação assíncrona que pode ser `await`ada.
- Arc: ponteiro com contagem de referência atômica para compartilhar entre threads.
- Result: enum de sucesso (`Ok`) ou erro (`Err`).
- Prost/Tonic: geração e runtime gRPC em Rust.
- RocksDB: KV-store embutido, otimizado para SSD.


## Leitura e recursos adicionais

- The Rust Book (oficial)
  - https://doc.rust-lang.org/book/
- Rust by Example
  - https://doc.rust-lang.org/rust-by-example/
- Rustlings (exercícios práticos)
  - https://github.com/rust-lang/rustlings
- Crates usados
  - tokio: https://docs.rs/tokio
  - tonic: https://docs.rs/tonic
  - prost: https://docs.rs/prost
  - rocksdb: https://docs.rs/rocksdb
  - serde/bincode: https://docs.rs/serde | https://docs.rs/bincode
  - tracing: https://docs.rs/tracing | tracing-subscriber: https://docs.rs/tracing-subscriber


## Próximos passos (avançado)

- Benchmark de mineração com `criterion`
  - Medir impacto de diferentes dificuldades e tamanhos de bloco.
- Pool de threads dedicado para PoW
  - Offload de mineração para evitar bloquear handlers gRPC.
- Métricas (Prometheus)
  - Expor contadores/tempos de mineração, latência de RPC, I/O do RocksDB.
- Autenticação/Autorização futura
  - Introduzir interceptors gRPC e JWT nas chamadas.