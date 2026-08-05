# Full Visual Specification - `lazy_cnpj`

This document contains ASCII mockups for the **3 MAIN TABS**, **HELP MODAL**, **EXTERNAL ACTIONS**, and the **360° COMPANY MODAL**.

---

## 1. Tab Navigation & Footer

The top of the terminal contains the **3 Main Tabs**:
- **`[ Search ]`** (Tab 1)
- **`[ Export ]`** (Tab 2)
- **`[ Update ]`** (Tab 3)

The **footer** is consistent across all tabs:
`[Tab/Shift+Tab] Switch Tabs  │  [?] Help  │  [q] Quit`

---

## 2. Mockups for All Tabs

### TAB 1: `[ Search ]`

```text
┌── lazy_cnpj v0.1.0 ─────────────────────────────────────────────────────────────────────────────┐
│ [ Search ]  Export  Update                                                                     │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 🔍 Term / CNPJ: [ PETROBRAS                                                           ] (Enter) │
│ ⚙️ Filters: Status: [ 🟢 Active Only ] │ State: [ RJ ] │ ZIP: [ All        ]  ([f] Edit Filters)│
├───────────────────────────────────────┬─────────────────────────────────────────────────────────┤
│ Results Found (12)                    │ Selected Company Details                                │
├───────────────────────────────────────┼─────────────────────────────────────────────────────────┤
│ > 33.000.167/0001-01  🟢               │ Company Name:   PETROLEO BRASILEIRO S A PETROBRAS       │
│   PETROLEO BRASILEIRO S A PETROBRAS   │ Trade Name:     CENTRO DE PESQUISAS                     │
│                                       │ Status:         [ ACTIVE ] since 03/11/2005             │
│   00.000.000/0001-91  🟢              │ Company Size:   LARGE ENTERPRISE                        │
│   BANCO DO BRASIL SA                  │ Capital Stock:  R$ 205.431.983.300,00                   │
│                                       │ Address:        AVENIDA HORACIO MACEDO, 950             │
│   60.701.190/0001-04  🔴              │ City/State:     RIO DE JANEIRO - RJ (ZIP: 21.941-598)   │
│   ITAU UNIBANCO S.A.                  │ Main CNAE:      72.10-0-00 (Research & Development)     │
│                                       │ Partners (QSA): MAGDA MARIA DE REGINATO (President)     │
│                                       ├─────────────────────────────────────────────────────────┤
│                                       │ 🔗 Quick Actions:                                       │
│                                       │   [c] Copy Full Card  │  [g] Google  │  [m] Maps        │
├───────────────────────────────────────┴─────────────────────────────────────────────────────────┤
│ [Tab] Switch Tabs  │  [?] Help  │  [q] Quit                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### TAB 2: `[ Export ]`

```text
┌── lazy_cnpj v0.1.0 ─────────────────────────────────────────────────────────────────────────────┐
│ Search  [ Export ]  Update                                                                     │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ CSV Export Configuration (Source: Local SQLite Database)                                        │
├───────────────────────────────────────┬─────────────────────────────────────────────────────────┤
│ Export Filters:                       │ Fields to Include in Output File:                       │
│ State:    [ RJ, SP                 ]  │ [X] Full CNPJ            [X] Phone / Email              │
│ Status:   [ 🟢 Active Only         ]  │ [X] Company Name         [X] Main CNAE                  │
│ ZIP:      [ All                    ]  │ [X] Trade Name           [ ] Secondary CNAEs            │
│ CNAE:     [ All                    ]  │ [X] Registration Status  [X] Board of Partners (QSA)    │
│                                       │ [X] Full Address         [ ] Capital Stock              │
│                                       ├─────────────────────────────────────────────────────────┤
│ Limit:    [ 10000 records          ]  │ Output File Path:                                       │
│                                       │ [ ./cnpjs_exported.csv                         ]        │
├───────────────────────────────────────┴─────────────────────────────────────────────────────────┤
│ [Tab] Switch Tabs  │  [?] Help  │  [q] Quit                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### TAB 3: `[ Update ]` (Adaptive Concurrency ETL Engine)

```text
┌── lazy_cnpj v0.1.0 ─────────────────────────────────────────────────────────────────────────────┐
│ Search  Export  [ Update ]                                                                      │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 💻 Hardware Resource Sensor & Adaptive Concurrency:                                             │
│ Available RAM: 8.4 GB (Optimal) │ Free Disk Space: 55.2 GB │ Mode: 🚀 HIGH PERFORMANCE PROFILE   │
│ Auto Configuration: 3 Concurrent Downloads │ 4 MPSC CSV Parsing Workers -> 1 DB Writer           │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Current Status: PROCESSING RECEITA FEDERAL PUBLIC DATA                                          │
│                                                                                                 │
│ Concurrent Downloads:                                                                           │
│ [████████████████████████████████████████] 100% Download: Estabelecimentos1.zip (240 MB)        │
│ [████████████████████████████████────────]  80% Download: Estabelecimentos2.zip (190/240 MB)    │
│ [████████████████────────────]  40% Download: Estabelecimentos3.zip (96/240 MB)                 │
│                                                                                                 │
│ MPSC Pipeline CSV Parsing -> SQLite Batch Insert:                                               │
│ [████████████████████████████────────────]  70% Inserting Estabelecimentos1 (Batch WAL Mode)    │
│                                                                                                 │
│ Total Records: 24.100.000 / ~45.000.000 | Speed: 145,000 rec/s | Free Disk Space: +480 MB       │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [Tab] Switch Tabs  │  [?] Help  │  [q] Quit                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### HELP OVERLAY MODAL (Triggered by `?` in any tab)

```text
┌── Help & Keyboard Shortcuts ────────────────────────────────────────┐
│                                                                     │
│ General Navigation:                                                 │
│   Tab / Shift+Tab  - Switch between main tabs (Search, Export, Update)│
│   1, 2, 3          - Jump directly to tab                           │
│   ?                - Toggle this help window                        │
│   q / Esc          - Quit application                               │
│                                                                     │
│ Update Tab (ETL Engine):                                            │
│   u                - Start / Resume Update                          │
│   p                - Pause / Resume Downloads                       │
│                                                                     │
│                                         [ Press Esc to Close ]     │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 360° COMPANY DETAILS MODAL (`tui-overlay` Floating Widget)

```text
┌── 🏢 Company 360° Details ───────────────────────────────────────────────────────────────── [ESC/q: Close] ──┐
│ [1] Main Registration  │  [2] QSA (Partners: 3)  │  [3] Branches (14)  │  [4] Secondary CNAEs (8)           │
├──────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 📌 COMPANY NAME: PETROLEO BRASILEIRO S A PETROBRAS                                                           │
│ 🏷️ TRADE NAME:   CENTRO DE PESQUISAS E DESENVOLVIMENTO LEOPOLDO AMERICO MIQUEZ DE MELLO                     │
├───────────────────────────────────────┬──────────────────────────────────────────────────────────────────────┤
│ 📋 Registration Data                  │ 📍 Location & Contact                                                │
│ • CNPJ:      33.000.167/0001-01 (Head) │ • Address:   AVENIDA HORACIO MACEDO, 950                              │
│ • Status:    🟢 ACTIVE since 03/11/2005│ • District:  CIDADE UNIVERSITARIA                                    │
│ • Reason:    NO REASON                │ • City/State:RIO DE JANEIRO - RJ                                     │
│ • Size:      DEMAIS (Large Enterprise)│ • ZIP:       21.941-598                                              │
│ • Cap. Stock:R$ 205.431.983.300,00    │ • Email:     contato@petrobras.com.br                                │
│ • Legal Nat.:203-8 - Soc. Anônima     │ • Phone:     (21) 3866-4000                                          │
├───────────────────────────────────────┴──────────────────────────────────────────────────────────────────────┤
│ 💼 Main Economic Activity (CNAE)                                                                             │
│ • Code: 72.10-0-00 - Experimental research and development in physical and natural sciences                  │
├──────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 👥 Board of Partners & Officers (QSA) - Summary                                                              │
│ > MAGDA MARIA DE REGINATO CHAMBRIARD (President) ── since 24/05/2024                                         │
│ > SERGIO CAETANO LEITE (Financial Director) ──────── since 15/06/2023                                         │
├──────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 🔗 Quick Actions:                                                                                            │
│  [c] Copy CNPJ  │  [f] Copy JSON Card  │  [g] Search on Google  │  [m] Open on Maps  │  [e] Export Record     │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```
