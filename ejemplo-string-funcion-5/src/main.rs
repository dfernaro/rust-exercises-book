fn main() {
    let ejemplo = "columna1 columna2 columna3";

    for columna in ejemplo.split_whitespace() {
        println!("{}", columna);
    }
}
