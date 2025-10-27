# Análise da Estrutura de Dados: Block e Blockchain

Este documento detalha a estrutura de dados do `Block` e da `BlockchainGraph` (cadeia) neste projeto, com foco em facilitar o entendimento para estudos.

## Índice
- [Visão Geral](#visão-geral)
- [Estrutura do Block](#estrutura-do-block)
- [Estrutura da BlockchainGraph](#estrutura-da-blockchaingraph)
- [Fluxo de Dados](#fluxo-de-dados)
- [Invariantes e Validações](#invariantes-e-validações)
- [Exemplos Práticos](#exemplos-práticos)
- [Comparação com C/C#](#comparação-com-cc)

---

## Visão Geral

O projeto implementa uma blockchain **multi-grafo**, onde:
- Cada **grafo** é uma blockchain independente com propósito específico
- Cada **bloco** contém dados, referência ao bloco anterior e hash criptográfico
- Blocos podem **referenciar** blocos de outros grafos (validação cruzada)

### Hierarquia Conceitual

```
BlockchainGraph (Grafo/Cadeia)
    └── Vec<Block> (Sequência de blocos)
            ├── Block 0 (Genesis)
            ├── Block 1 (previous_hash → Block 0.hash)
            ├── Block 2 (previous_hash → Block 1.hash)
            └── ...
```

---

## Estrutura do Block

### Definição Rust

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub hash: String,                    // Hash do bloco (SHA-256)
    pub previous_hash: String,           // Hash do bloco anterior (linkagem)
    pub timestamp: i64,                  // Timestamp Unix (segundos desde epoch)
    pub data: String,                    // Dados arbitrários do bloco
    pub nonce: u64,                      // Nonce para Proof of Work (PoW)
    pub height: u64,                     // Altura/posição na cadeia (0 = genesis)
    pub graph_id: String,                // ID do grafo ao qual pertence
    pub cross_references: Vec<String>,   // Hashes de blocos em outros grafos
}
```

### Campos Detalhados

#### 1. `hash: String`
- **O que é:** Identificador único do bloco, calculado com SHA-256
- **Como é calculado:** 
  ```rust
  hash = SHA256(previous_hash + timestamp + data + nonce + height + graph_id + cross_refs)
  ```
- **Formato:** String hexadecimal de 64 caracteres (256 bits / 4 bits por caractere hex)
- **Exemplo:** `"00abc123...def"` (com zeros à esquerda se houver PoW)
- **Imutabilidade:** Se qualquer campo mudar, o hash muda completamente

#### 2. `previous_hash: String`
- **O que é:** Hash do bloco anterior na cadeia
- **Propósito:** Cria a **linkagem** entre blocos (daí o nome "blockchain")
- **Bloco Genesis:** Tem `previous_hash = "0"` (convenção)
- **Validação:** Ao adicionar bloco N, deve-se verificar que `block_N.previous_hash == block_N-1.hash`

#### 3. `timestamp: i64`
- **O que é:** Momento da criação do bloco (Unix timestamp em segundos)
- **Tipo:** `i64` → suporta datas até ~292 bilhões de anos no futuro
- **Geração:** `Utc::now().timestamp()` (chrono)
- **Propósito:** 
  - Ordenação temporal
  - Validações (ex.: rejeitar blocos com timestamps muito no futuro)
  - Auditoria

#### 4. `data: String`
- **O que é:** Payload do bloco — dados de aplicação
- **Formato:** String arbitrária (pode ser JSON, XML, binário codificado em base64, etc.)
- **Exemplos:**
  - Transações financeiras: `{"from": "Alice", "to": "Bob", "amount": 100}`
  - Logs de auditoria: `"User X accessed resource Y"`
  - Dados de identidade: `{"user_id": "123", "verified": true}`
- **Flexibilidade:** Cada grafo pode ter seu próprio formato/schema

#### 5. `nonce: u64`
- **O que é:** Número que varia para encontrar um hash válido no PoW
- **Proof of Work (PoW):** 
  - Objetivo: encontrar `nonce` tal que `hash` comece com N zeros
  - Algoritmo: incrementar `nonce` e recalcular hash até atingir dificuldade
- **Exemplo:** Se dificuldade = 2, o hash deve começar com "00"
- **Range:** 0 a 18.446.744.073.709.551.615 (2^64 - 1)

#### 6. `height: u64`
- **O que é:** Posição do bloco na cadeia (índice/altura)
- **Bloco Genesis:** `height = 0`
- **Próximo bloco:** `height = altura_anterior + 1`
- **Propósito:**
  - Navegação rápida
  - Indexação no RocksDB (`block:{graph}:{height}`)
  - Validação de sequência

#### 7. `graph_id: String`
- **O que é:** Identificador do grafo/blockchain ao qual o bloco pertence
- **Exemplos:** `"transactions"`, `"identity"`, `"audit_logs"`
- **Propósito:** Permite múltiplas blockchains independentes no mesmo sistema

#### 8. `cross_references: Vec<String>`
- **O que é:** Lista de hashes de blocos de **outros grafos**
- **Propósito:** Validação cruzada e integridade inter-grafos
- **Exemplo:**
  ```rust
  // Bloco no grafo "identity" referencia blocos em "transactions" e "audit"
  cross_references: vec![
      "abc123...def",  // hash de bloco em "transactions"
      "456789...xyz",  // hash de bloco em "audit"
  ]
  ```
- **Validação:** Ao verificar o grafo, checa-se se todos os cross-refs existem

---

## Estrutura da BlockchainGraph

### Definição Rust

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainGraph {
    pub id: String,                   // Identificador único do grafo
    pub graph_type: GraphType,        // Tipo/propósito do grafo
    pub description: String,          // Descrição legível
    pub created_at: i64,              // Timestamp de criação
    pub difficulty: usize,            // Dificuldade do PoW (nº de zeros)
    #[serde(skip)]
    pub chain: Vec<Block>,            // Cadeia de blocos (cache em memória)
}
```

### Campos Detalhados

#### 1. `id: String`
- **O que é:** Identificador único do grafo
- **Exemplos:** `"tx_main"`, `"identity_prod"`, `"audit_2024"`
- **Convenção:** Use nomes descritivos e sem espaços

#### 2. `graph_type: GraphType`
- **O que é:** Enum que categoriza o propósito do grafo
- **Valores:**
  ```rust
  pub enum GraphType {
      Transaction,  // Transações financeiras
      Identity,     // Dados de identidade
      Asset,        // Propriedade de ativos
      Audit,        // Logs de auditoria
      Custom,       // Customizado
  }
  ```
- **Serialização:** Convertido para `i32` no Protobuf (Transaction=0, Identity=1, etc.)

#### 3. `description: String`
- **O que é:** Descrição legível do propósito do grafo
- **Exemplo:** `"Blockchain de transações financeiras do sistema XYZ"`

#### 4. `created_at: i64`
- **O que é:** Timestamp Unix de quando o grafo foi criado
- **Geração:** `Utc::now().timestamp()`

#### 5. `difficulty: usize`
- **O que é:** Número de zeros que o hash deve ter no início (PoW)
- **Valores típicos:**
  - 0-1: Instantâneo (dev/testes)
  - 2-3: Rápido (~milissegundos)
  - 4-5: Moderado (~segundos)
  - 6+: Lento (minutos ou mais)
- **Cálculo:** Para dificuldade N, o hash deve satisfazer: `hash.starts_with("0".repeat(N))`

#### 6. `chain: Vec<Block>`
- **O que é:** Vetor ordenado de blocos (cache em memória)
- **Ordem:** `chain[0]` = genesis, `chain[n]` = bloco de altura n
- **Persistência:** `#[serde(skip)]` — não é serializado (blocos são salvos individualmente no RocksDB)
- **Carregamento:** Preenchido ao inicializar o serviço via `load_blocks()`

---

## Fluxo de Dados

### 1. Criação de um Novo Bloco

```rust
// Pseudocódigo do fluxo
let previous_hash = graph.get_latest_block().unwrap().hash.clone();
let height = graph.get_latest_block().unwrap().height + 1;

let mut block = Block::new(
    previous_hash,    // Hash do último bloco
    data,             // Dados de aplicação
    graph_id,         // ID do grafo
    height,           // Altura atual
    cross_refs,       // Referências cruzadas
);
// Neste ponto: hash calculado com nonce=0

// Mineração (PoW)
block.mine_block(difficulty);  // Incrementa nonce até hash válido

// Validação
assert!(block.is_valid());
assert!(block.has_valid_difficulty(difficulty));

// Adicionar ao grafo
graph.chain.push(block.clone());

// Persistir
repository.save_block(&graph_id, &block).await;
```

### 2. Cálculo do Hash

```rust
pub fn calculate_hash(&self) -> String {
    // Concatenar todos os campos relevantes
    let content = format!(
        "{}{}{}{}{}{}{}",
        self.previous_hash,
        self.timestamp,
        self.data,
        self.nonce,
        self.height,
        self.graph_id,
        self.cross_references.join(",")
    );

    // SHA-256
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    
    // Converter para hex
    hex::encode(result)
}
```

**Importante:** Qualquer mudança em **qualquer campo** resulta em hash completamente diferente (efeito avalanche do SHA-256).

### 3. Mineração (Proof of Work)

```rust
pub fn mine_block(&mut self, difficulty: usize) {
    let target = "0".repeat(difficulty);  // Ex: "00" para difficulty=2

    while !self.hash.starts_with(&target) {
        self.nonce += 1;                  // Incrementa nonce
        self.hash = self.calculate_hash(); // Recalcula hash
    }
    // Loop termina quando hash tem os zeros necessários
}
```

**Complexidade:** O(2^difficulty) — dobra a cada incremento de dificuldade.

### 4. Validação da Cadeia

```rust
pub fn is_valid(&self) -> bool {
    for i in 1..self.chain.len() {
        let current = &self.chain[i];
        let previous = &self.chain[i - 1];

        // 1. Validar hash do bloco atual
        if !current.is_valid() {
            return false;
        }

        // 2. Validar linkagem (previous_hash)
        if current.previous_hash != previous.hash {
            return false;
        }

        // 3. Validar dificuldade (PoW)
        if !current.has_valid_difficulty(self.difficulty) {
            return false;
        }

        // 4. Validar altura sequencial
        if current.height != previous.height + 1 {
            return false;
        }
    }
    true
}
```

---

## Invariantes e Validações

### Invariantes de um Block

1. **Hash Correto:** `block.hash == block.calculate_hash()`
2. **Dificuldade:** `block.hash.starts_with("0".repeat(difficulty))`
3. **Timestamp:** `block.timestamp > 0` (e idealmente `<= now + tolerance`)

### Invariantes de uma BlockchainGraph

1. **Genesis é o primeiro:** `chain[0].height == 0 && chain[0].previous_hash == "0"`
2. **Linkagem íntegra:** `∀i > 0: chain[i].previous_hash == chain[i-1].hash`
3. **Alturas sequenciais:** `∀i > 0: chain[i].height == chain[i-1].height + 1`
4. **Todos os blocos válidos:** `∀block ∈ chain: block.is_valid()`
5. **PoW em todos:** `∀block ∈ chain: block.has_valid_difficulty(difficulty)`

### Validações ao Adicionar Bloco

```rust
// graph.add_block() executa estas verificações:
if block.previous_hash != last_block.hash {
    return Err("Invalid previous hash");
}
if block.height != last_block.height + 1 {
    return Err("Invalid block height");
}
// Minera o bloco
block.mine_block(self.difficulty);

if !block.is_valid() {
    return Err("Invalid block hash");
}
if !block.has_valid_difficulty(self.difficulty) {
    return Err("Block does not meet difficulty");
}
```

---

## Exemplos Práticos

### Exemplo 1: Criando um Grafo e Adicionando Blocos

```rust
// 1. Criar grafo
let mut graph = BlockchainGraph::new(
    "transactions".to_string(),
    GraphType::Transaction,
    "Financial transactions".to_string(),
    2  // difficulty
);

// Grafo começa com bloco genesis
assert_eq!(graph.chain.len(), 1);
assert_eq!(graph.chain[0].height, 0);
assert_eq!(graph.chain[0].previous_hash, "0");

// 2. Adicionar primeiro bloco
let latest = graph.get_latest_block().unwrap();
let block1 = Block::new(
    latest.hash.clone(),                      // previous_hash do genesis
    r#"{"from":"Alice","to":"Bob","amt":50}"#.to_string(),
    "transactions".to_string(),
    1,                                         // height
    vec![],                                    // sem cross-refs
);

graph.add_block(block1).unwrap();

// 3. Verificar
assert_eq!(graph.chain.len(), 2);
assert!(graph.is_valid());
```

### Exemplo 2: Cross-References entre Grafos

```rust
// Grafo 1: Transações
let mut tx_graph = BlockchainGraph::new("tx".to_string(), GraphType::Transaction, "Tx".to_string(), 2);
let tx_block = Block::new(/* ... */);
let mined_tx = tx_graph.add_block(tx_block).unwrap();

// Grafo 2: Auditoria (referencia a transação)
let mut audit_graph = BlockchainGraph::new("audit".to_string(), GraphType::Audit, "Audit".to_string(), 2);
let audit_block = Block::new(
    /* ... */,
    vec![mined_tx.hash.clone()],  // Cross-reference!
);
audit_graph.add_block(audit_block).unwrap();

// Validação cruzada
let mut graphs = HashMap::new();
graphs.insert("tx".to_string(), tx_graph);
graphs.insert("audit".to_string(), audit_graph.clone());

// Verifica se os cross-refs existem
assert!(audit_graph.validate_cross_references(&graphs).is_ok());
```

### Exemplo 3: Detecção de Adulteração

```rust
let mut graph = BlockchainGraph::new(/* ... */);
// Adicionar blocos...

// ❌ Tentar adulterar um bloco no meio da cadeia
graph.chain[1].data = "DADOS FALSOS".to_string();

// A validação falha!
assert!(!graph.is_valid());  // Retorna false

// Por quê?
// - O hash do bloco[1] mudou (pois depende de `data`)
// - O bloco[2].previous_hash não bate mais com o novo hash do bloco[1]
// - A cadeia está quebrada!
```

---

## Comparação com C/C#

### Estrutura do Block

#### Em C (struct)

```c
typedef struct Block {
    char hash[65];              // SHA-256 hex + null terminator
    char previous_hash[65];
    int64_t timestamp;
    char* data;                 // heap-allocated string
    uint64_t nonce;
    uint64_t height;
    char graph_id[50];
    char** cross_references;    // array de strings
    size_t cross_ref_count;
} Block;
```

**Diferenças:**
- Rust: `String` gerenciado automaticamente (ownership)
- C: Ponteiros crus, malloc/free manual, risco de leaks

#### Em C# (class)

```csharp
public class Block
{
    public string Hash { get; set; }
    public string PreviousHash { get; set; }
    public long Timestamp { get; set; }
    public string Data { get; set; }
    public ulong Nonce { get; set; }
    public ulong Height { get; set; }
    public string GraphId { get; set; }
    public List<string> CrossReferences { get; set; }
}
```

**Diferenças:**
- Rust: `pub` fields → acesso direto (struct-like)
- C#: Properties com getters/setters, GC para memória
- Rust: `Vec<String>` → ownership; C#: `List<string>` → referência gerenciada

### Validação de Cadeia

#### Em Rust

```rust
pub fn is_valid(&self) -> bool {
    for i in 1..self.chain.len() {
        let current = &self.chain[i];      // empréstimo imutável
        let previous = &self.chain[i - 1];
        
        if current.previous_hash != previous.hash {
            return false;
        }
    }
    true
}
```

#### Em C

```c
bool is_valid(BlockchainGraph* graph) {
    for (size_t i = 1; i < graph->chain_len; i++) {
        Block* current = &graph->chain[i];   // ponteiro
        Block* previous = &graph->chain[i-1];
        
        if (strcmp(current->previous_hash, previous->hash) != 0) {
            return false;
        }
    }
    return true;
}
```

#### Em C#

```csharp
public bool IsValid()
{
    for (int i = 1; i < Chain.Count; i++)
    {
        var current = Chain[i];    // referência
        var previous = Chain[i - 1];
        
        if (current.PreviousHash != previous.Hash)
        {
            return false;
        }
    }
    return true;
}
```

**Comparação:**
- Rust: Borrow checker garante segurança em compile-time
- C: Ponteiros manuais, sem verificações automáticas
- C#: Referências gerenciadas, GC cuida da memória

---

## Representação Visual

### Block Individual

```
┌─────────────────────────────────────────────────────┐
│ Block #42                                           │
├─────────────────────────────────────────────────────┤
│ hash:          "00a3f8e9..."                        │
│ previous_hash: "00b7c2d1..."  ───┐                  │
│ timestamp:     1698345600         │ (linkagem)      │
│ data:          "Transaction X"    │                 │
│ nonce:         128374             │                 │
│ height:        42                 │                 │
│ graph_id:      "transactions"     │                 │
│ cross_refs:    ["abc...", "def..."]                 │
└─────────────────────────────────────────────────────┘
                                     │
                    ┌────────────────┘
                    ▼
          ┌──────────────────────┐
          │ Block #41            │
          │ hash: "00b7c2d1..."  │
          └──────────────────────┘
```

### BlockchainGraph (Cadeia Completa)

```
BlockchainGraph: "transactions" (difficulty=2)
───────────────────────────────────────────────────

Block 0 (Genesis)              Block 1                    Block 2
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│ height: 0        │───▶│ height: 1        │───▶│ height: 2        │
│ prev:   "0"      │    │ prev: "00abc..." │    │ prev: "00def..." │
│ hash: "00abc..." │    │ hash: "00def..." │    │ hash: "00ghi..." │
│ data: "Genesis"  │    │ data: "TX 1"     │    │ data: "TX 2"     │
│ nonce: 42        │    │ nonce: 1837      │    │ nonce: 9274      │
└──────────────────┘    └──────────────────┘    └──────────────────┘
```

### Multi-Grafo com Cross-References

```
Grafo: "transactions"                Grafo: "audit"
┌────────────────┐                  ┌────────────────┐
│ Block 0        │                  │ Block 0        │
│ hash: "00a..." │◀─────────────────│ cross_ref:     │
└────────────────┘                  │ ["00a..."]     │
        │                            └────────────────┘
        ▼                                    │
┌────────────────┐                          ▼
│ Block 1        │                  ┌────────────────┐
│ hash: "00b..." │◀─────────────────│ Block 1        │
└────────────────┘                  │ cross_ref:     │
                                     │ ["00b..."]     │
                                     └────────────────┘
```

---

## Glossário Técnico

- **Hash:** Saída de função criptográfica (SHA-256) que mapeia dados de tamanho arbitrário em 256 bits (64 caracteres hex)
- **Nonce:** "Number used once" — contador incrementado para variar o hash durante mineração
- **PoW (Proof of Work):** Algoritmo que exige trabalho computacional para criar um bloco válido
- **Genesis Block:** Primeiro bloco da cadeia (altura 0, previous_hash = "0")
- **Chain:** Sequência ordenada de blocos ligados por hashes
- **Height:** Posição/índice do bloco na cadeia (genesis = 0)
- **Cross-Reference:** Referência (hash) de um bloco em outro grafo
- **Difficulty:** Número de zeros requeridos no início do hash (aumenta custo computacional exponencialmente)

---

## Pontos de Atenção

1. **Imutabilidade:** Uma vez minerado e adicionado, um bloco **não deve** ser modificado
2. **Ordem importa:** A sequência dos blocos é crítica; trocar ordem quebra a cadeia
3. **Hash é barato de verificar, caro de encontrar:** SHA-256 é rápido, mas encontrar hash com N zeros é lento
4. **Persistência:** `chain` é cache; a fonte da verdade é o RocksDB
5. **Concorrência:** Múltiplas threads podem acessar grafos via `Arc<RwLock<HashMap>>`

---

## Recursos Adicionais

- **SHA-256:** https://en.wikipedia.org/wiki/SHA-2
- **Proof of Work:** https://en.bitcoin.it/wiki/Proof_of_work
- **Merkle Trees:** Estrutura complementar para validação eficiente (não implementada aqui, mas comum em blockchains)
- **Código-fonte:** 
  - `src/domain/block.rs` — Estrutura e métodos do Block
  - `src/domain/graph.rs` — Estrutura e métodos da BlockchainGraph
  - `tests/integration_tests.rs` — Testes que demonstram uso

---

**Data de Criação:** 26 de outubro de 2025  
**Versão do Projeto:** 0.1.0  
**Autor:** Documentação para estudos
