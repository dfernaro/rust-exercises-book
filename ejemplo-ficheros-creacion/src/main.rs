use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejemplo.txt").expect("Error durante la creación");
    fichero.write_all("Este es mi fichero!".as_bytes()).expect("Error durante la escritura");
    println!("Fichero creado correctamente!");
}
