use std::fs;

fn main() {
    let contenido_fichero = fs::remove_file("ejemplo.txt").expect("Error borrando el fichero");
}
