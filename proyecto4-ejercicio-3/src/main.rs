#[derive(Debug)]
enum Color {
    Rojo, Verde, Amarillo
}

struct Persona {
    nombre:String,
    edad:u32,
    color:Color
}

fn main() {
    let persona = Persona {
        nombre:String::from("David"),
        edad:18,
        color:Color::Amarillo
    };
    
    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);
    println!("Color: {:?}", persona.color);
}
