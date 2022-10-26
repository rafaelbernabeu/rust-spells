use std::process::{Command, Output};
use std::string::{String, FromUtf8Error};

fn main() {

    let output: Output = Command::new("ls")
        .arg("/")
        .output()
        .expect("Failed to execute command");

    let output_str: Result<String, FromUtf8Error> = String::from_utf8(output.stdout.to_vec());
    match output_str {
        Ok(str) => { println!("Resultado: {}", str); }
        Err(error) => println!("Erro: {}", error),
    }
   
}
