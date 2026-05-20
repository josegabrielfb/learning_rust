<div align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://github.com/SEU_USUARIO/learning_rust/actions/workflows/ci.yml/badge.svg" alt="CI"/>
</div>

# Aprendizado de Rust

Repositório pessoal de estudos da linguagem Rust, organizado progressivamente do nível **Iniciante** ao **Avançado**.

---

## Estrutura do Projeto

```
.
├── Cargo.toml      # Workspace — inclui todas as tasks/tarefa* automaticamente
├── .github/        # Configurações do Copilot e CI (veja seções abaixo)
├── tasks/          # Exercícios avaliados com nota
│   ├── tarefaN/    # Cada tarefa tem README, src/main.rs e Cargo.toml
│   └── progress.md # Histórico de notas e tentativas
│
├── Iniciante/      # Exercícios de curso (não avaliados)
│   ├── aulaX/      # Exercícios das aulas
│   └── bee_XXXX/   # Problemas do Beecrowd
│
└── notes/          # Anotações e referências de padrões Rust
```

---

## Sistema de Tarefas

Cada tarefa em `tasks/tarefaN/` é um exercício avaliado em cinco dimensões:

| Dimensão | Peso |
|----------|------|
| D1 — Corretude | 20 pts |
| D2 — Conformidade com o enunciado | 20 pts |
| D3 — Qualidade do código | 20 pts |
| D4 — Idiomaticidade Rust | 20 pts |
| D5 — Boas práticas | 20 pts |

**Conceitos:** S (90–100) · A (75–89) · B (60–74) · C (45–59) · D (0–44)

O progresso de todas as tarefas é registrado automaticamente em [`tasks/progress.md`](tasks/progress.md).

---

## Criando Tarefas a partir de um Enunciado

Ao receber um enunciado bruto (descrição informal, exercício de curso, problema do Beecrowd, etc.), o Copilot transforma esse texto em um enunciado profissional e estruturado antes de criar a tarefa.

**Como usar:**
1. Digite `/criar-tarefa` no chat do Copilot
2. Cole ou descreva o enunciado bruto
3. O Copilot irá:
   - Determinar o próximo número de tarefa disponível
   - Criar a pasta com `cargo new tasks/tarefaN --vcs none`
   - Formatar o enunciado e gerar o `README.md` seguindo o padrão do projeto

O README gerado sempre conterá: nível, tópicos, descrição, requisitos com assinaturas de funções, entrada, saída esperada e restrições técnicas.

---

## CI — Integração Contínua

Cada push na branch `main` dispara automaticamente o workflow `.github/workflows/ci.yml`, que executa em ordem:

| Etapa | Comando |
|-------|---------|
| Formatação | `cargo fmt --all -- --check` |
| Linter | `cargo clippy --workspace -- -D warnings` |
| Build | `cargo build --workspace` |
| Testes | `cargo test --workspace` |

Se qualquer etapa falhar, o commit recebe ❌ no GitHub. Todas passando → ✅.

---

## Cargo Workspace

O `Cargo.toml` na raiz define um workspace com glob pattern:

```toml
members = ["tasks/tarefa*"]
```

Qualquer nova pasta `tasks/tarefaN/` criada com `cargo new` é incluída automaticamente — sem necessidade de editar o arquivo.

---

## Configuração do Copilot (`.github/`)

A pasta `.github/` contém toda a configuração do GitHub Copilot para este workspace:

```
.github/
├── copilot-instructions.md   # Contexto global — carregado em toda conversa
├── workflows/
│   └── ci.yml                # Pipeline de CI (fmt, clippy, build, test)
├── instructions/
│   ├── exercise-problem-statement.instructions.md  # Padrão de formatação dos READMEs
│   └── rust-exercise-review.instructions.md        # Regras de revisão de código .rs
└── prompt/
    ├── criar-tarefa.prompt.md      # /criar-tarefa
    ├── corrigir-tarefa.prompt.md   # /corrigir-tarefa
    ├── dica-tarefa.prompt.md       # /dica-tarefa
    └── explicar-conceito.prompt.md # /explicar-conceito
```

- **`copilot-instructions.md`** — define idioma, nível atual e regras gerais para todas as conversas
- **`instructions/`** — regras aplicadas automaticamente quando arquivos específicos estão em contexto (ex: ao editar um `.rs` ou criar um `README.md`)
- **`prompt/`** — comandos invocáveis pelo usuário via `/nome-do-prompt`

---

## Comandos do Copilot

| Comando | O que faz |
|---------|-----------|
| `/criar-tarefa` | Cria uma nova pasta de tarefa a partir de um enunciado bruto |
| `/corrigir-tarefa <tarefaN>` | Avalia o código, atribui nota e registra no histórico |
| `/dica-tarefa <tarefaN>` | Dá uma dica direcionada sem revelar a solução |
| `/explicar-conceito <conceito>` | Explica um conceito Rust no nível atual |

---

## Progresso Atual

**Nível:** Iniciante  
**Tarefas concluídas:** 1  
**Média:** 99.0/100

Veja o histórico completo em [`tasks/progress.md`](tasks/progress.md).
