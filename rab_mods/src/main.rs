use std::env;

use loops::m_lps;

fn main() {
    loops::hello_from_other_module();

    m_lps::loop_loop();
    m_lps::loop_while();
    m_lps::loop_for();

    self::hello_world();
}

fn hello_world() {
    print!("Hello World!");
}