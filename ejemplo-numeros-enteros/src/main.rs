fn main() {
    let entero_con_signo_32: i32 = -15;
    let entero_con_signo_64: i64 = 123456789012345;

    let entero_sin_signo_32: u8 = 15;
    let entero_sin_signo_64: u16 = 12345;

    println!("Entero con signo (32 bits): {entero_con_signo_32}");
    println!("Entero con signo (32 bits): {entero_con_signo_64}");
    println!("Entero sin signo (64 bits): {entero_sin_signo_32}");
    println!("Entero sin signo (64 bits): {entero_sin_signo_64}");
}
