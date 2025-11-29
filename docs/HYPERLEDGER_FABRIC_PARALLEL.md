# Blockchain-GRPC × Hyperledger Fabric

> Documento estratégico comparando o projeto **Blockchain-GRPC** com a plataforma **Hyperledger Fabric**, destacando semelhanças conceituais, diferenças arquiteturais e oportunidades de convergência.

## 1. Visão Geral Rápida

| Dimensão | Blockchain-GRPC | Hyperledger Fabric |
| --- | --- | --- |
| **Modelo de Rede** | Node único (por enquanto), focado em laboratório e pilotos. | Rede permissionada distribuída com múltiplas organizações, peers e orderers. |
| **Linguagem/Lógica** | Núcleo em Rust + serviços gRPC + proxy HTTP + cliente C#. | Chaincode (Go/Node/Java) executando em peers; SDKs em várias linguagens. |
| **Dados/Ledger** | Multi-grafos independentes persistidos em RocksDB (key-value). | Ledger por canal com blockchain + world state (LevelDB/CouchDB). |
| **Consenso** | Prova de Trabalho local por grafo, validando blocos sequenciais. | Pipeline Endorsement → Ordering (Raft/Kafka/BFT) → Commit. |
| **Identidade** | Configuração local; autenticação ainda não implementada. | MSP, CAs, certificados X.509 e políticas por organização. |
| **Integração** | gRPC público + bridge HTTP + cliente .NET. | SDKs oficiais, eventos, REST gateways de terceiros. |
| **Observabilidade/Deploy** | Docker multi-stage (cargo-chef), health probe gRPC, tracing opcional. | Ferramentas Fabric CA, peer/orderer CLIs, operadores Kubernetes, Prometheus/Grafana. |

## 2. Arquitetura e Camadas

- **Blockchain-GRPC** segue Clean Architecture: domínio puro (blocks, graphs, traits), camada de aplicação (serviços assíncronos com `tokio`/`tonic`) e infraestrutura (gRPC server, RocksDB, scripts). A separação permite portar o domínio para outros front-ends (CLI, REST, Chaincode) mantendo regras de negócio centralizadas.
- **Hyperledger Fabric** adota arquitetura modular composta por peers, orderers, CAs e chaincode containers. A lógica de negócio (chaincode) roda dentro dos peers, enquanto a ordenação de blocos é responsabilidade separada do ordering service.

**Paralelo:** Em Fabric, o par peer+chaincode ≈ (domínio + aplicação) do Blockchain-GRPC, enquanto o ordering service ≈ papel de PoW/local scheduler atual. Entretanto, Fabric separa fisicamente os papéis, fornecendo tolerância a falhas e governança multi-organização.

## 3. Identidade, Governança e Permissão

- **Blockchain-GRPC** ainda opera com confiança implícita (sem PKI). A configuração (`config.json`) define host, porta e diretórios; futuras camadas de autenticação foram planejadas no README.
- **Fabric** exige identidades X.509 emitidas por CAs, agrupadas em MSPs. Toda transação carrega assinaturas e políticas de endorsement determinam quem precisa aprovar.

**Oportunidade:** Adotar conceitos Fabric como MSP light-weight ou integração com `rustls` + certificados client-side para evoluir o projeto rumo a redes permissionadas menores (consórcio privado).

## 4. Consenso e Fluxo de Transações

| Etapa | Blockchain-GRPC | Hyperledger Fabric |
| --- | --- | --- |
| **Proposta** | Cliente gRPC chama `AddBlock`, serviço carrega grafo e monta bloco. | Cliente envia proposta para peers endossadores que executam chaincode. |
| **Validação** | `BlockchainGraph::add_block` checa hashes, alturas e cross references. | Endorsers verificam chaincode determinístico e colam resultados (RW sets). |
| **Ordenação** | PoW local garante ordem linear dentro do grafo. | Ordering service agrega transações em blocos por canal, usando Raft/Kafka/BFT. |
| **Commit** | Bloco salvo em RocksDB; cache em memória atualizado. | Peers validam políticas e RW sets, atualizam ledger + world state. |

**Divergência-chave:** Hyperledger separa validação/execução (peers) de ordenação, permitindo throughput e governança distribuída. Blockchain-GRPC mantém tudo em um único processo, adequado para protótipos rápidos e pipelines offline.

## 5. Modelo de Dados e Persistência

- **Blockchain-GRPC** organiza dados em múltiplos grafos (Transaction, Identity, Asset, Audit, Custom). Cada grafo é um ledger linear persistido com prefixos em RocksDB (`block:{graph}:{height}`, `latest:{graph}` etc.). Cross-references ligam grafos distintos.
- **Fabric** usa dois componentes por canal: (a) blockchain imutável com blocos ordenados, (b) world state (tipicamente LevelDB/CouchDB) contendo o estado atual chave-valor. Referências cruzadas são implementadas via chaincode (ex.: relacionar ativos e identidades em um mesmo canal).

**Paralelo conceitual:** Os multi-grafos funcionam como “canais temáticos”. Adicionar suporte a world state derivado (ex.: índices em RocksDB ou SQLite) aproximaria o projeto da ergonomia de consultas no Fabric.

## 6. Lógica de Negócio e Smart Contracts

- **Blockchain-GRPC** encapsula orquestrações em `BlockchainServiceImpl` e casos de uso dedicados. A extensão de lógica ocorre adicionando novos métodos gRPC + use cases, ou compondo micro-serviços (ex.: REST bridge em .NET).
- **Fabric** delega lógica aos chaincodes, executados determinística e isoladamente. Cada chaincode manipula o world state via API key-value e respeita políticas de endorsement.

**Equivalência:** O `BlockchainServiceImpl` funciona como um chaincode global, e os módulos `application/use_cases/*` equivalem a transações (Invoke) no Fabric. Para aproximar dos smart contracts, é possível permitir que outros binários (ou WebAssembly) pluguem implementações específicas para cada grafo.

## 7. Comunicação, APIs e Integração

- **Blockchain-GRPC** oferece uma API gRPC externa (tonic), um proxy HTTP/Axum (`src/bin/http_proxy.rs`) e um cliente C# (Smart-Contract). Tudo exposto publicamente para consumidores externos.
- **Fabric** usa gRPC internamente para comunicação entre peers/orderers, mas expõe SDKs (Node, Go, Java, Python, C#) que negociam identidades, políticas e envio de propostas/blocos.

**Ponto comum:** Ambos adotam gRPC como camada de transporte eficiente. Diferença: no Fabric, o gRPC é interno e autenticado; no Blockchain-GRPC é uma API pública. Adicionar interceptors de autenticação e metadados (peer/channel) alinharia ainda mais os modelos.

## 8. Observabilidade, Deploy e Operações

- **Blockchain-GRPC** conta com Docker multi-stage (cargo-chef), entrypoint que inicializa `config.json`, executa como usuário não root e inclui `grpc_health_probe`. Logs usam `tracing`. Build otimizado via `.cargo/config.toml` + `sccache`/`lld` + scripts PowerShell/Bash.
- **Fabric** depende de docker-compose/Kubernetes com múltiplos containers (CA, peer, orderer, CouchDB). Observabilidade é feita com Prometheus, Grafana, ELK e ferramentas do Fabric Operations Console.

**Convergência possível:** Instrumentar métricas gRPC/Prometheus no Rust e publicar dashboards inspirados nos visuais do Fabric (taxa de blocos, tempo de validação, falhas de endorsement vs falhas de PoW).

## 9. Conceitos Compartilhados e Dissonantes

| Conceito | Similaridade | Diferenciação |
| --- | --- | --- |
| **Imutabilidade** | Ambos mantêm histórico append-only, detectando alterações via hash. | Fabric adiciona world state mutável; projeto usa apenas ledger linear por grafo. |
| **Modularidade** | Módulos Rust (domínio/infrastrutura) ↔ componentes Fabric (peer/orderer/chaincode). | Fabric exige múltiplos nós por organização; projeto é monolito escalável verticalmente. |
| **Consenso Pluggable** | Projeto pode trocar PoW por outro algoritmo (grafo = canal). | Fabric já possui abstração de ordering service com algoritmos prontos. |
| **Identidade/Permissão** | Ambos reconhecem a necessidade de acesso controlado. | Fabric traz MSP/CA nativos; projeto ainda não implementou PKI. |
| **Interop / SDKs** | gRPC + REST + cliente .NET ↔ SDKs Fabric (Node/Go/Java/Python/C#). | Fabric SDK inclui fluxo completo de inscrição, assinatura e políticas; projeto expõe endpoints diretos. |

## 10. Recomendações para Aproximação com Fabric

1. **Introduzir um módulo de identidade** inspirado no MSP (ex.: emitir certificados com `rustls` ou integrar com Fabric CA). Isso permitiria grafos privados e políticas de validação.
2. **Separar funções de ordenação e validação** em processos/containers distintos. Um “graph-orderer” poderia receber blocos de múltiplos serviços e aplicar consenso BFT ou Raft, aproximando o pipeline Fabric.
3. **Adicionar um world state derivado** (ex.: RocksDB column families, SQLite ou PostgreSQL) para consultas rápidas e para suportar operações `query` vs `invoke`, tal como Fabric separa ledger e state.
4. **Plugar políticas de validação configuráveis** (endorsement). Cada grafo poderia exigir assinaturas de serviços externos ou validações customizadas antes de aceitar um bloco.
5. **Fornecer ferramentas de operação multi-peer**, como scripts para gerar certificados, subir múltiplos nós e monitorá-los, em linha com os samples `fabric-samples`.

## 11. Quando Usar Cada Solução

| Cenário | Blockchain-GRPC | Hyperledger Fabric |
| --- | --- | --- |
| **Prototipagem rápida, POCs e laboratórios** | ✅ APIs simples, single node, fácil extender em Rust/.NET. | ⚠️ Exige setup complexo de múltiplos nós. |
| **Plataformas corporativas multi-organização** | ⚠️ Necessita evoluir governança e consenso. | ✅ Nativamente desenhado para consórcios permissionados. |
| **Casos de uso customizados (multi-grafos, cross references)** | ✅ Estrutura de grafos fornece isolação e vínculo sem canais separados. | ✅ Canais separados + chaincode para relacionar dados. |
| **Compliance forte (auditoria, PKI, políticas)** | ⚠️ Precisa roadmap de identidade. | ✅ MSP, policies e auditoria integrados. |

## 12. Conclusão

- O **Blockchain-GRPC** entrega uma base Rust moderna com arquitetura limpa, multi-grafos, persistência sólida e um pipeline gRPC bem definido — ideal para squads menores que desejam dominar conceitos de blockchain enterprise antes de partir para redes distribuídas plenas.
- O **Hyperledger Fabric** fornece o ecossistema completo de redes permissionadas, com identidade forte, consenso modular e ferramentas operacionais, mas com curva de configuração maior.

**Estratégia sugerida:** usar o Blockchain-GRPC como laboratório para testar modelos de dados, fluxos de validação e integrações (gRPC/REST), e migrar gradualmente conceitos bem-sucedidos para redes Fabric — ou incorporar no próprio projeto os elementos de identidade, políticas e ordenação inspirados na plataforma Hyperledger.
