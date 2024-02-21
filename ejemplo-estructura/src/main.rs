struct Alumno {
    nombre:String,
    apellido:String,
    edad:u32
}

fn main() {
    let alumno = Alumno {
        nombre:String::from("David"),
        apellido:String::from("Fernández"),
        edad:21
    };
    println!("Nombre: {}", alumno.nombre);
    println!("Apellido: {}", alumno.apellido);
    println!("Edad: {}", alumno.edad);
}
