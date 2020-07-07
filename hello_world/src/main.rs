use std::env;
use std::env::Args;

use loops::m_lps;

fn main() {
    loops::hello_from_other_module();

    m_lps::loop_loop();
    m_lps::loop_while();
    m_lps::loop_for();

    self::hello_world();
}

fn hello_world() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Program started without an argument.");
    }

    println!("System started at {} with {} arguments", args[0], args.len());
    println!("Hello world!\nWelcome {:?}", args[1..].join(" "));
}

