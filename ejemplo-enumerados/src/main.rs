#[derive(Debug)]
enum Ciudad {
    Sevilla, Madrid
}

#[derive(Debug)]
enum Color {
    Rojo, Azul, Verde
}

fn main() {
    let ciudad = Ciudad::Sevilla;
    let color = Color::Rojo;
    println!("{:?}", ciudad);
    println!("{:?}", color);
}
