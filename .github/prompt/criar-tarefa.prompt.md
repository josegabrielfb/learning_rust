---
description: Cria uma nova pasta de tarefa Rust com Cargo e README formatado a partir de um enunciado bruto.
---

# Criar Nova Tarefa Rust

**Uso:** `/criar-tarefa`  
Em seguida, forneça o enunciado bruto da tarefa quando solicitado.

Siga cada etapa em ordem. Não pule etapas.

---

## Etapa 1 — Determinar o Número da Tarefa

Liste o conteúdo da pasta `tasks/` e identifique todas as subpastas com o padrão `tarefaN` (onde N é um inteiro).

Determine o próximo número disponível: `N = maior número encontrado + 1`.

Se não existir nenhuma pasta `tarefaN`, comece com `N = 1`.

Anuncie ao usuário: `Criando tarefa${N}...`

---

## Etapa 2 — Criar o Projeto Cargo

No terminal PowerShell, a partir da raiz do workspace, execute:

```powershell
Set-Location "tasks"
cargo new tarefa${N} --vcs none
Set-Location ..
```

> Use `--vcs none` para não inicializar um repositório Git separado.

---

## Etapa 3 — Formatar e Criar o README.md

Com base no enunciado bruto fornecido pelo usuário, crie o arquivo `tasks/tarefa${N}/README.md` seguindo **exatamente** o padrão definido na instruction `exercise-problem-statement.instructions.md`.

Regras de formatação (resumo):
- Título: `# Tarefa ${N} — <Título>`
- Campos obrigatórios: **Nível**, **Tópicos**, **Descrição**, **Requisitos**, **Entrada**, **Saída Esperada**, **Restrições**
- Não revele a implementação — descreva *o quê* fazer, não *como* fazer
- Assinaturas de funções com tipos Rust corretos
- Saída esperada deve ser determinística e completa

---

## Etapa 4 — Registrar no settings.json

Abra `.vscode/settings.json` e adicione a nova pasta na chave `"material-icon-theme.folders.associations"`:

```json
"tarefa${N}": "rust",
```

> Insira a linha em ordem numérica dentro do bloco `// ── tasks/`.  
> Nunca remova ou altere entradas de outras tarefas.

---

## Regras Gerais

- Nunca modifique o `src/main.rs` gerado pelo Cargo — deixe o conteúdo padrão (`fn main() { println!("Hello, world!"); }`).
- Se `cargo new` falhar (ex: pasta já existe), informe o usuário e interrompa.
- O README criado deve ser suficiente para resolver o exercício sem consultar o enunciado original.
