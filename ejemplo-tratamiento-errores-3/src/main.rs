use std::fs::File;

fn main() {
    let fichero = File::open("fichero.txt").expect("El fichero no existe");

    println!("Mensaje final");
}
