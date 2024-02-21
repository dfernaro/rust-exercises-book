enum Genero {
    Masculino, Femenino
}

fn imprimir_genero(genero: Genero) {
    match genero {
        Genero::Masculino => {
            println!("El valor es Masculino!");
        },
        Genero::Femenino => {
            println!("El valor es Femenino!");
        }
    }
}

fn main() {
    imprimir_genero(Genero::Femenino);
}
