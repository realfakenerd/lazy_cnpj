# lazy_cnpj ⚡

Uma TUI (Terminal User Interface) de alta performance desenvolvida em **Rust** para ingestão, consulta, visualização e exportação da base de dados pública de **CNPJs da Receita Federal do Brasil**.

---

## 🚀 Destaques

- **ETL Assíncrono Adaptativo**: Processamento de downloads `.zip` da Receita Federal em segundo plano, descompactação em fluxo (streaming), parsing paralelo em MPSC e inserção no SQLite em lote sem lock ou congelamento de UI.
- **Busca de Alta Performance (FTS5)**: Pesquisas Full-Text instantâneas por Razão Social, Nome Fantasia, CNPJ, UF, Município e CNAE.
- **Visão 360° da Empresa**: Modal com detalhes cadastrais completos, endereço, dados de contato e Quadro de Sócio e Administradores (QSA).
- **Exportação Flexível**: Geração de relatórios filtrados em formatos CSV e JSON.
- **Arquitetura Reativa**: Construído com o padrão **Component + Flux**, garantindo modularidade de interface e execução 100% não-bloqueante.

---

## 🛠️ Tecnologias Utilizadas

- **Linguagem**: [Rust](https://www.rust-lang.org/)
- **Interface TUI**: [Ratatui](https://ratatui.rs/) & [Crossterm](https://crates.io/crates/crossterm)
- **Runtime Async**: [Tokio](https://tokio.rs/)
- **Banco de Dados**: SQLite (via [Diesel ORM](https://diesel.rs/) & FTS5)
- **Monitoramento de Hardware**: [sysinfo](https://crates.io/crates/sysinfo)

---

## 📐 Arquitetura da Aplicação (Component + Flux)

A aplicação utiliza uma arquitetura reativa desacoplada combinando **Component Pattern** e **Flux Pattern**:

```text
┌────────────────────────────────────────────────────────┐
│               Eventos (Teclado / Ticks)                │
└───────────────────────────┬────────────────────────────┘
                            │ (dispara Action)
                            ▼
 ┌──────────────────────────────────────────────────────┐
 │       mpsc::UnboundedChannel<Action> (Flux)          │
 └──────────────────────────┬───────────────────────────┘
                            │ (recebe Action)
                            ▼
 ┌──────────────────────────────────────────────────────┐
 │               Main Reducer / Update Loop             │
 └──────────────────────────┬───────────────────────────┘
                            │ (atualiza estado e renderiza)
                            ▼
 ┌──────────────────────────────────────────────────────┐
 │               Componentes TUI (Ratatui)              │
 │  [ SearchTab ]    [ ExportTab ]    [ UpdateTab ]     │
 └──────────────────────────────────────────────────────┘
```

---

## 📋 Pré-requisitos & Instalação

### Pré-requisitos
- **Rust** (MSRV 1.75+)
- **SQLite3** instalado no sistema

### Como Executar

```bash
# Clonar o repositório
git clone https://github.com/usuario/lazy_cnpj.git
cd lazy_cnpj

# Compilar e executar em modo de desenvolvimento
cargo run

# Compilar versão otimizada de produção
cargo build --release
```

---

## ⌨️ Atalhos Principais

- `Tab` / `Shift+Tab`: Alternar entre as abas (`Search`, `Export`, `Update`).
- `1`, `2`, `3`: Atalho direto para as abas.
- `?`: Abrir modal de ajuda / atalhos.
- `q` / `Esc`: Fechar modal ou sair da aplicação.

---

## 📄 Licença

Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.
