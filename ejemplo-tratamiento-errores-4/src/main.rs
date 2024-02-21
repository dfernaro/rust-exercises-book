use std::fs::File;

fn main() {
    let fichero = File::open("fichero.txt").unwrap();
    
    println!("Mensaje final");
}
