struct Ejemplo {
    atributo1:u32,
    atributo2:u32
}

fn main() {
    let mut ejemplo = Ejemplo {
        atributo1:10,
        atributo2:20
    };
    println!("Antes: {}", ejemplo.atributo1);
    ejemplo.atributo1 = 15;
    println!("Después: {}", ejemplo.atributo1);
}
