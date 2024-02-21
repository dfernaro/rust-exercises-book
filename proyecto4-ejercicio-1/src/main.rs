struct Vehiculo {
    matricula:String,
    nombre:String
}

fn main() {
    let vehiculo = Vehiculo {
        matricula:String::from("XXXXXX"),
        nombre:String::from("Mi coche")
    };
    
    println!("Matrícula: {}", vehiculo.matricula);
    println!("Nombre: {}", vehiculo.nombre);
}
