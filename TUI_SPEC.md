# Especificação Visual Completa - `lazy_cnpj`

Este documento contém os mockups ASCII das **3 ABAS DO APLICATIVO**, do **MODAL DE AJUDA**, das **AÇÕES EXTERNAS** e do **ENGINE ADAPTATIVO DE CARGA (Update)**.

---

## 1. Navegação por Abas e Rodapé

O topo do terminal conterá a barra de **3 Abas Principais**:
- **`[ Pesquisa ]`** (Aba 1)
- **`[ Exportação ]`** (Aba 2)
- **`[ Update ]`** (Aba 3)

O **rodapé (footer)** é idêntico em todas as abas:
`[Tab/Shift+Tab] Alternar Abas  │  [?] Ajuda  │  [q] Sair`

---

## 2. Mockups de TODAS as Abas

### ABA 1: `[ Pesquisa ]`

```text
┌── lazy_cnpj v0.1.0 ─────────────────────────────────────────────────────────────────────────────┐
│ [ Pesquisa ]  Exportação  Update                                                               │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 🔍 Termo / CNPJ: [ PETROBRAS                                                          ] (Enter) │
│ ⚙️ Filtros: Situação: [ 🟢 Apenas Ativas ] │ UF: [ RJ ] │ CEP: [ Todos      ]  ([f] Editar Filtros)│
├───────────────────────────────────────┬─────────────────────────────────────────────────────────┤
│ Resultados Encontrados (12)           │ Detalhes da Empresa Selecionada                         │
├───────────────────────────────────────┼─────────────────────────────────────────────────────────┤
│ > 33.000.167/0001-01  🟢               │ Razão Social:   PETROLEO BRASILEIRO S A PETROBRAS       │
│   PETROLEO BRASILEIRO S A PETROBRAS   │ Nome Fantasia:  CENTRO DE PESQUISAS                     │
│                                       │ Situação:       [ ATIVA ] desde 03/11/2005              │
│   00.000.000/0001-91  🟢              │ Porte:          GRANDE PORTE                            │
│   BANCO DO BRASIL SA                  │ Capital Social: R$ 205.431.983.300,00                   │
│                                       │ Logradouro:     AVENIDA HORACIO MACEDO, 950             │
│   60.701.190/0001-04  🔴              │ Cidade/UF:      RIO DE JANEIRO - RJ (CEP: 21.941-598)   │
│   ITAU UNIBANCO S.A.                  │ CNAE Principal: 72.10-0-00 (Pesquisa e desenvolvimento) │
│                                       │ Quadro Sócios:  MAGDA MARIA DE REGINATO (Presidente)    │
│                                       ├─────────────────────────────────────────────────────────┤
│                                       │ 🔗 Ações Rápidas:                                       │
│                                       │   [c] Copiar Ficha Completa  │  [g] Google  │  [m] Maps│
├───────────────────────────────────────┴─────────────────────────────────────────────────────────┤
│ [Tab] Alternar Abas  │  [?] Ajuda  │  [q] Sair                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### ABA 2: `[ Exportação ]`

```text
┌── lazy_cnpj v0.1.0 ─────────────────────────────────────────────────────────────────────────────┐
│ Pesquisa  [ Exportação ]  Update                                                               │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Configuração de Exportação para CSV (Fonte: Banco SQLite Local)                                 │
├───────────────────────────────────────┬─────────────────────────────────────────────────────────┤
│ Filtros de Exportação:                │ Campos a Incluir no Arquivo CSV:                        │
│ UF:       [ RJ, SP                 ]  │ [X] CNPJ Completo        [X] Telefone / E-mail          │
│ Situação: [ 🟢 Apenas Ativas       ]  │ [X] Razão Social         [X] CNAE Principal             │
│ CEP:      [ Todos                  ]  │ [X] Nome Fantasia        [ ] CNAEs Secundários          │
│ CNAE:     [ Todos                  ]  │ [X] Situação Cadastral   [X] Quadro de Sócios (QSA)     │
│                                       │ [X] Endereço Completo    [ ] Capital Social             │
│                                       ├─────────────────────────────────────────────────────────┤
│ Limit:    [ 10000 registros        ]  │ Caminho do Arquivo de Saída:                            │
│                                       │ [ ./cnpjs_exportados.csv                       ]        │
├───────────────────────────────────────┴─────────────────────────────────────────────────────────┤
│ [Tab] Alternar Abas  │  [?] Ajuda  │  [q] Sair                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### ABA 3: `[ Update ]` (Motor de Carga Paralela Adaptativo)

```text
┌── lazy_cnpj v0.1.0 ─────────────────────────────────────────────────────────────────────────────┐
│ Pesquisa  Exportação  [ Update ]                                                                │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 💻 Diagnóstico de Recursos & Concorrência Adaptativa:                                           │
│ RAM Disponível: 8.4 GB (Excelente) │ Espaço Livre: 55.2 GB │ Modo Concorrente: 🚀 ALTO PERFIL   │
│ Configuração Automática: 3 Downloads Concorrentes │ 2 Workers de Unzip/Stream em Memória        │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Status Atual: PROCESSANDO DADOS DA RECEITA FEDERAL                                              │
│                                                                                                 │
│ Downloads Simultâneos:                                                                          │
│ [████████████████████████████████████████] 100% Download: Estabelecimentos1.zip (240 MB)        │
│ [████████████████████████████████────────]  80% Download: Estabelecimentos2.zip (190/240 MB)    │
│ [████████████████────────────]  40% Download: Estabelecimentos3.zip (96/240 MB)                 │
│                                                                                                 │
│ Pipeline Streaming CSV -> SQLite:                                                               │
│ [████████████████████████████────────────]  70% Inserindo Estabelecimentos1 (Batch WAL Mode)    │
│                                                                                                 │
│ Registros Totais: 24.100.000 / ~42.000.000 | Velocidade: 145.000 reg/s | Temp. Livre: +480 MB   │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [Tab] Alternar Abas  │  [?] Ajuda  │  [q] Sair                                                  │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### MODAL DE AJUDA (Acionado por `?` em qualquer aba)

```text
┌── Ajuda & Atalhos Teclado ──────────────────────────────────────────┐
│                                                                     │
│ Navegação Geral:                                                    │
│   Tab / Shift+Tab  - Alternar entre as 3 abas (Pesquisa, Export, Update)│
│   1, 2, 3          - Ir diretamente para a aba correspondente       │
│   ?                - Abrir / Fechar esta janela de ajuda            │
│   q / Esc          - Sair do aplicativo                             │
│                                                                     │
│ Aba Update (Motor Adaptativo):                                      │
│   u                - Iniciar/Retomar Atualização                    │
│   p                - Pausar / Retomar Downloads & Carga             │
│                                                                     │
│                                         [ Pressione Esc para Fechar ]│
└─────────────────────────────────────────────────────────────────────┘
```
