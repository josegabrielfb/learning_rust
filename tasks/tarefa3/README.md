# Tarefa 3 — Verificação de Número Primo

**Nível:** Iniciante  
**Tópicos:** `for`, `if`, funções, retorno booleano, operador `%`, raiz quadrada

---

## Descrição

Implemente uma função em Rust que receba um número inteiro e determine se ele é primo. Em seguida, chame essa função na `main` com um valor de exemplo e exiba o resultado.

---

## Requisitos

### Função `eh_primo`

- **Assinatura:** `fn eh_primo(numero: i32) -> bool`
- Se `numero` for menor ou igual a `1`, retorne `false` imediatamente.
- Calcule o limite de verificação como a raiz quadrada de `numero`, arredondada para cima, e armazene em uma variável `limite`.
- Use um loop `for` para iterar de `2` até `limite` (inclusive). Se `numero` for divisível por qualquer valor nesse intervalo, retorne `false`.
- Se o loop terminar sem encontrar divisores, retorne `true`.

---

## Entrada

Não há entrada do usuário. Na função `main`, declare uma variável `numero` com um valor inteiro de exemplo e passe-a para `eh_primo`.

```rust
fn main() {
    let numero = /* valor de exemplo */;
    let resultado = eh_primo(numero);
    println!("{}", resultado);
}
```

---

## Saída Esperada

A saída deve ser `true` se o número for primo, ou `false` caso contrário. Para `numero = 7`:

```
true
```

Para `numero = 9`:

```
false
```

---

## Restrições

- A função `eh_primo` **deve** utilizar um loop `for` para verificar os divisores.
- O limite de iteração **deve** ser calculado a partir da raiz quadrada do número.
- Não utilize bibliotecas externas (`crates`).
