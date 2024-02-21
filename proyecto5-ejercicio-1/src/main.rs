use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("proyecto.txt").expect("Error durante la creación");
    fichero.write_all("Me encanta Rust!\n".as_bytes()).expect("Error durante la escritura");
    fichero.write_all("Estoy aprendiendo mucho!\n".as_bytes()).expect("Error durante la escritura");
    println!("Fichero creado correctamente!");
}
