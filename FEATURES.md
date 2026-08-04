# Sugestões de Funcionalidades & Recursos Avançados (`FEATURES.md`)

Este documento consolida sugestões de funcionalidades avançadas, utilitários e melhorias de UX para o **`lazy_cnpj`**, focando no ecossistema Rust e na experiência do usuário em ambiente de terminal (TUI Ratatui).

---

## 1. Módulo de Busca & Filtros Avançados (`Search Tab`)

### 1.1 FTS5 (Full-Text Search no SQLite)

- **Busca por Texto Livre**: Implementar suporte a **SQLite FTS5** na Razão Social, Nome Fantasia e Nome do Sócio.
- **Benefício**: Permite buscas parciais por palavras-chave sem perder performance (ex: buscar "Tecnologia Silva" encontrando "SILVA TECNOLOGIA E SERVICOS LTDA").

### 1.2 Filtros Combinados / Multi-Critério

- **Filtros por Estado (UF) e Município**: Auto-complete com lista de UFs e municípios.
- **Filtro por Faixa de Capital Social**: Permitir filtrar empresas com `capital_social >= X` e `<= Y`.
- **Filtro por Situação Cadastral**: Filtrar apenas empresas **Ativas** (código `02`), ou Baixadas/Inaptas.
- **Filtro por Simples Nacional / MEI**: Checkboxes para incluir/excluir Optantes do Simples Nacional ou MEI.
- **Filtro por CNAE (Setor Econômico)**: Seleção por código de atividade econômica principal ou secundária.

---

## 2. Visualizador Detalhado da Empresa (`Company Detail Modal / Tab`)

- **Visão 360° da Empresa**: Ao pressionar `Enter` na lista de resultados, abrir uma tela detalhada dividida em seções:
  - **Dados Cadastrais**: CNPJ completo, Razão Social, Nome Fantasia, Situação, Data de Início, Porte.
  - **Quadro de Sócios e Administradores (QSA)**: Tabela de sócios, faixas etárias, qualificações e representantes legais.
  - **Endereço Completo & Contatos**: Logradouro, Bairro, CEP, Cidade, UF, Telefones e E-mail.
  - **Atividades Econômicas**: CNAE Principal e lista expandida de CNAEs Secundários com descrições amigáveis.
  - **Histórico & Situação Especial**: Motivo da situação cadastral, ente federativo e dados de opção Simples/MEI.

---

## 3. Integrações Externa & Ações Rápidas (Shortcuts)

- **Cópia Instantânea para Clipboard (`c` / `Ctrl+C`)**:
  - Copiar CNPJ formatado (`XX.XXX.XXX/XXXX-XX`) ou limpo.
  - Copiar cartão de dados da empresa em formato JSON / Texto simples.
- **Abertura no Navegador (`o` / `Open`)**:
  - Atalho para abrir a empresa diretamente no **Sintegra**, **Redesim**, **Google Maps** (endereço) ou consultas de cartão CNPJ da Receita.
- **Consulta de CEP Integrada**:
  - Atalho para validar ou enriquecer o endereço via APIs públicas (ex: ViaCEP / BrasilAPI) quando houver conexão com a internet.

---

## 4. Recursos de Exportação Personalizada (`Export Tab`)

- **Formatos Adicionais de Exportação**:
  - **CSV / TSV**: Com seletor de delimitador (vírgula ou ponto-e-vírgula para Excel em PT-BR).
  - **JSON / JSON Lines (JSONL)**: Para integração com pipelines de dados ou scripts em Python/Node.
  - **SQLite Dump / Parquet**: Para usuários de Data Science que desejam subsets filtrados da base.
- **Templates de Exportação Predefinidos**:
  - Preset **"Mailing / Prospecção"**: CNPJ, Razão Social, E-mail, Telefones, UF, Cidade, CNAE.
  - Preset **"Análise Cadastral"**: Dados cadastrais completos + QSA (Sócios).
  - Preset **"Geográfico"**: CNPJ, UF, Município, CEP, Bairro.

---

## 5. Gerenciamento de ETL, Atualizações & Diagnósticos (`Update Tab`)

- **Downloads Parciais / Seleção de Arquivos**:
  - Permitir ao usuário escolher atualizar apenas tabelas específicas (ex: apenas a tabela de `Empresas` ou apenas `Municípios`).
- **Verificação de Integridade Checksum (MD5/SHA256)**:
  - Validar os downloads dos arquivos ZIP da Receita Federal antes do processamento para evitar corrupção de dados.
- **Resumo Estatístico da Base (Dashboard)**:
  - Painel exibindo estatísticas gerais da base local:
    - Total de Empresas e Estabelecimentos cadastrados.
    - Distribuição por UF (Top 5 estados com mais empresas).
    - Porcentagem de empresas ativas vs. baixadas.
    - Data da última carga/atualização realizada.

---

## 6. Experiência de Uso (TUI & UX)

- **Suporte Completo a Mouse**:
  - Clique para selecionar abas, scroll no mouse para rolar tabelas e modais.
- **Temas Customizáveis (Dark / Light / High Contrast)**:
  - Palette de cores configurável no arquivo de configuração `lazy_cnpj.toml`.
- **Modo CLI / Headless (Sem TUI)**:
  - Permitir rodar o `lazy_cnpj` via linha de comando para automação de ETL sem abrir a interface de terminal:
    - Ex: `lazy_cnpj update --auto` ou `lazy_cnpj export --uf SP --cnae 6201500 --output sp_tech.csv`.

---

## 7. Mapeamento Direto com os Widgets Padrão do Ratatui

Todas as funcionalidades acima são **100% realizáveis utilizando os widgets nativos** (ou através da composição deles) fornecidos pelo Ratatui:

| Funcionalidade Planejada            | Widget Ratatui Nativo / Componente              | Como é Implementado                                                                             |
| :---------------------------------- | :---------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| **Navegação entre Seções**          | `Tabs`                                          | Alterna entre as telas de Pesquisa, Exportação e Update.                                        |
| **Listas de Resultados / Empresas** | `Table` + `TableState`                          | Exibe tabelas paginadas ou roláveis com seleção de linha ativa, ordenação e colunas ajustáveis. |
| **Filtros e Entradas de Texto**     | `Paragraph` + `Block` (ou crate `tui-textarea`) | Caixas de busca de CNPJ, Razão Social e selects de UF/Município.                                |
| **Popups e Modal 360°**             | `Clear` + `Block` + `Layout`                    | Renderiza a camada modal sobrepondo a tela principal (`Clear`) com abas/blocos internos.        |
| **Indicadores de Carga/ETL**        | `Gauge` / `LineGauge`                           | Barras de progresso dinâmicas de download e inserção no SQLite.                                 |
| **Dashboard e Métricas**            | `BarChart` / `Sparkline`                        | Gráficos de barras de empresas por UF ou histórico de progresso do ETL.                         |
| **Listas de Opções / Checkboxes**   | `List` + `ListState`                            | Listagem de colunas para exportação e seleção múltipla de arquivos.                             |
| **Barras de Rolagem**               | `Scrollbar` + `ScrollbarState`                  | Rolagem lateral e vertical em modais com textos longos ou tabelas com muitas colunas.           |
