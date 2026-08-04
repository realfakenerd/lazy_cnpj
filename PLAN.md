# Plano de Implementação Final - `lazy_cnpj` (Ratatui TUI)

Este documento consolida a arquitetura técnica e o design de interface do **`lazy_cnpj`**.

---

## 1. Visão Geral da Arquitetura & Motor de Carga Concorrente Adaptativo

### Motor de ETL Adaptativo em Rust (Tokio + Streams + Memory Zip)
O sistema detecta em tempo real os recursos do hardware (RAM e Espaço em Disco) e ajusta dinamicamente a taxa de concorrência:

1. **Diagnóstico do Perfil de Hardware (`src/etl/monitor.rs`)**:
   - **Perfil Conservador (RAM < 4GB ou Disco < 15GB)**:
     - 1 Download por vez + 1 Streamer de Leitura em Memória + Exclusão imediata.
   - **Perfil Moderado (RAM 4GB~8GB e Disco > 15GB)**:
     - 2 Downloads simultâneos + 1 Worker de Inserção SQLite.
   - **Perfil Alta Performance (RAM > 8GB e Disco > 30GB)**:
     - 3 a 4 Downloads simultâneos em paralelo + Pipeline de múltiplos workers descompactando e inserindo no SQLite simultaneamente.

2. **Parsing Direct em Memória (Sem Extração em Disco)**:
   - Utilização da crate `zip` integrada com a crate `csv` para ler dados diretamente da stream/buffer de memória do arquivo ZIP sem descompactá-lo no HD.

---

## 2. Estrutura dos Arquivos (`src/`)

```text
lazy_cnpj/
├── Cargo.toml
└── src/
    ├── main.rs                 # Loop principal Tokio/Crossterm/Ratatui
    ├── app.rs                  # Gestão de estado global
    ├── db/
    │   ├── mod.rs              # Conexão SQLite (WAL Mode + Mutex/Arc para concorrência)
    │   ├── schema.rs           # Definição de tabelas e índices
    │   └── queries.rs          # Queries de busca e stream de exportação
    ├── etl/
    │   ├── monitor.rs          # Sensor de hardware (RAM Livre e Espaço em Disco com `sysinfo`)
    │   ├── downloader.rs       # Streamer HTTP concorrente (Tokio + `reqwest`)
    │   ├── zip_streamer.rs     # Parser de ZIP em memória (crate `zip` + `csv`)
    │   └── importer.rs         # Gerenciador de workers concorrentes de inserção
    ├── exporter/
    │   └── csv_writer.rs       # Streamer de exportação SQL -> CSV
    ├── ui/
    │   ├── mod.rs              # Layout base (Header, Body, Footer, Help Popup)
    │   ├── search_tab.rs       # Layout da aba Pesquisa
    │   ├── export_tab.rs       # Layout da aba Exportação
    │   ├── update_tab.rs       # Layout da aba Update (Gauges de Concorrência Adaptativa)
    │   └── help_popup.rs       # Popup modal de ajuda
    ├── utils/
    │   ├── clipboard.rs        # Gerenciador da Área de Transferência (`arboard`)
    │   └── launcher.rs         # Utilitário para abrir URLs no navegador (`open`)
    └── models/
        ├── company.rs          # Modelos de dados
        └── export_config.rs    # Opções e colunas da exportação CSV
```

---

## 3. Dependências Adicionadas ao Cargo.toml

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.32", features = ["bundled"] }
csv = "1.3"
zip = "2.2"
reqwest = { version = "0.12", features = ["stream"] }
sysinfo = "0.31"   # Monitoramento de hardware (RAM e Espaço Livre)
open = "5.3"       # Abertura de links no navegador default
arboard = "3.4"    # Cópia para área de transferência do SO
```

---

## 4. Plano de Execução

1. Inicializar o repositório Cargo (`cargo init`).
2. Adicionar as dependências no `Cargo.toml`.
3. Criar os utilitários de sistema e o sensor de hardware adaptativo (`etl/monitor.rs`).
4. Construir o motor concorrente de downloads/stream e a TUI Ratatui.
