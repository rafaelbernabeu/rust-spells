use std::io;
use std::io::Write;

fn main() {
    println!("Digite o seu nome:");

    let mut name: String = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Olá {}!", &name.trim());

    let name_encod = [82, 97, 102, 97, 101, 108, 10]; //UTF-8 as Decimal
    let error = [69, 114, 114, 111]; //UTF-8 as Decimal
    io::stdout().write(&name_encod).unwrap();
    io::stderr().write(&error).unwrap();
}
