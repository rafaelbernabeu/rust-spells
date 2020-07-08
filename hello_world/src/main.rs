use std::io;
use std::io::{Read, Write};

fn main() {
    println!("Digite o seu nome:");

    let mut name: String = String::new();

    io::stdin().read_line(&mut name);

    print!("Olá {}!", &name.trim());

    let name_encod = [10, 82, 97, 102, 97, 101, 108]; //UTF-8 as Decimal
    io::stdout().write(&name_encod);
}
