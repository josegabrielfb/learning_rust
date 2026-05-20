fn maior_numero(numero: &[i32]) -> i32 {
    let mut maior: i32 = numero[0];

    for &i in numero {
        if i >= maior {
            maior = i;
        }
    }

    maior
}

fn main() {
    let numeros = vec![2, 2, 6, 8, 4, 5, 20];
    let maior = maior_numero(&numeros);
    println!("{}", maior);
}
