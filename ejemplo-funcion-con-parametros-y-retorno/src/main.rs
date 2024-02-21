fn cuadrado(numero: i32) -> i32 {
    numero * numero
}

fn main() {
    let numero = 5;
    let resultado = cuadrado(numero);
    println!("El cuadrado de {} es: {}", numero, resultado);
}
