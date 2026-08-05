# Advanced Features & Resource Roadmap (`FEATURES.md`)

This document consolidates advanced features, utility tools, and UX enhancements for **`lazy_cnpj`**, focusing on the Rust ecosystem and high-performance Terminal User Interface (Ratatui TUI).

---

## 1. Advanced Search & Filter Module (`Search Tab`)

### 1.1 FTS5 (SQLite Full-Text Search)
- **Free Text Search**: Implement **SQLite FTS5** support for Company Name (*Razão Social*), Trade Name (*Nome Fantasia*), and Partner Name (*Nome do Sócio*).
- **Benefit**: Enables partial word and substring matching with sub-millisecond execution times (e.g., searching for "Tech Silva" finds "SILVA TECHNOLOGY AND SERVICES LTD").

### 1.2 Combined Multi-Criteria Filtering
- **State (UF) & City Filter**: Autocomplete dropdown for Brazilian states and municipalities.
- **Capital Stock Range**: Filter companies by minimum and maximum `capital_social`.
- **Registration Status**: Filter active companies (`02 - ATIVA`), or closed/suspended entries.
- **Tax Option Filter**: Toggle checkboxes to include/exclude Simples Nacional or MEI optants.
- **CNAE (Sector) Filter**: Select by primary or secondary economic activity code.

---

## 2. Company Detail Viewer (`Company 360° Modal`)

- **360° Company View**: Pressing `Enter` on any table result opens an overlay modal split into structured tabbed sections:
  - **Registration Data**: Full CNPJ, Company Name, Trade Name, Status, Start Date, Size.
  - **Board of Directors & Partners (QSA)**: Table of partners, age ranges, qualifications, and legal representatives.
  - **Address & Contacts**: Street, Neighborhood, ZIP Code, City, State, Phones, and Email.
  - **Economic Activities**: Main CNAE and expandable list of secondary CNAEs with friendly descriptions.
  - **Tax & History**: Special status reasons, federative entity, and Simples/MEI status.

---

## 3. External Integrations & Quick Action Shortcuts

- **Instant Clipboard Copy (`c` / `Ctrl+C`)**:
  - Copy formatted CNPJ (`XX.XXX.XXX/XXXX-XX`) or raw numbers.
  - Copy complete company data card as JSON / Plain Text.
- **Browser Quick Links (`o` / `Open`)**:
  - Shortcut keys to open the selected company directly on **Google Maps** (address) or official public lookup portals.
- **ZIP Code / Address Validation**:
  - Integration with public APIs (e.g., ViaCEP / BrasilAPI) to enrich or validate address data when online.

---

## 4. Custom Data Export Options (`Export Tab`)

- **Additional Export Formats**:
  - **CSV / TSV**: Configurable delimiter (comma or semicolon for Excel).
  - **JSON / JSON Lines (JSONL)**: For integration with Python/Node data pipelines.
  - **SQLite Dump / Parquet**: For data science workflows requiring filtered database subsets.
- **Predefined Export Presets**:
  - **"Prospecting / Sales" Preset**: CNPJ, Company Name, Email, Phones, State, City, Main CNAE.
  - **"Compliance / Registration" Preset**: Complete registration data + Partners (QSA).
  - **"Geographic" Preset**: CNPJ, State, Municipality, ZIP, Neighborhood.

---

## 5. ETL Management, Updates & Diagnostics (`Update Tab`)

- **Selective Downloads / Partial File Sync**:
  - Allow users to update specific tables (e.g., only update `Companies` or `Municipalities`).
- **File Checksum Verification (MD5/SHA256)**:
  - Validate Receita Federal `.zip` downloads prior to extraction to prevent corrupted data ingestion.
- **Database Statistics Dashboard**:
  - Real-time dashboard showing local database stats:
    - Total registered Companies and Establishments.
    - Distribution by State (Top 5 states with most companies).
    - Percentage of Active vs. Closed companies.
    - Last sync date timestamp.

---

## 6. User Experience & TUI Optimizations

- **Full Mouse Support**:
  - Click to select tabs, scroll wheel support for tables and modals.
- **Customizable Color Themes**:
  - Palette settings in `lazy_cnpj.toml` (`Dracula`, `Nord`, `Gruvbox`, `Catppuccin`).
- **CLI / Headless Mode (No TUI)**:
  - Allow running `lazy_cnpj` directly from the command line for automated headless ETL scripts:
    - Example: `lazy_cnpj update --auto` or `lazy_cnpj export --uf SP --cnae 6201500 --output sp_tech.csv`.

---

## 7. Native Ratatui Widget Mapping

All features mapped directly to native or composite Ratatui widgets:

| Planned Feature | Native Ratatui Widget | Implementation |
| :--- | :--- | :--- |
| **Section Navigation** | `Tabs` | Switches between Search, Export, and Update screens. |
| **Results List** | `Table` + `TableState` | Displays paginated/scrollable tables with selection and custom columns. |
| **Filter Inputs** | `Paragraph` + `Block` / `ratatui-textarea` | Interactive input boxes for search terms and filters. |
| **360° View Popup** | `tui-overlay` / `Clear` + `Block` | Render floating modal overlay with internal tab layout. |
| **ETL Progress** | `Gauge` / `LineGauge` | Dynamic progress bars for downloads and SQLite inserts. |
| **Dashboard Charts** | `BarChart` / `Sparkline` | Bar charts for state distribution or ETL throughput. |
| **Export Column Selector** | `List` + `ListState` | Multi-select checkboxes for export columns. |
| **Modal Scrollbars** | `Scrollbar` + `ScrollbarState` | Scrollbars for lengthy company descriptions. |
