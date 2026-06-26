fn eh_primo(numero: i32) -> bool {
    if numero <= 1 {
        return false;
    }

    let limite = (numero as f64).sqrt().ceil() as i32;

    for i in 2..=limite {
<<<<<<< HEAD
=======
        // Se o 'i' for igual ao próprio número (ex: no caso do 2 ou 3),
        // não contamos como divisor de exclusão.
>>>>>>> d966046d497802c12c6fa1393894b612fb2d3691
        if i == numero {
            continue;
        }

        if numero % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numero: i32 = 1001;
    let resultado: bool = eh_primo(numero);
    println!("{}", resultado);
}
