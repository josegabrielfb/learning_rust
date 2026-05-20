# Padrões de Programação Rust — Nível Sênior

> Este documento define os padrões que serão cobrados progressivamente nas tarefas.  
> **Cobrança inicia na Tarefa 20.** Antes disso, os padrões são apresentados como referência e dicas, mas não penalizam a nota.

---

## P1 — Nomenclatura (cobrado a partir da tarefa 20)

| Contexto | Convenção | Exemplo |
|---|---|---|
| Funções e métodos | `snake_case` | `calculate_total`, `get_user_name` |
| Variáveis e parâmetros | `snake_case` | `user_count`, `max_value` |
| Structs, Enums, Traits | `PascalCase` | `UserProfile`, `HttpError` |
| Constantes e statics | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES`, `DEFAULT_PORT` |
| Módulos e arquivos | `snake_case` | `user_service.rs`, `mod http_client` |
| Booleanos | prefixo `is_`, `has_`, `can_` | `is_valid`, `has_permission` |

**Regras adicionais:**
- Nomes devem descrever *o que é*, não *como funciona*. `user_count` sim, `n` não.
- Evite abreviações. `calculate` sim, `calc` não. `index` sim, `idx` não (exceto em loops muito locais).
- Funções devem ser verbos ou frases verbais: `get_user`, `validate_input`, `parse_config`.

---

## P2 — Documentação de Funções (cobrado a partir da tarefa 20)

Todo item público deve ter doc comment `///`. Formato padrão:

```rust
/// Breve descrição em uma linha do que a função faz.
///
/// Descrição mais detalhada se necessário. Explica comportamento,
/// casos especiais, ou o porquê de existir.
///
/// # Arguments
///
/// * `param_name` - O que este parâmetro representa.
///
/// # Returns
///
/// O que a função retorna e em que condições.
///
/// # Examples
///
/// ```
/// let result = my_function(42);
/// assert_eq!(result, 84);
/// ```
fn my_function(param_name: i32) -> i32 {
    param_name * 2
}
```

**Regras mínimas (o que será cobrado):**
- Funções públicas (`pub fn`) devem ter ao menos a linha de descrição `///`.
- Funções privadas com lógica não óbvia devem ter `//` explicando o porquê.
- Nunca documente o óbvio: `// incrementa i por 1` acima de `i += 1` é ruído.

---

## P3 — Design de Funções (cobrado a partir da tarefa 20)

- **Uma função, uma responsabilidade.** Se o nome precisar de "e" (`validate_and_save`), divida.
- **Tamanho:** idealmente ≤ 20 linhas. Acima de 40 linhas, considere dividir.
- **Parâmetros:** prefira referências `&T` a ownership `T` quando não precisar consumir o valor.
- **Retorno:** funções que podem falhar devem retornar `Result<T, E>`, não usar `panic!`.

```rust
// ❌ Ruim — ownership desnecessário
fn print_name(name: String) {
    println!("{}", name);
}

// ✅ Bom — empresta, não consome
fn print_name(name: &str) {
    println!("{}", name);
}
```

---

## P4 — Tratamento de Erros (cobrado a partir da tarefa 30)

- **Nunca use `.unwrap()` em código que não é teste ou prototipagem.**
- Use o operador `?` para propagar erros.
- Prefira `Result<T, E>` a `Option<T>` quando a causa do erro importa.

```rust
// ❌ Ruim — panic em produção
let value = some_option.unwrap();

// ✅ Bom — propaga o erro
let value = some_result?;

// ✅ Bom — trata explicitamente
let value = some_option.unwrap_or(0);
let value = some_option.unwrap_or_else(|| calculate_default());
```

---

## P5 — Iteradores e Loops (cobrado a partir da tarefa 25)

Prefira iteradores a loops manuais quando a operação é uma transformação de coleção:

```rust
// ❌ Loop manual para transformação
let mut doubled = Vec::new();
for x in &numbers {
    doubled.push(x * 2);
}

// ✅ Iterador idiomático
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// ✅ Ranges inclusivos e exclusivos
for i in 1..=10 { }   // 1 até 10 inclusivo
for i in 0..10  { }   // 0 até 9 (exclusivo no fim)
```

**Quando usar loop manual:** quando a lógica é complexa e o iterador prejudica a legibilidade.

---

## P6 — Ownership e Borrowing (cobrado a partir da tarefa 25)

```rust
// ❌ Clone desnecessário
fn process(data: String) -> String {
    let copy = data.clone(); // por que clonar se não usa o original depois?
    copy.to_uppercase()
}

// ✅ Sem clone
fn process(data: &str) -> String {
    data.to_uppercase()
}
```

- Prefira `&str` a `String` em parâmetros de função.
- Prefira `&[T]` a `Vec<T>` em parâmetros de função.
- Clone apenas quando semanticamente necessário (ex: manter duas versões independentes).

---

## P7 — Formatação e Ferramentas (cobrado a partir da tarefa 20)

**Sempre antes de entregar:**

```bash
cargo fmt        # formata o código automaticamente
cargo clippy     # linter — aponta problemas e sugere melhorias
cargo test       # roda os testes
```

- `cargo fmt` é obrigatório — o código deve estar formatado.
- `cargo clippy` não deve ter warnings. Se tiver, corrija antes de entregar.
- Código com warnings do compilador (`unused variable`, `dead_code`, etc.) perde pontos em D5.

---

## P8 — Testes (cobrado a partir da tarefa 35)

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-1, -1), -2);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 5), 5);
    }
}
```

**Convenção de nomes:** `test_<função>_<condição>_<resultado esperado>`  
Exemplo: `test_divide_by_zero_returns_error`

---

## P9 — Constantes vs Variáveis (cobrado a partir da tarefa 20)

```rust
// ❌ Número mágico
fn calculate_tax(price: f64) -> f64 {
    price * 0.15
}

// ✅ Constante nomeada
const TAX_RATE: f64 = 0.15;

fn calculate_tax(price: f64) -> f64 {
    price * TAX_RATE
}
```

- Use `const` para valores fixos conhecidos em tempo de compilação.
- Use `let` para valores calculados em runtime.
- Nunca deixe números ou strings "mágicas" sem nome.

---

## P10 — Estrutura de Arquivo (referência, não cobrado formalmente)

```
src/
  main.rs          ← ponto de entrada, apenas chama funções de outros módulos
  lib.rs           ← lógica principal (em projetos maiores)
  models/          ← structs e tipos de dados
  services/        ← lógica de negócio
  utils/           ← funções auxiliares reutilizáveis
```

Para os exercícios deste workspace, tudo em `main.rs` é aceitável até a tarefa 40.

---

## Resumo de Quando Cada Padrão é Cobrado

| Padrão | Descrição | Cobrado a partir de |
|--------|-----------|---------------------|
| P1 | Nomenclatura | Tarefa 20 |
| P2 | Documentação | Tarefa 20 |
| P3 | Design de funções | Tarefa 20 |
| P4 | Tratamento de erros | Tarefa 30 |
| P5 | Iteradores idiomáticos | Tarefa 25 |
| P6 | Ownership/borrowing correto | Tarefa 25 |
| P7 | `cargo fmt` + `cargo clippy` | Tarefa 20 |
| P8 | Testes unitários | Tarefa 35 |
| P9 | Constantes nomeadas | Tarefa 20 |
| P10 | Estrutura de módulos | Tarefa 40 |
