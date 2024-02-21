fn maximo(lista: &[i32]) -> i32 {
    let mut maximo = lista[0];

    for &numero in lista.iter() {
        if numero > maximo {
            maximo = numero;
        }
    }

    maximo
}

fn main() {
    let numeros = [15, 69, 78, 55, 100, 365];
    let maximo_valor = maximo(&numeros);
    println!("El máximo valor encontrado es: {}", maximo_valor);
}
