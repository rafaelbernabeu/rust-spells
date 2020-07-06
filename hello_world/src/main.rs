use std::env;
use std::env::Args;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Program started without an argument.");
    }

    println!("System started at {} with {} arguments", args[0], args.len());
    println!("Hello world!\nWelcome {:?}", args[1..].join(" "));
}