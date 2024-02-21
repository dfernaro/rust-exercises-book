fn es_mayor_edad(edad: i32) -> bool{
    let mut mayor = false;
    if edad >= 18 {
        mayor = true;
    }
    mayor
}

fn main() {
    let resultado = es_mayor_edad(19);
    println!("Es mayor de edad: {}", resultado);
}
