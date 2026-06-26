fn eh_primo(numero: i32) -> bool {
    // Requisito 1: Se o número for menor ou igual a 1, não é primo
    if numero <= 1 {
        return false;
    }

    // Requisito 2: Criar a variável 'limite' com a raiz quadrada arredondada para cima
    // Convertemos para f64 para usar as funções matemáticas e voltamos para i32
    let limite = (numero as f64).sqrt().ceil() as i32;

    // Requisito 3 e 4: Loop for de 2 até o limite (inclusive) para testar divisores
    for i in 2..=limite {
        // Se o 'i' for igual ao próprio número (ex: no caso do 2 ou 3), 
        // não contamos como divisor de exclusão.
        if i == numero {
            continue;
        }
        
        if numero % i == 0 {
            return false; // Encontrou um divisor exato, não é primo
        }
    }

    // Requisito 5: Nenhum divisor encontrado
    true
}

fn main() {
    let numero: i32 = 1001;
    let resultado: bool = eh_primo(numero);
    println!("{}", resultado);
}