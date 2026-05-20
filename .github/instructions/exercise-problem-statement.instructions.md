---
applyTo: tasks/**/README.md,Iniciante/**/README.md
---

# Instrução: Criar Enunciado Profissional de Exercício

Quando o usuário fornecer um enunciado bruto (descrição informal, passo a passo de curso, etc.), transforme-o em um enunciado profissional seguindo o padrão abaixo. O objetivo é que o enunciado seja claro, conciso e suficiente para resolver o problema sem revelar a solução.

---

## Estrutura Obrigatória do README.md

```markdown
# <Título do Exercício>

**Nível:** <Iniciante | Intermediário | Avançado>
**Tópicos:** `<tópico1>`, `<tópico2>`, ...

---

## Descrição

<Parágrafo descrevendo o problema de forma objetiva. Não liste passos; descreva o que o programa deve fazer.>

---

## Requisitos

<Descreva as funções, structs ou comportamentos exigidos. Use subseções quando houver múltiplos itens.>

### Função `nome_da_funcao`

- **Assinatura:** `fn nome_da_funcao(param: Tipo) -> RetornoOpcional`
- <Descrição do comportamento esperado.>

---

## Entrada

<Descreva de onde vêm os dados: stdin, argumentos fixos em `main`, sem entrada, etc.>

---

## Saída Esperada

\```
<Exemplo exato da saída do programa para a entrada descrita.>
\```

---

## Restrições

- <Restrições técnicas obrigatórias, ex: "deve usar loop `for`", "não use `.clone()`", etc.>
```

---

## Regras de Formatação

1. **Não revele a implementação.** O enunciado descreve *o quê* fazer, não *como* fazer.
2. **Seja preciso com tipos.** Sempre indique os tipos Rust nas assinaturas (`i32`, `&str`, `String`, etc.).
3. **A saída deve ser reproduzível.** O exemplo de saída deve ser determinístico e completo.
4. **Nível de dificuldade:** avalie pelos tópicos envolvidos (tipos, ownership, traits, async, etc.).
5. **Tópicos:** liste somente os conceitos Rust diretamente exercitados (ex: `for`, `while`, `match`, `Vec`, `struct`).
6. **Entrada:** se não houver entrada do usuário, mostre o trecho de `main` que chama as funções.
7. **Restrições:** inclua somente restrições que diferenciam esta solução de outra equivalente (ex: exigir `for` em vez de `while`).

---

## Exemplo de Transformação

**Entrada bruta do usuário:**
> "Crie uma função chamada count que receba um inteiro e use um for loop para contar de 1 a 10 imprimindo cada número."

**Saída esperada:** ver `tasks/tarefa1/README.md` como referência de enunciado bem formatado.
