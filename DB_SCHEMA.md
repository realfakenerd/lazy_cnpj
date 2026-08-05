# SQLite Database Schema (`DB_SCHEMA.md`) - Extreme Tuning (v3.0)

This document details the **SQLite** database schema planning for the **`lazy_cnpj`** application, based on Rust data structures and Receita Federal's public CNPJ dataset.

---

## 1. Overview & Performance Strategy (Extreme Tuning)

The public CNPJ database contains ~45 million records. To shrink database storage from **~40 GB to ~8 GB** and achieve sub-millisecond query response times (**< 2ms**), the system enforces the following guidelines:

1. **Optimized Pragma Settings**:
   - `PRAGMA page_size = 16384;` (16KB page size for shallow B-Trees during massive read operations).
   - `PRAGMA journal_mode = WAL;` (Write-Ahead Logging for concurrent reads and writes).
   - `PRAGMA synchronous = NORMAL;` during ETL loading.
   - `PRAGMA cache_size = -128000;` (~128MB RAM cache per connection).
   - `PRAGMA temp_store = MEMORY;`
   - `PRAGMA foreign_keys = OFF;` temporarily disabled during batch insertion.

2. **Compact Data Types**:
   - **`WITHOUT ROWID`**: Applied to all tables with a unique Primary Key, eliminating hidden B-Tree overhead.
   - **`INTEGER` for CNPJs**: `cnpj_basico`, `cnpj_ordem`, and `cnpj_dv` stored as 64-bit integers (`u32`/`u64` varints).
   - **`INTEGER` for CNAE**: 7-digit numeric codes (`6201500`) instead of text strings.
   - **`INTEGER` for Dates**: Stored as numeric `YYYYMMDD` integers (`20051103`) instead of text.

3. **Indexes & Post-Processing Strategy**:
   - B-Tree indexes and FTS5 virtual tables are built **exclusively after full batch ingestion**.
   - Automatic execution of `PRAGMA optimize;` and `VACUUM;` upon ETL pipeline completion.

---

## 2. DDL Schema (Data Definition Language)

### 2.1 Domain Lookup Tables

```sql
-- Table: PAISES (Countries)
CREATE TABLE IF NOT EXISTS paises (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
) WITHOUT ROWID;

-- Table: MUNICIPIOS (Municipalities with State/UF mapping)
CREATE TABLE IF NOT EXISTS municipios (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL,
    uf TEXT NOT NULL
) WITHOUT ROWID;

-- Table: QUALIFICACOES_SOCIOS (Partner Qualifications)
CREATE TABLE IF NOT EXISTS qualificacoes_socios (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
) WITHOUT ROWID;

-- Table: NATUREZAS_JURIDICAS (Legal Natures)
CREATE TABLE IF NOT EXISTS naturezas_juridicas (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
) WITHOUT ROWID;

-- Table: CNAES (Economic Activities - 7-digit numeric primary key)
CREATE TABLE IF NOT EXISTS cnaes (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
) WITHOUT ROWID;
```

---

### 2.2 Main Entity Tables (Optimized - Extreme Tuning)

```sql
-- Table: EMPRESAS (No hidden ROWID + Integer CNPJ Base)
CREATE TABLE IF NOT EXISTS empresas (
    cnpj_basico INTEGER PRIMARY KEY, -- Stored as u32 (1 to 4 bytes varint)
    razao_social TEXT NOT NULL,
    natureza_juridica INTEGER NOT NULL,
    qualificacao_responsavel INTEGER NOT NULL,
    capital_social REAL NOT NULL DEFAULT 0.0,
    porte_empresa INTEGER NOT NULL, -- Enum u8 (1: ME, 3: EPP, 5: DEMAIS)
    ente_federativo_responsavel TEXT
) WITHOUT ROWID;

-- Table: ESTABELECIMENTOS (Integer CNPJ, CNAE, and Numeric Dates)
CREATE TABLE IF NOT EXISTS estabelecimentos (
    cnpj_basico INTEGER NOT NULL,
    cnpj_ordem INTEGER NOT NULL,
    cnpj_dv INTEGER NOT NULL,
    identificador_matriz_filial INTEGER NOT NULL, -- 1: Head office, 2: Branch
    nome_fantasia TEXT,
    situacao_cadastral INTEGER NOT NULL,
    data_situacao_cadastral INTEGER, -- Numeric YYYYMMDD
    motivo_situacao_cadastral INTEGER,
    nome_cidade_exterior TEXT,
    pais INTEGER,
    data_inicio_atividade INTEGER,   -- Numeric YYYYMMDD
    cnae_fiscal_principal INTEGER NOT NULL, -- 7-digit numeric CNAE
    cnae_fiscal_secundaria TEXT,     -- Comma-separated list of secondary CNAEs
    tipo_logradouro TEXT,
    logradouro TEXT,
    numero TEXT,
    complemento TEXT,
    bairro TEXT,
    cep TEXT,
    municipio INTEGER, -- FK to municipios(codigo) - UF resolved via JOIN
    telefone_1 TEXT,   -- Concatenated DDD + Number in Rust (e.g., "(11) 98888-7777")
    telefone_2 TEXT,
    correio_eletronico TEXT,
    situacao_especial TEXT,
    data_situacao_especial INTEGER,  -- Numeric YYYYMMDD
    PRIMARY KEY (cnpj_basico, cnpj_ordem, cnpj_dv)
) WITHOUT ROWID;

-- Table: DADOS_SIMPLES (Compact Tax Status + Integer Flags)
CREATE TABLE IF NOT EXISTS dados_simples (
    cnpj_basico INTEGER PRIMARY KEY,
    opcao_simples INTEGER,        -- 1: Yes, 0: No, NULL: Other
    data_opcao_simples INTEGER,   -- Numeric YYYYMMDD
    data_exclusao_simples INTEGER,-- Numeric YYYYMMDD
    opcao_mei INTEGER,            -- 1: Yes, 0: No, NULL: Other
    data_opcao_mei INTEGER,       -- Numeric YYYYMMDD
    data_exclusao_mei INTEGER     -- Numeric YYYYMMDD
) WITHOUT ROWID;

-- Table: SOCIOS (Partners / Board Members)
CREATE TABLE IF NOT EXISTS socios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cnpj_basico INTEGER NOT NULL,
    identificador_socio INTEGER NOT NULL,
    nome_socio_razao_social TEXT NOT NULL,
    cnpj_cpf_socio TEXT,
    qualificacao_socio INTEGER NOT NULL,
    data_entrada_sociedade INTEGER, -- Numeric YYYYMMDD
    pais INTEGER,
    representante_legal TEXT,
    nome_representante TEXT,
    qualificacao_representante_legal INTEGER,
    faixa_etaria INTEGER NOT NULL
);
```

---

## 3. High-Performance Indexes & FTS5 (Full-Text Search)

### 3.1 Traditional B-Tree Indexes
```sql
-- Full CNPJ lookups & Integer joins
CREATE INDEX IF NOT EXISTS idx_estab_cnpj_full ON estabelecimentos(cnpj_basico, cnpj_ordem, cnpj_dv);
CREATE INDEX IF NOT EXISTS idx_estab_muni ON estabelecimentos(municipio);
CREATE INDEX IF NOT EXISTS idx_estab_cnae ON estabelecimentos(cnae_fiscal_principal);
CREATE INDEX IF NOT EXISTS idx_estab_data_inicio ON estabelecimentos(data_inicio_atividade);

-- Partner company joins
CREATE INDEX IF NOT EXISTS idx_socios_cnpj_basico ON socios(cnpj_basico);
```

### 3.2 FTS5 Virtual Tables (Instant Substring Search < 2ms)
```sql
-- FTS5 table for free-text search on Company Name and Trade Name
CREATE VIRTUAL TABLE IF NOT EXISTS fts_empresas USING fts5(
    cnpj_basico UNINDEXED,
    razao_social,
    nome_fantasia,
    content='empresas',
    content_rowid='cnpj_basico'
);

-- FTS5 table for instant partner name lookup
CREATE VIRTUAL TABLE IF NOT EXISTS fts_socios USING fts5(
    cnpj_basico UNINDEXED,
    nome_socio,
    content='socios',
    content_rowid='id'
);
```

---

## 4. Type Mapping & Sanitization: Rust <-> SQLite (`diesel`)

| Field / Table | Original CSV Format | SQLite Type | Rust Sanitization Rule (`etl/zip_streamer.rs`) |
| :--- | :--- | :--- | :--- |
| `cnpj_basico`, `ordem`, `dv` | `"00000001"` (String) | `INTEGER` | Parse `u32`. Saves 75% storage per CNPJ. |
| `cnae_fiscal_principal` | `"6201500"` (String) | `INTEGER` | Parse `u32`. Reduces storage space by 50%. |
| Dates (`data_inicio`, etc) | `"YYYYMMDD"` (e.g. `20051103`) | `INTEGER` | Parse `u32` (e.g. `20051103`). Reduces storage by 60%. |
| `capital_social` (Companies) | `"1000,00"` or `"205431,50"` | `REAL` | Replace `,` with `.` -> parse `f64`. Default `0.0`. |
| `opcao_simples` / `mei` | `"S"`, `"N"`, `""` | `INTEGER` | `"S"` -> `1`, `"N"` -> `0`, other -> `NULL`. |
| Phones (`1` and `2`) | Separate Area Code & Number | `TEXT` | Format in Rust: `format!("({}) {}", ddd, tel)` or `NULL`. |
| Text (`razao_social`, etc) | Strings with accents/spaces | `TEXT` | `.trim()` and remove null characters (`\0`). |

---

## 5. Optimized View for Complete Company Display

The View resolves integer keys, reconstructs formatted dates/CNPJs, and retrieves State/UF directly from `municipios`:

```sql
CREATE VIEW IF NOT EXISTS vw_empresa_completa AS
SELECT
    printf('%08d/%04d-%02d', e.cnpj_basico, est.cnpj_ordem, est.cnpj_dv) AS cnpj_completo,
    e.cnpj_basico,
    e.razao_social,
    est.nome_fantasia,
    nj.descricao AS natureza_juridica_desc,
    e.capital_social,
    e.porte_empresa,
    est.situacao_cadastral,
    -- Date formatter YYYYMMDD Integer -> YYYY-MM-DD Text for display
    printf('%04d-%02d-%02d', est.data_inicio_atividade / 10000, (est.data_inicio_atividade % 10000) / 100, est.data_inicio_atividade % 100) AS data_inicio_atividade_formatada,
    est.cnae_fiscal_principal,
    cnae.descricao AS cnae_principal_desc,
    est.tipo_logradouro || ' ' || est.logradouro || ', ' || est.numero AS endereco_completo,
    est.bairro,
    est.cep,
    m.uf,
    m.descricao AS municipio_desc,
    est.telefone_1,
    est.telefone_2,
    est.correio_eletronico,
    ds.opcao_simples,
    ds.opcao_mei
FROM empresas e
JOIN estabelecimentos est ON e.cnpj_basico = est.cnpj_basico
LEFT JOIN dados_simples ds ON e.cnpj_basico = ds.cnpj_basico
LEFT JOIN naturezas_juridicas nj ON e.natureza_juridica = nj.codigo
LEFT JOIN cnaes cnae ON est.cnae_fiscal_principal = cnae.codigo
LEFT JOIN municipios m ON est.municipio = m.codigo;
```
