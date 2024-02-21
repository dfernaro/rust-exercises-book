fn main() {
    let texto = "Me encantan los perros";
    let mut total = 0;

    for _palabra in texto.split_whitespace() {
        total = total + 1;
    }

    println!("Total: {}", total);
}
