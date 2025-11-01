# 🚀 Guia de Otimização de Build

## O que foi configurado?

### 1. **Cache Incremental** ✅
- Compilação incremental ativada por padrão
- Reutiliza código já compilado entre builds

### 2. **Otimização de Dependências** ✅
- Dependências compiladas em modo otimizado (`opt-level = 3`)
- Seu código compilado em modo debug rápido
- **Resultado:** Dependências são compiladas UMA VEZ e cacheadas

### 3. **Paralelização** ✅
- Build usa 8 jobs paralelos (ajuste em `.cargo/config.toml` se necessário)

## Como usar?

### Build Normal (mais rápido agora!)
```bash
cargo build
```

### Build com Script Otimizado
```bash
.\scripts\fast-build.ps1
```

## Instalações Opcionais (Aceleram MUITO!)

### 1. sccache (Recomendado!)
Cache compartilhado entre projetos:
```bash
cargo install sccache
```

Depois descomente no `.cargo/config.toml`:
```toml
rustc-wrapper = "sccache"
```

### 2. LLD Linker (Mais rápido no Windows)
Instale o LLVM: https://releases.llvm.org/

Depois descomente no `.cargo/config.toml`:
```toml
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### 3. cargo-watch (Desenvolvimento)
Recompila automaticamente ao salvar:
```bash
cargo install cargo-watch
cargo watch -x build
```

## Quanto mais rápido fica?

### Antes:
- **Build completo:** ~5-10 minutos
- **Build incremental:** ~2-5 minutos

### Depois (com todas otimizações):
- **Build completo (primeira vez):** ~5-10 minutos (igual)
- **Build incremental:** ~10-30 segundos! 🚀
- **Com sccache entre projetos:** Quase instantâneo!

## Dicas Extras

### 1. Usar `cargo check` em vez de `cargo build`
Para apenas verificar erros (muito mais rápido):
```bash
cargo check
```

### 2. Build apenas um binário
```bash
cargo build --bin blockchain-grpc
# ou
cargo build --bin http_proxy
```

### 3. Usar `cargo-nextest` para testes
Testes paralelos e mais rápidos:
```bash
cargo install cargo-nextest
cargo nextest run
```

### 4. Limpar cache antigo (quando necessário)
```bash
cargo clean
```

## Ver estatísticas do sccache
```bash
sccache --show-stats
```

## Resetar cache do sccache
```bash
sccache --zero-stats
```

---

## 🐳 Build Docker Otimizado (cargo-chef)

### Como funciona?
- Build multi-stage usa `cargo-chef` para cachear dependências Rust agressivamente.
- Binário final é gerado em modo release e "stripado" para reduzir tamanho.
- `grpc_health_probe` é baixado em estágio separado e incluído na imagem final.
- EntryPoint executa bootstrap (cria config, garante permissões) e sobe o serviço como usuário não-root via `gosu`.

### Comandos principais
```bash
# Build da imagem otimizada
docker build -t blockchain-grpc:optimized .

# Subir com docker compose (reaproveita cache da imagem)
docker compose up --build
```

### Variáveis importantes
- `CONFIG_PATH` (default `/app/config.json`): caminho do arquivo de configuração no container.
- `DATA_DIR` (default `/app/data/blockchain`): diretório persistente (montado como volume).
- `RUST_LOG` (default `info`): nível de log do serviço.

### Estágios do `Dockerfile`
1. **chef**: instala toolchain, `cargo-chef` e dependências do protobuf.
2. **planner**: gera `recipe.json` com grafo de dependências.
3. **builder**: compila dependências e binário em release, depois aplica `strip`.
4. **healthcheck**: baixa `grpc_health_probe` estático.
5. **runtime**: imagem final enxuta com entrypoint que garante configuração e permissões.

### Dicas
- Alterações apenas em código (sem mexer em `Cargo.toml`) reaproveitam build de dependências.
- Se não montar um `config.json`, o entrypoint gera um baseado no template `config.example.json`.
- Ajuste o ARG `GRPC_HEALTH_PROBE_VERSION` no `Dockerfile` para controlar a versão do probe.
