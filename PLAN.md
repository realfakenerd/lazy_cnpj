# Final Implementation Plan - `lazy_cnpj` (Ratatui TUI)

This document consolidates the technical architecture and interface design for **`lazy_cnpj`**.

---

## 1. Architecture Overview & Adaptive Concurrent ETL Engine

### Adaptive ETL Engine in Rust (Tokio + MPSC + Disk Pipeline)
The system operates on an optimized pipeline with adaptive concurrency based on hardware resources:

1. **4-Stage ETL Pipeline (Download > Unzip > Parse/Sanitize > Batch Insert > Cleanup)**:
   - **Temporary Download**: The `.zip` file from Receita Federal is downloaded into the temporary folder `.cache/downloads/`.
   - **Unzip & Stream Reading**: The CSV is extracted temporarily and read via `BufReader` + `csv::Reader`.
   - **MPSC Channel (Multi-Producer, Single-Consumer)**: Multiple parsing workers convert CSV rows into sanitized Rust structs in parallel and send them into an async `tokio::sync::mpsc` channel.
   - **Single SQLite DB Writer Worker**: A dedicated worker consumes from the MPSC channel and inserts records into SQLite in massive transactions (batches of 50,000~100,000 records), avoiding `SQLITE_BUSY` / `DatabaseLocked` errors.
   - **Cleanup**: After each file chunk is processed, the temporary `.zip` and `.csv` files are immediately deleted from disk.

2. **Monthly Database Update (Atomic Swap DB Strategy)**:
   - For full monthly reloads without locking or corrupting the active database, the ETL creates a temporary database `lazy_cnpj_temp.db`.
   - All data insertion occurs in the temporary database **without active indexes** for maximum insertion speed.
   - After insertion completes, B-Tree indexes and FTS5 virtual tables are built in a single pass.
   - An atomic file swap is performed: `lazy_cnpj_temp.db` is renamed to replace `lazy_cnpj.db`.

3. **Hardware Sensor & Diagnostics (`src/etl/monitor.rs`)**:
   - **Conservative Profile (RAM < 4GB or Disk < 15GB)**:
     - 1 Download at a time + Sequential pipeline.
   - **Moderate Profile (RAM 4GB~8GB and Disk > 15GB)**:
     - 2 Concurrent downloads + 2 CSV Parsing Workers -> 1 DB Writer.
   - **High Performance Profile (RAM > 8GB and Disk > 30GB)**:
     - 3 to 4 Concurrent downloads + 4 CSV Parsing Workers -> 1 Batch DB Writer.

---

## 2. Directory & Source Structure (`src/`)

```text
lazy_cnpj/
├── Cargo.toml
└── src/
    ├── main.rs                 # Tokio/Crossterm/Ratatui main event loop
    ├── app.rs                  # Global application state management
    ├── db/
    │   ├── mod.rs              # SQLite connection pool & Diesel setup
    │   ├── schema.rs           # Diesel schema mapping definitions
    │   └── queries.rs          # Search queries, FTS5 lookup & export streaming
    ├── etl/
    │   ├── monitor.rs          # Hardware sensor (Free RAM & Disk space via `sysinfo`)
    │   ├── downloader.rs       # Concurrent HTTP streamer (Tokio + `reqwest`)
    │   ├── zip_streamer.rs     # Disk unzip & CSV parser pipeline (`zip` + `csv`)
    │   └── importer.rs         # MPSC channel & batch insertion worker manager
    ├── exporter/
    │   └── csv_writer.rs       # SQL query to CSV/JSON streaming exporter
    ├── ui/
    │   ├── mod.rs              # Base TUI layout (Header, Body, Footer, Overlay manager)
    │   ├── search_tab.rs       # Search Tab layout & table view
    │   ├── export_tab.rs       # Export configuration Tab layout
    │   ├── update_tab.rs       # Update Tab layout (Adaptive ETL progress gauges)
    │   └── help_popup.rs       # Keyboard shortcuts & help overlay modal
    ├── utils/
    │   ├── clipboard.rs        # OS Clipboard helper (`arboard`)
    │   └── launcher.rs         # Default browser URL opener (`open`)
    └── models/
        ├── company.rs          # Domain & Diesel model structs
        └── export_config.rs    # Export options & column selections
```

---

## 3. Cargo.toml Dependencies

```toml
[dependencies]
# UI & Terminal
ratatui = { version = "0.30", features = ["crossterm", "macros"] }
ratatui-textarea = "0.4"      # Interactive text field with cursor support
throbber-widgets-tui = "0.8"  # Loading spinners for ETL status
tui-overlay = "0.1"           # Overlay, popups, and modal drawer manager
tui-scrollview = "0.2"        # Scrollable container for company 360 view
tui-logger = "0.14"           # Real-time diagnostic logger panel

# Async Runtime & Hardware Monitoring
tokio = { version = "1", features = ["full"] }
sysinfo = "0.31"   # Hardware sensor (RAM and free disk space)

# SQLite Database (Diesel ORM)
diesel = { version = "2.2", features = ["sqlite", "returning"] }
diesel_migrations = "2.2"

# ETL, Parsers & Streams
csv = "1.3"
zip = "2.2"
reqwest = { version = "0.12", features = ["stream"] }
serde = { version = "1", features = ["derive"] }

# System Utilities
open = "5.3"       # Open URLs in system default browser
arboard = "3.4"    # OS Clipboard integration
```

---

## 4. Execution Plan

1. Initialize the Cargo repository (`cargo init`).
2. Populate `Cargo.toml` with the specified dependencies.
3. Build system utilities and adaptive hardware monitor (`etl/monitor.rs`).
4. Construct the concurrent MPSC ETL pipeline and the Ratatui TUI engine.
