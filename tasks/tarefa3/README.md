# Tarefa 3 - Verificador de Numero Primo

**Nivel:** Iniciante  
**Topicos:** `funcoes`, `for`, `i32`, `bool`, `raiz quadrada`

---

## Descricao

Implemente uma funcao em Rust que receba um numero inteiro e determine se ele e primo. O programa deve chamar essa funcao na `main` e imprimir o resultado booleano.

---

## Requisitos

### Funcao `eh_primo`

- **Assinatura:** `fn eh_primo(numero: i32) -> bool`
- Se `numero` for menor ou igual a `1`, retorne `false`.
- Crie uma variavel `limite` com a raiz quadrada de `numero`, arredondada para cima.
- Use um loop `for` para testar divisores de `2` ate `limite` (inclusive).
- Se encontrar um divisor exato, retorne `false`.
- Se nenhum divisor exato for encontrado, retorne `true`.

---

## Entrada

Nao ha entrada do usuario. Na funcao `main`, declare um valor fixo e use-o para chamar `eh_primo`.

```rust
fn main() {
    let numero: i32 = 17;
    let resultado: bool = eh_primo(numero);
    println!("{}", resultado);
}
```

---

## Saida Esperada

Para `numero = 17`:

```
true
```

---

## Restricoes

- A verificacao de divisores deve ser feita com loop `for`.
- O limite de iteracao deve usar a raiz quadrada arredondada para cima.
- Nao utilize bibliotecas externas (`crates`).
