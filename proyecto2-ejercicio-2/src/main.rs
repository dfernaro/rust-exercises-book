fn sumar(lista: &[i32]) -> i32 {
    let mut suma = 0;

    for &numero in lista.iter() {
        suma = suma + numero;
    }

    suma
}

fn main() {
    let elementos = [10, 20, 30, 40, 50];
    let suma_total = sumar(&elementos);

    println!("El resultado obtenido es: {}", suma_total);
}
