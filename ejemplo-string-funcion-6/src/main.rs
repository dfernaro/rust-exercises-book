fn main() {
    let entero: u32 = 10;
    let decimal: f32 = 10.5;
    let texto: String = String::from("Hola");

    let combinado: String = format!("{}, {}, {}", entero, decimal, texto);

    println!("{}", combinado);
}
