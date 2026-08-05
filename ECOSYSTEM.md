# Ratatui Ecosystem & Selected Crates (`ECOSYSTEM.md`)

This document analyzes the libraries, widgets, and utilities from the **Awesome Ratatui** community recommended for integration into **`lazy_cnpj`**.

---

## 1. Native Ratatui Widget Mapping

All core features of `lazy_cnpj` utilize native Ratatui widgets or simple compositions:

| Planned Feature | Native Ratatui Widget | Application in `lazy_cnpj` |
| :--- | :--- | :--- |
| **Main Navigation** | `Tabs` | Switches between main tabs (`[1] Search`, `[2] Export`, `[3] Update`). |
| **Results Table** | `Table` + `TableState` | Displays company list with dynamic columns, arrow navigation (`Up`/`Down`), and active row highlighting. |
| **Filters & Inputs** | `Paragraph` + `Block` | Display boxes for search terms and applied filters. |
| **Company 360° Modal** | `tui-overlay` / `Clear` + `Block` | Floating modal layer over the table for detailed company registration data. |
| **ETL & Download Gauges** | `Gauge` / `LineGauge` | Dynamic progress bars for downloads and SQLite insertions. |
| **Metrics & Dashboards** | `BarChart` / `Sparkline` | Statistical charts (e.g., company distribution by State/UF). |
| **Presets & Option List** | `List` + `ListState` | Column selection for CSV export and file selection during ETL. |
| **Modal Scrollbars** | `Scrollbar` + `ScrollbarState` | Vertical and horizontal scrollbars for large details view. |

---

## 2. Recommended Community Crates (`Awesome Ratatui`)

### 2.1 Advanced Widgets & Components

| Crate / Library | Role in `lazy_cnpj` | Why use it? |
| :--- | :--- | :--- |
| [`ratatui-textarea`](https://crates.io/crates/ratatui-textarea) | Editable text box with cursor, navigation, and undo (`Ctrl+Z`). | Manages interactive typing in search fields (Company Name, CNPJ, UF). |
| [`throbber-widgets-tui`](https://crates.io/crates/throbber-widgets-tui) | Spinners and loading animations in the terminal. | Displays loading spinners for "Searching database..." or "Processing ZIP batch...". |
| [`tui-overlay`](https://github.com/ratatui/tui-overlay) | Composable overlay, popup, and modal system. | Renders the Company 360° Modal view with flexible positioning and layer management. |
| [`tui-scrollview`](https://crates.io/crates/tui-scrollview) | Auto-scrolling container for content larger than screen size. | Enables mouse/keyboard scrolling inside the 360° Company Modal. |
| [`ratatui-comfy-toaster`](https://crates.io/crates/ratatui-comfy-toaster) | Floating toast notifications. | Displays quick visual alerts (*"CNPJ copied!"*, *"Export completed!"*). |
| [`tui-tree-widget`](https://crates.io/crates/tui-tree-widget) | Collapsible tree view widget (`Tree`). | Displays CNAE economic activity hierarchy or Parent/Branch relationships. |

---

### 2.2 Utilities & Styling

| Crate / Library | Role in `lazy_cnpj` | Why use it? |
| :--- | :--- | :--- |
| **Ratatui Native Macros** (`ratatui::macros`) | Layout macros (`constraints!`, `vertical!`, `horizontal!`, `span!`, `line!`). | **Natively integrated into `ratatui` (v0.26+)**. No extra crate required! |
| [`tui-logger`](https://crates.io/crates/tui-logger) | Async logger with in-TUI log widget. | Captures Rust `tracing`/`log` messages and displays them in a diagnostic tab (`[F12] Logs`). |
| [`opaline`](https://crates.io/crates/opaline) | Token-based theme manager with gradient support. | Enables quick visual theme switching (`Dracula`, `Nord`, `Catppuccin`, `Gruvbox`). |
| [`tachyonfx`](https://github.com/junkdog/tachyonfx) | Visual effects and terminal animation library. | Adds smooth transitions when opening popups and modals. |

---

## 3. Ratatui Feature Flags & Native Re-exports

**`ratatui`** re-exports several sub-crates directly, simplifying `Cargo.toml` dependency management:

### 3.1 Re-exported Sub-crates
- **`crossterm`** (via `crossterm` feature): Re-exported as `ratatui::crossterm`.
- **`ratatui-macros`**: Re-exported natively at `ratatui::macros`.
- **`ratatui-widgets`**: Re-exported at `ratatui::widgets` (`Table`, `Block`, `Paragraph`, `Gauge`, `Tabs`).
- **`ratatui-core`**: Core structures like `Buffer`, `Terminal`, `Frame`, and `Rect`.

---

## 4. Lean `Cargo.toml` Configuration

```toml
[dependencies]
# Ratatui with embedded crossterm and macros
ratatui = { version = "0.30", features = ["crossterm", "macros"] }

# Async runtime & Diesel SQLite ORM
tokio = { version = "1", features = ["full"] }
diesel = { version = "2.2", features = ["sqlite", "returning"] }
diesel_migrations = "2.2"

# Parsers & Streams
csv = "1.3"
zip = "2.2"
reqwest = { version = "0.12", features = ["stream"] }
serde = { version = "1", features = ["derive"] }

# UI & Community Utilities
ratatui-textarea = "0.4"      # Interactive text field
throbber-widgets-tui = "0.8"  # Loading spinners
tui-overlay = "0.1"           # Modal & Popup overlay manager
tui-scrollview = "0.2"        # Modal scrolling area
tui-logger = "0.14"           # Real-time diagnostic logger panel
```
