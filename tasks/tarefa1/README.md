# Tarefa 1 — Loops e Funções

**Nível:** Iniciante  
**Tópicos:** `for`, `while`, funções, parâmetros

---

## Descrição

Implemente duas funções em Rust para demonstrar o uso de loops e funções com parâmetros.

---

## Requisitos

### Função `count`

- **Assinatura:** `fn count(num: i32)`
- Use um loop `for` para iterar de `1` até `num` (inclusive).
- A cada iteração, imprima o número atual seguido de uma quebra de linha.

### Função `count_down`

- **Assinatura:** `fn count_down(num: i32)`
- Use um loop `while` para iterar de `num` até `1` (inclusive), em ordem decrescente.
- A cada iteração, imprima o número atual seguido de uma quebra de linha.

---

## Entrada

Não há entrada do usuário. As funções devem ser chamadas na função `main` com o valor `10`.

```rust
fn main() {
    count(10);
    count_down(10);
}
```

---

## Saída Esperada

```
1
2
3
4
5
6
7
8
9
10
10
9
8
7
6
5
4
3
2
1
```

---

## Restrições

- A função `count` **deve** utilizar um loop `for`.
- A função `count_down` **deve** utilizar um loop `while`.
- Não utilize bibliotecas externas (`crates`).
