---
description: Fornece uma dica direcionada para a tarefa atual sem revelar a solução.
---

# Dica de Tarefa Rust

**Uso:** `/dica-tarefa <nome-da-pasta>`  
Exemplo: `/dica-tarefa tarefa2`

---

## Etapa 1 — Identificar a Tarefa

A tarefa é: **`$task`**

Se `$task` não foi fornecido, use o arquivo `.rs` aberto no editor para determinar a pasta.

---

## Etapa 2 — Ler o Contexto

Leia em paralelo:
- `tasks/$task/README.md` — para entender o que é exigido
- `tasks/$task/src/main.rs` — para entender onde o aluno está parado

---

## Etapa 3 — Identificar o Ponto de Dificuldade

Analise o código atual e determine:
- O que já está correto
- Qual é o próximo obstáculo concreto (erro de compilação, lógica errada, conceito faltando)

---

## Etapa 4 — Dar a Dica

Forneça **uma única dica** direcionada ao ponto de dificuldade identificado.

Regras:
- **Nunca mostre a solução completa** — oriente o raciocínio, não escreva o código final.
- A dica deve ser adequada ao nível **Iniciante**.
- Se o erro for de compilação, explique o que o compilador está dizendo em linguagem simples.
- Se for lógica, faça uma pergunta que leve o aluno a perceber o problema por conta própria.
- Mostre no máximo um trecho de código ilustrativo pequeno (1–3 linhas), se necessário.
- Termine com uma pergunta ou desafio para o aluno tentar sozinho.
