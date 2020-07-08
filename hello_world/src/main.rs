use std::io;
use std::io::Read;

fn main() {
    println!("Digite o seu nome:");

    let mut nome: String = String::new();

    io::stdin().read_line(&mut nome);

    print!("Olá {} !", &nome.trim());
}
