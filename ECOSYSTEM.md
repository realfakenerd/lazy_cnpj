# Ecossistema Ratatui & Crates Selecionadas (`ECOSYSTEM.md`)

Este documento analisa as bibliotecas, widgets e utilitários da comunidade **Awesome Ratatui** recomendados para integrar e expandir o **`lazy_cnpj`**.

---

## 1. Mapeamento de Widgets Padrão do Ratatui

Todas as funcionalidades principais do `lazy_cnpj` utilizam widgets nativos do Ratatui ou composições simples:

| Funcionalidade Planejada | Widget Nativo Ratatui | Aplicação no `lazy_cnpj` |
| :--- | :--- | :--- |
| **Navegação Principal** | `Tabs` | Alterna entre as abas (`[1] Pesquisa`, `[2] Exportação`, `[3] Atualizar Base`). |
| **Tabelas de Resultados** | `Table` + `TableState` | Exibe a lista de empresas com colunas dinâmicas, navegação por setas (`Up`/`Down`) e linha ativa. |
| **Filtros e Inputs** | `Paragraph` + `Block` | Caixas de exibição de filtros ou termos buscados. |
| **Modal 360° da Empresa** | `Clear` + `Block` + `Layout` | Camada modal flutuante sobreposta à tabela para detalhamento cadastral. |
| **Gauges de Download & ETL** | `Gauge` / `LineGauge` | Barras de progresso dinâmicas de download e inserção no SQLite. |
| **Métricas e Dashboards** | `BarChart` / `Sparkline` | Gráficos estatísticos (ex: distribuição de empresas por UF). |
| **Seleção de Opções / Presets** | `List` + `ListState` | Seleção de colunas para exportação e arquivos no ETL. |
| **Rolagem em Modais** | `Scrollbar` + `ScrollbarState` | Barras de rolagem vertical e horizontal. |

---

## 2. Crates Recomendadas da Comunidade (`Awesome Ratatui`)

### 2.1 Widgets & Componentes Avançados

| Crate / Biblioteca | O que faz no `lazy_cnpj` | Por que usar? |
| :--- | :--- | :--- |
| [`ratatui-textarea`](https://crates.io/crates/ratatui-textarea) | Caixa de texto editável com cursor, navegação e desfazer (`Ctrl+Z`). | Resolve a digitação interativa nos campos de busca (Razão Social, CNPJ, UF). |
| [`throbber-widgets-tui`](https://crates.io/crates/throbber-widgets-tui) | Spinners e animações de carregamento no terminal. | Exibe animações de "Buscando no banco..." ou "Processando lote ZIP..." na barra de status. |
| [`tui-scrollview`](https://crates.io/crates/tui-scrollview) | Contêiner com rolagem automática para áreas maiores que a tela. | Permite rolar com o mouse/teclado no Modal 360° da Empresa em telas menores. |
| [`ratatui-comfy-toaster`](https://crates.io/crates/ratatui-comfy-toaster) | Notificações "Toast" flutuantes. | Emite avisos visuais rápidos (*"CNPJ copiado!"*, *"Exportação concluída!"*). |
| [`tui-tree-widget`](https://crates.io/crates/tui-tree-widget) | Exibição de dados em árvore retrátil (`Tree`). | Exibe a hierarquia de CNAEs (Seção -> Divisão -> Grupo -> Classe -> Subclasse) ou Matriz/Filiais. |

---

### 2.2 Utilitários & Estilização

| Crate / Biblioteca | O que faz no `lazy_cnpj` | Por que usar? |
| :--- | :--- | :--- |
| **Macros Nativas do Ratatui** (`ratatui::macros`) | Macros (`constraints!`, `vertical!`, `horizontal!`, `span!`, `line!`) para definir layouts e textos rapidamente. | **Já integradas nativamente na crate `ratatui` (v0.26+)**. Não requer crate externa adicional! |
| [`tui-logger`](https://crates.io/crates/tui-logger) | Logger assíncrono com widget de logs na TUI. | Captura mensagens do `tracing` / `log` do Rust e permite exibi-las em uma aba/painel de diagnóstico (`[F12] Logs`). |
| [`opaline`](https://crates.io/crates/opaline) | Gerenciador de temas baseados em tokens e suporte a gradientes. | Permite trocar o tema visual do `lazy_cnpj` (Dracula, Nord, Catppuccin, Gruvbox) via atalho. |
| [`tachyonfx`](https://github.com/junkdog/tachyonfx) | Biblioteca de efeitos visuais e animação de transição no terminal. | Adiciona transições suaves ao abrir popups e modais. |

---

## 3. Re-exportações Nativas e Feature Flags do Ratatui

O **`ratatui`** re-exporta diretamente vários sub-crates e integrações internas, o que simplifica a gestão de dependências no `Cargo.toml`:

### 3.1 Sub-crates Re-exportadas
- **`crossterm`** (via feature flag `crossterm`): Re-exportado como `ratatui::crossterm`. Não é necessário depender diretamente de `crossterm` em projetos simples!
- **`ratatui-macros`**: Re-exportado nativamente em `ratatui::macros` (ex: `constraints!`, `vertical!`, `horizontal!`).
- **`ratatui-widgets`**: Re-exportado em `ratatui::widgets` (ex: `Table`, `Block`, `Paragraph`, `Gauge`, `Tabs`).
- **`ratatui-core`**: Contém estruturas primárias como `Buffer`, `Terminal`, `Frame` e `Rect`.
- **`palette`** (via feature flag `palette`): Integra a biblioteca de cores `palette` para conversões avançadas de cores em RGB/HSL.
- **`serde`** (via feature flag `serde`): Permite serializar e deserializar objetos de estilo (`Style`, `Color`) diretamente com Serde.

---

## 4. Configuração Enxuta Recomendada para o `Cargo.toml`

Aproveitando todas as re-exportações do `ratatui`, a lista final de dependências fica extremamente limpa:

```toml
[dependencies]
# Ratatui com suporte a crossterm e macros embutidas
ratatui = { version = "0.30", features = ["crossterm", "macros"] }

# Async runtime e banco de dados SQLite
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.32", features = ["bundled"] }

# Parsers & Streams
csv = "1.3"
zip = "2.2"
reqwest = { version = "0.12", features = ["stream"] }
serde = { version = "1", features = ["derive"] }

# Utilitários complementares de UI da comunidade
ratatui-textarea = "0.4"      # Campo de texto interativo com cursor
throbber-widgets-tui = "0.8"  # Spinners de carregamento no ETL
tui-scrollview = "0.2"        # Rolagem no modal de detalhes da empresa
tui-logger = "0.14"           # Painel de diagnóstico/logs em tempo real
```
