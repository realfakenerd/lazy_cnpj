# Estrutura do Banco de Dados SQLite (`DB_SCHEMA.md`)

Este documento detalha o planejamento da estrutura do banco de dados **SQLite** para a aplicação **`lazy_cnpj`**, com base nas estruturas de dados Rust dos dados públicos do CNPJ fornecidos pela Receita Federal.

---

## 1. Visão Geral e Estratégia de Desempenho

A base de dados pública do CNPJ contém dezenas de milhões de registros (especialmente nas tabelas `empresas`, `estabelecimentos` e `socios`). Para garantir inserções ultrarrápidas na carga de ETL e buscas instantâneas na TUI, a modelagem adota as seguintes estratégias:

1. **Configurações Pragma Otimizadas (Sessão de Carga/ETL)**:
   - `PRAGMA journal_mode = WAL;` (Write-Ahead Logging para leitura/escrita concorrente).
   - `PRAGMA synchronous = NORMAL;` ou `OFF` durante o ETL em lote.
   - `PRAGMA cache_size = -64000;` (~64MB de cache em RAM por conexão).
   - `PRAGMA temp_store = MEMORY;`
   - `PRAGMA foreign_keys = OFF;` (Desativado temporariamente na carga para máxima performance de inserção).

2. **Tipos de Dados SQLite**:
   - `TEXT`: `cnpj_basico`, `razao_social`, `nome_fantasia`, `cnae_fiscal_principal`, `cnae_fiscal_secundaria`, `cep`, `uf`, datas (`YYYY-MM-DD` ou `YYYYMMDD`), telefones, e-mails e descrições.
   - `INTEGER`: Códigos numéricos (`natureza_juridica`, `qualificacao_responsavel`, `situacao_cadastral`, `motivo_situacao_cadastral`, `pais`, `municipio`, `qualificacao_socio`, `identificador_socio`, `identificador_matriz_filial`, `faixa_etaria`).
   - `REAL` / `NUMERIC`: `capital_social` (convertido ou mantido como `NUMERIC` / `REAL` para buscas e filtros numéricos).

3. **Estratégia de Índices**:
   - Os índices são criados **APÓS** a inserção em lote dos dados de ETL para evitar _overhead_ durante a ingestão massiva.

---

## 2. Esquema DDL (Data Definition Language)

### 2.1 Tabelas de Domínio (Lookups)

```sql
-- Tabela: PAISES
CREATE TABLE IF NOT EXISTS paises (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
);

-- Tabela: MUNICIPIOS
CREATE TABLE IF NOT EXISTS municipios (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
);

-- Tabela: QUALIFICACOES_SOCIOS
CREATE TABLE IF NOT EXISTS qualificacoes_socios (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
);

-- Tabela: NATUREZAS_JURIDICAS
CREATE TABLE IF NOT EXISTS naturezas_juridicas (
    codigo INTEGER PRIMARY KEY,
    descricao TEXT NOT NULL
);

-- Tabela: CNAES
CREATE TABLE IF NOT EXISTS cnaes (
    codigo TEXT PRIMARY KEY,
    descricao TEXT NOT NULL
);
```

---

### 2.2 Tabelas Principais (Entidades)

```sql
-- Tabela: EMPRESAS
CREATE TABLE IF NOT EXISTS empresas (
    cnpj_basico TEXT PRIMARY KEY,
    razao_social TEXT NOT NULL,
    natureza_juridica INTEGER NOT NULL,
    qualificacao_responsavel INTEGER NOT NULL,
    capital_social REAL NOT NULL DEFAULT 0.0,
    porte_empresa TEXT NOT NULL,
    ente_federativo_responsavel TEXT
);

-- Tabela: ESTABELECIMENTOS
CREATE TABLE IF NOT EXISTS estabelecimentos (
    cnpj_basico TEXT NOT NULL,
    cnpj_ordem TEXT NOT NULL,
    cnpj_dv TEXT NOT NULL,
    identificador_matriz_filial INTEGER NOT NULL,
    nome_fantasia TEXT,
    situacao_cadastral INTEGER NOT NULL,
    data_situacao_cadastral TEXT,
    motivo_situacao_cadastral INTEGER,
    nome_cidade_exterior TEXT,
    pais INTEGER,
    data_inicio_atividade TEXT,
    cnae_fiscal_principal TEXT NOT NULL,
    cnae_fiscal_secundaria TEXT,
    tipo_logradouro TEXT,
    logradouro TEXT,
    numero TEXT,
    complemento TEXT,
    bairro TEXT,
    cep TEXT,
    uf TEXT,
    municipio INTEGER,
    ddd_1 TEXT,
    telefone_1 TEXT,
    ddd_2 TEXT,
    telefone_2 TEXT,
    ddd_fax TEXT,
    fax TEXT,
    correio_eletronico TEXT,
    situacao_especial TEXT,
    data_situacao_especial TEXT,
    PRIMARY KEY (cnpj_basico, cnpj_ordem, cnpj_dv)
);

-- Tabela: DADOS_SIMPLES
CREATE TABLE IF NOT EXISTS dados_simples (
    cnpj_basico TEXT PRIMARY KEY,
    opcao_simples TEXT,
    data_opcao_simples TEXT,
    data_exclusao_simples TEXT,
    opcao_mei TEXT,
    data_opcao_mei TEXT,
    data_exclusao_mei TEXT
);

-- Tabela: SOCIOS
CREATE TABLE IF NOT EXISTS socios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cnpj_basico TEXT NOT NULL,
    identificador_socio INTEGER NOT NULL,
    nome_socio_razao_social TEXT NOT NULL,
    cnpj_cpf_socio TEXT,
    qualificacao_socio INTEGER NOT NULL,
    data_entrada_sociedade TEXT,
    pais INTEGER,
    representante_legal TEXT,
    nome_representante TEXT,
    qualificacao_representante_legal INTEGER,
    faixa_etaria INTEGER NOT NULL
);
```

---

## 3. Índices de Alta Performance (`INDEXES`)

Criados para suportar buscas rápidas por CNPJ completo, Razão Social/Nome Fantasia, Nome de Sócio, UF e Município.

```sql
-- Busca por CNPJ completo (CNPJ Básico + Ordem + DV) e filtro por CNPJ Básico em Estabelecimentos
CREATE INDEX IF NOT EXISTS idx_estab_cnpj_full ON estabelecimentos(cnpj_basico, cnpj_ordem, cnpj_dv);
CREATE INDEX IF NOT EXISTS idx_estab_uf_muni ON estabelecimentos(uf, municipio);
CREATE INDEX IF NOT EXISTS idx_estab_cnae ON estabelecimentos(cnae_fiscal_principal);

-- Busca por Razão Social (suporte a LIKE / prefixo)
CREATE INDEX IF NOT EXISTS idx_empresas_razao_social ON empresas(razao_social COLLATE NOCASE);

-- Busca por Sócios vinculados a empresas ou nome de sócio
CREATE INDEX IF NOT EXISTS idx_socios_cnpj_basico ON socios(cnpj_basico);
CREATE INDEX IF NOT EXISTS idx_socios_nome ON socios(nome_socio_razao_social COLLATE NOCASE);
```

---

## 4. Mapeamento de Tipos Rust <-> SQLite (`rusqlite`)

| Tabela Rust / Campo           | Tipo Struct Rust | Tipo SQLite | Observações / Conversão                     |
| :---------------------------- | :--------------- | :---------- | :------------------------------------------ |
| **Empresa**                   |                  |             |                                             |
| `cnpj_basico`                 | `String`         | `TEXT`      | Primary Key                                 |
| `razao_social`                | `String`         | `TEXT`      | Indexed `NOCASE`                            |
| `natureza_juridica`           | `u16`            | `INTEGER`   | FK -> `naturezas_juridicas(codigo)`         |
| `qualificacao_responsavel`    | `u8`             | `INTEGER`   |                                             |
| `capital_social`              | `String`         | `REAL`      | Converter `String` ("1000,00" -> `1000.00`) |
| `porte_empresa`               | `String`         | `TEXT`      | "00", "01", "03", "05"                      |
| `ente_federativo_responsavel` | `Option<String>` | `TEXT`      | Nullable                                    |
| **Estabelecimento**           |                  |             |                                             |
| `cnpj_basico`                 | `String`         | `TEXT`      | Composite PK                                |
| `cnpj_ordem`                  | `String`         | `TEXT`      | Composite PK                                |
| `cnpj_dv`                     | `String`         | `TEXT`      | Composite PK                                |
| `identificador_matriz_filial` | `u8`             | `INTEGER`   | 1: Matriz, 2: Filial                        |
| `situacao_cadastral`          | `u8`             | `INTEGER`   | 01, 02, 03, 04, 08                          |
| `pais`                        | `Option<u16>`    | `INTEGER`   | Nullable                                    |
| `municipio`                   | `Option<u16>`    | `INTEGER`   | Nullable                                    |
| **Sócio**                     |                  |             |                                             |
| `cnpj_basico`                 | `String`         | `TEXT`      | Indexado                                    |
| `identificador_socio`         | `u8`             | `INTEGER`   | 1: PJ, 2: PF, 3: Estrangeiro                |
| `qualificacao_socio`          | `u8`             | `INTEGER`   | FK -> `qualificacoes_socios(codigo)`        |
| `faixa_etaria`                | `u8`             | `INTEGER`   | 0 a 9                                       |

---

## 5. View Otimizada para Visualização Completa da Empresa

Para facilitar consultas na TUI e exportações integradas sem precisar refazer múltiplos `JOIN`s manualmente na camada de aplicação:

```sql
CREATE VIEW IF NOT EXISTS vw_empresa_completa AS
SELECT
    e.cnpj_basico || est.cnpj_ordem || est.cnpj_dv AS cnpj_completo,
    e.razao_social,
    est.nome_fantasia,
    nj.descricao AS natureza_juridica_desc,
    e.capital_social,
    e.porte_empresa,
    est.situacao_cadastral,
    est.data_inicio_atividade,
    est.cnae_fiscal_principal,
    cnae.descricao AS cnae_principal_desc,
    est.tipo_logradouro || ' ' || est.logradouro || ', ' || est.numero AS endereco_completo,
    est.bairro,
    est.cep,
    est.uf,
    m.descricao AS municipio_desc,
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
