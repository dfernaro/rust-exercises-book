use std::fs;

fn main() {
    let contenido_fichero = fs::read_to_string("proyecto.txt").expect("Error leyendo el fichero");
    println!("{}", contenido_fichero);
}
