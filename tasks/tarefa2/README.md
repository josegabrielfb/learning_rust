# Tarefa 2 — Maior Número em um Vetor

**Nível:** Iniciante  
**Tópicos:** `Vec`, `for`, funções, parâmetros, retorno de valores

---

## Descrição

Implemente uma função em Rust que percorra um vetor de números inteiros e retorne o maior valor encontrado. Em seguida, chame essa função na `main` e exiba o resultado.

---

## Requisitos

### Função `maior_numero`

- **Assinatura:** `fn maior_numero(numeros: Vec<i32>) -> i32`
- Inicialize uma variável `maior` com o primeiro elemento do vetor.
- Use um loop `for` para percorrer os elementos e atualizar `maior` sempre que encontrar um valor superior.
- Retorne o valor de `maior` ao final.

---

## Entrada

Não há entrada do usuário. Na função `main`, declare um vetor `numeros` com alguns valores inteiros de exemplo e passe-o para `maior_numero`.

```rust
fn main() {
    let numeros = vec![/* valores de exemplo */];
    let resultado = maior_numero(numeros);
    println!("{}", resultado);
}
```

---

## Saída Esperada

A saída deve ser o maior número presente no vetor. Para o vetor `[3, 7, 1, 9, 4]`, por exemplo:

```
9
```

---

## Restrições

- A função `maior_numero` **deve** utilizar um loop `for` para percorrer o vetor.
- Não utilize funções prontas como `.iter().max()`.
- Não utilize bibliotecas externas (`crates`).
