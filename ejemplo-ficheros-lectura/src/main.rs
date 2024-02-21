use std::fs;

fn main() {
    let contenido_fichero = fs::read_to_string("ejemplo.txt").expect("Error leyendo el fichero");
    println!("Contenido: {}", contenido_fichero);
}
