---
description: Explica um conceito de Rust no nível atual do aluno com exemplos práticos.
---

# Explicar Conceito Rust

**Uso:** `/explicar-conceito <conceito>`  
Exemplo: `/explicar-conceito ownership`

---

## Etapa 1 — Identificar o Conceito

O conceito a explicar é: **`$conceito`**

Se `$conceito` não foi fornecido, pergunte ao usuário qual conceito deseja entender.

---

## Etapa 2 — Calibrar ao Nível Atual

O aluno está no nível **Iniciante**. Isso significa:
- Conceitos conhecidos: variáveis, tipos primitivos, funções, loops (`for`, `while`), `println!`, `if/else`
- Conceitos ainda não cobertos: traits avançadas, generics, async/await, macros, unsafe

Adapte a explicação para não exigir conhecimento além do nível Iniciante.  
Se o conceito for avançado demais, explique uma versão simplificada e avise que a explicação completa virá em níveis futuros.

---

## Etapa 3 — Explicar

Estruture a explicação em três partes:

### O que é
Explique o conceito em 2–4 frases simples, sem jargão desnecessário.

### Por que existe
Explique qual problema o conceito resolve em Rust — contextualize com algo concreto.

### Como usar
Mostre um exemplo de código mínimo e funcional (máximo 15 linhas).  
O exemplo deve ser auto-contido e compilável.

---

## Regras

- Use português em toda a resposta.
- Não liste todos os casos de uso possíveis — foque no que é útil para o nível Iniciante.
- Se o conceito se relacionar com uma tarefa já feita pelo aluno (verifique `tasks/progress.md`), mencione essa conexão.
- Prefira exemplos com números, strings simples ou problemas do dia a dia — não crie exemplos abstratos.
