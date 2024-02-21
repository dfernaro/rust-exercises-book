struct Persona {
    nombre:String,
    edad:u32,
    color:String
}

fn main() {
    let persona = Persona {
        nombre:String::from("David"),
        edad:18,
        color:String::from("Rojo")
    };
    
    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);
    println!("Color: {}", persona.color);
}
