fn maior_numero(numero: &Vec<i32>) -> i32 {
    let mut maior: i32 = numero[0];

    for i in 0..numero.len() {
        if numero[i] >= maior {
            maior = numero[i];
        }
    }

    return maior;
}

fn main() {
    let numeros = vec![2, 2, 6, 8, 4, 5, 20];
    let maior = maior_numero(&numeros);
    println!("{}", maior);
}
