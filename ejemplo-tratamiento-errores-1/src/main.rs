use std::fs::File;

fn main() {
    let fichero = File::open("fichero.txt");
    
    match fichero {
        Ok(_f) => {
            println!("El fichero existe");
        },
        Err(_e) => {
            println!("Fichero no encontrado");
        }
    }

    println!("Mensaje final");
}
