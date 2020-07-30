use std::process::{Command, Output};
use std::string::String;

fn main() {
    let output: Output = Command::new("ls")
        .arg("/")
        .output()
        .expect("Failed to execute command");

    let mut output_str: String = String::from_utf8(output.stdout.to_vec()).unwrap();
    println!("{}", output_str);
}
