fn main() {
    let ejemplo = "columna 1,columna 2,columna 3";

    for columna in ejemplo.split(",") {
        println!("{}", columna);
    }
}
