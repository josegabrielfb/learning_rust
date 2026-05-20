---
applyTo: Iniciante/**/*.rs,tasks/**/*.rs
---

# Instrução: Revisão de Exercício Rust

Quando o usuário pedir para corrigir, revisar ou dar feedback sobre um exercício Rust, siga este processo:

---

## Etapa 0 — Verificar Padrões Aplicáveis

Antes de qualquer análise, leia `notes/rust-senior-patterns.md` e determine o número da tarefa atual (pelo nome da pasta, ex: `tarefa23` → tarefa 23).

Com base no número da tarefa e na tabela de padrões do arquivo de notas:
- Se tarefa < 20: **não penalize** padrões P1–P10. Mencione como dica se relevante.
- Se tarefa ≥ 20: **aplique e penalize** os padrões marcados como cobrados para aquele número.

Lista rápida de quando cada padrão é exigido (consulte `notes/rust-senior-patterns.md` para detalhes):
- Tarefa 20+: P1 (nomenclatura), P2 (documentação), P3 (design de funções), P7 (fmt/clippy), P9 (constantes)
- Tarefa 25+: P5 (iteradores), P6 (ownership)
- Tarefa 30+: P4 (tratamento de erros)
- Tarefa 35+: P8 (testes)
- Tarefa 40+: P10 (estrutura de módulos)

---

## Processo de Revisão

### 1. Ler o Enunciado

Antes de analisar o código, leia o `README.md` na mesma pasta do arquivo `.rs` para entender:
- O que o exercício pede
- Quais funções/structs são obrigatórias
- A saída esperada
- As restrições técnicas (ex: obrigatoriedade de usar `for` ou `while`)

### 2. Analisar o Código

Verifique os seguintes pontos em ordem:

#### Corretude
- [ ] O programa compila sem erros?
- [ ] A saída do programa corresponde exatamente à saída esperada no README?
- [ ] Todos os casos de borda descritos estão cobertos?

#### Conformidade com o Enunciado
- [ ] As assinaturas de função correspondem ao especificado?
- [ ] As restrições foram respeitadas (ex: uso de `for` onde exigido, `while` onde exigido)?
- [ ] Nenhuma biblioteca externa (`crate`) foi usada se proibido?

#### Qualidade do Código Rust
- [ ] Os tipos estão corretos e bem escolhidos?
- [ ] Ownership e borrowing estão sendo usados corretamente?
- [ ] O código é idiomático para o nível do exercício?

### 3. Estrutura do Feedback

Organize o feedback assim:

```
## Resultado: ✅ Correto | ⚠️ Parcialmente Correto | ❌ Incorreto

### O que está certo
<Liste o que foi implementado corretamente.>

### Problemas Encontrados
<Para cada problema, indique:>
- **Problema:** descrição clara do erro ou desvio do enunciado
- **Localização:** linha ou trecho de código afetado
- **Correção:** como corrigir (mostre código quando necessário)

### Sugestões (opcional)
<Somente se o código funciona mas pode ser melhorado dentro do escopo do nível do exercício.>
```

---

## Regras do Revisor

1. **Não refatore além do pedido.** Se o código está correto, não reescreva por preferência estética.
2. **Respeite o nível.** Não sugira recursos avançados em exercícios iniciantes (ex: não sugira iteradores complexos em um exercício de `for`).
3. **Seja específico.** Aponte a linha exata do problema, não o arquivo todo.
4. **Mostre a correção.** Para cada erro, inclua o trecho corrigido em Rust.
5. **Não revele a solução completa** se o exercício estiver apenas parcialmente resolvido — oriente o usuário para chegar lá.
6. **Compile mentalmente.** Identifique erros de compilação (`borrow checker`, tipos incompatíveis, etc.) além de erros lógicos.

---

## Checklist Rápido para Exercícios Iniciantes

- Funções declaradas com `fn` e assinatura correta?
- `main` chama as funções corretamente?
- `println!` formata a saída corretamente (sem espaços extras, sem texto extra)?
- Variáveis mutáveis (`mut`) declaradas onde necessário?
- Loop `for` usa `1..=n` para incluir `n`?
- Loop `while` decrementa o contador corretamente?
