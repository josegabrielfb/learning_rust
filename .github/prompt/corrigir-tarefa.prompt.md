---
description: Corrige uma tarefa Rust, executa o código, atribui uma nota estruturada e registra o progresso no arquivo tasks/progress.md.
---

# Correção de Tarefa Rust

**Uso:** `/corrigir-tarefa <nome-da-pasta>`  
Exemplo: `/corrigir-tarefa tarefa1`

Siga cada etapa em ordem. Não pule etapas.

---

## Etapa 1 — Identificar a Tarefa

A tarefa a corrigir é: **`$task`**

Se `$task` não foi fornecido, use o arquivo `.rs` atualmente aberto no editor para determinar a pasta.

A estrutura esperada é:
```
tasks/$task/
  README.md
  src/main.rs
  Cargo.toml
```

---

## Etapa 2 — Verificar Tentativas Anteriores e Aplicar Penalidade

Antes de qualquer avaliação, leia o arquivo `tasks/progress.md` (se existir) e procure a linha correspondente à tarefa `$task`.

Extraia o valor da coluna **Tentativas**. Se a tarefa não existir no arquivo, a tentativa atual é a **1ª**.

Calcule a **nota máxima permitida (cap)** e o **desconto acumulado** conforme a tabela:

| Tentativa | Nota Máxima | Desconto |
|-----------|-------------|----------|
| 1ª a 5ª   | 100/100     | —        |
| 6ª        | 85/100      | -15 pts  |
| 7ª        | 70/100      | -30 pts  |
| 8ª        | 55/100      | -45 pts  |
| 9ª+       | 40/100      | -60 pts  |

> **Regra:** As primeiras 5 tentativas não têm penalidade (cap = 100). A partir da 6ª tentativa, a nota bruta é limitada pelo cap. Se a nota bruta for maior que o cap, a nota final exibida é o cap.  
> Exemplo: nota bruta 92, 6ª tentativa → nota final = 85.

Anuncie ao usuário no início do relatório:
- Número da tentativa atual
- Cap aplicado (ou "sem penalidade" se for tentativa 1–5)

---

## Etapa 3 — Ler o Enunciado

Leia o `README.md` da pasta `tasks/$task/` e extraia:
- **Nível** (Iniciante / Intermediário / Avançado)
- **Tópicos** exigidos
- **Funções/structs** obrigatórias com assinaturas
- **Saída esperada** (bloco de código exato)
- **Restrições** técnicas (ex: "deve usar `for`", "não use `clone`")

---

## Etapa 4 — Verificar Padrões Sênior Aplicáveis

Leia o arquivo `notes/rust-senior-patterns.md` e determine o número da tarefa a partir do nome da pasta `$task` (ex: `tarefa23` → tarefa 23).

Com base no número, identifique quais padrões (P1–P10) já são **exigidos e penalizados**. Guarde essa lista para usar na avaliação D3 e D5.

- Se o número da tarefa < 20: padrões são **apenas dicas** — não penalizam a nota.
- Se ≥ 20: violações dos padrões aplicáveis são **penalizadas nas dimensões D3 e D5**.

Anuncie no relatório quais padrões estão sendo cobrados nesta tarefa (ou "nenhum padrão sênior cobrado ainda" se < 20).

---

## Etapa 5 — Ler o Código

Leia `tasks/$task/src/main.rs` completo. Identifique:
- Todas as funções implementadas
- Os tipos utilizados
- Como `main` chama as funções

---

## Etapa 5 — Compilar e Executar

Navegue até a pasta `tasks/$task/` e execute os comandos abaixo **separadamente** (não use `2>&1` em PowerShell — causa falso erro):

```powershell
Set-Location "tasks/$task"
cargo build *>&1
cargo run
```

> `*>&1` é o redirecionamento correto para PowerShell (redireciona todos os streams sem gerar `NativeCommandError`).  
> Execute `cargo build` e `cargo run` como chamadas separadas — nunca encadeados com `;` na mesma linha.

Capture e anote:
- **Erros de compilação** (se houver) — indica falha em D1
- **Warnings do compilador** — lista cada `warning:` encontrado na saída de `cargo build`; cada warning penaliza D5
- **Saída real** do programa (stdout de `cargo run`)

---

## Etapa 6 — Avaliar e Atribuir Nota

Avalie em cinco dimensões. Cada dimensão vale **20 pontos** (total: **100 pontos**).

### D1 — Corretude (20 pts)
| Critério | Pts |
|---|---|
| Compila sem erros | 5 |
| Saída real == saída esperada (exata, linha a linha) | 10 |
| Casos de borda cobertos (se houver) | 5 |

### D2 — Conformidade com o Enunciado (20 pts)
| Critério | Pts |
|---|---|
| Assinaturas de função corretas | 8 |
| Restrições técnicas respeitadas (ex: `for`/`while` onde exigido) | 8 |
| Sem dependências externas quando proibido | 4 |

### D3 — Qualidade do Código (20 pts)
| Critério | Pts |
|---|---|
| Tipos corretos e bem escolhidos | 6 |
| Ownership/borrowing sem erros ou workarounds desnecessários | 7 |
| Código legível (nomes significativos, sem código morto) | 7 |

### D4 — Idiomaticidade Rust (20 pts)
| Critério | Pts |
|---|---|
| Uso idiomático de loops (`1..=n`, `while cond`) | 8 |
| Sem padrões de outras linguagens forçados em Rust | 6 |
| `println!` e formatação de strings corretos | 6 |

### D5 — Boas Práticas (20 pts)
| Critério | Pts |
|---|---|
| Sem warnings do compilador (0 warnings) | 10 |
| Variáveis `mut` usadas apenas onde necessário | 5 |
| Sem código comentado ou debugging esquecido | 5 |

> **Penalidade automática por warnings:** cada `warning:` encontrado na saída de `cargo build` desconta **4 pts** de D5 (mínimo 0).  
> Exemplo: 2 warnings → D5 máximo = 10 pts (−8 pts de warnings, independente dos outros critérios).  
> Isso se aplica **mesmo que o código compile e rode corretamente.**

**Nota Bruta = D1 + D2 + D3 + D4 + D5**  
**Nota Final = min(Nota Bruta, Cap da Tentativa)**

Classifique pela **Nota Final**:
- 90–100 → **S** (Excepcional)
- 75–89 → **A** (Ótimo)
- 60–74 → **B** (Bom)
- 45–59 → **C** (Regular)
- 0–44 → **D** (Precisa melhorar)

---

## Etapa 7 — Exibir Relatório de Correção

Mostre ao usuário o seguinte relatório:

```
╔══════════════════════════════════════════════════╗
║           RELATÓRIO DE CORREÇÃO                  ║
╠══════════════════════════════════════════════════╣
║  Tarefa    : <nome>                              ║
║  Nível     : <Iniciante|Intermediário|Avançado>  ║
║  Tópicos   : <lista>                             ║
║  Tentativa : <N>ª  |  Cap: <XX>/100              ║
╠══════════════════════════════════════════════════╣
║  D1 Corretude            : XX/20                 ║
║  D2 Conformidade         : XX/20                 ║
║  D3 Qualidade de Código  : XX/20                 ║
║  D4 Idiomaticidade Rust  : XX/20                 ║
║  D5 Boas Práticas        : XX/20                 ║
╠══════════════════════════════════════════════════╣
║  Nota Bruta : XX/100                             ║
║  Penalidade : -XX pts (Xª tentativa)             ║
║  NOTA FINAL : XX/100  [ S | A | B | C | D ]      ║
╚══════════════════════════════════════════════════╝
```

> Se for tentativa 1–5, omita a linha "Penalidade" e mostre "Tentativa: Xª | Sem penalidade".

Em seguida, exiba:

### ✅ O que está correto
<liste os pontos positivos>

### ❌ Problemas Encontrados
Para cada problema:
- **Problema:** descrição objetiva
- **Localização:** `src/main.rs` linha X
- **Como corrigir:** trecho de código corrigido

### 💡 Dicas de Evolução
<Máximo 3 dicas contextualizadas ao nível do exercício. Não sugira recursos além do nível.>

---

## Etapa 8 — Registrar no Histórico de Progresso

Atualize (ou crie se não existir) o arquivo `tasks/progress.md` com uma linha na tabela de progresso.

### Formato do `tasks/progress.md`

Se o arquivo não existir, crie com este cabeçalho:

```markdown
# Progresso das Tarefas

> Atualizado automaticamente pela correção de tarefas.

## Resumo

| Tarefa | Nível | Tópicos | Nota Bruta | Nota Final | Conceito | Tentativas | Última Data | Status |
|--------|-------|---------|------------|------------|----------|------------|-------------|--------|
```

Adicione ou atualize a linha correspondente à tarefa `$task`:

```
| tarefa1 | Iniciante | `for`, `while` | 92/100 | 85/100 | A | 2 | 2026-05-19 | ✅ |
```

Regras:
- Se a tarefa **já existe** na tabela, **atualize** a linha incrementando `Tentativas` em 1.  
  **Não crie uma nova linha.** Substitua a linha existente inteira.
- Se a tarefa **não existe**, insira uma nova linha com `Tentativas = 1`.
- **Nota Bruta:** soma D1+D2+D3+D4+D5 antes da penalidade.
- **Nota Final:** nota após aplicar o cap da tentativa.
- **Status:** `✅` se Nota Final ≥ 60, `⚠️` se 45–59, `❌` se < 45.
- **Última Data:** data atual no formato `YYYY-MM-DD`.

### Seção de Evolução por Nível

Após a tabela, mantenha uma seção por nível com a média das **Notas Finais**:

```markdown
## Iniciante

Tarefas: X | Média Final: XX.X/100 | Melhor Final: XX/100 | Total de Tentativas: X

## Intermediário

Tarefas: X | Média Final: XX.X/100 | Melhor Final: XX/100 | Total de Tentativas: X

## Avançado

Tarefas: X | Média Final: XX.X/100 | Melhor Final: XX/100 | Total de Tentativas: X
```

Recalcule os valores após cada atualização.

---

## Regras Gerais

- Nunca modifique `src/main.rs` nem `README.md` durante a correção.
- Nunca revele a solução completa se o exercício estiver incompleto — dê pistas.
- Se `cargo build` falhar, D1 = 0 automaticamente e D4/D5 são avaliados pelo código-fonte.
- Se o README não tiver saída esperada, avalie somente D2, D3, D4, D5 (máx 80 pts, normalize para 100 antes de aplicar o cap).
- A penalidade **sempre** é aplicada com base no número de tentativas registradas em `progress.md`, independentemente da nota bruta obtida.
- O cap nunca pode ser removido ou ignorado, mesmo que o usuário peça.



