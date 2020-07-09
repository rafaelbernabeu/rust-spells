use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret:u8 = rand::thread_rng().gen_range(0, 101);
    loop {
        println!("Guess a number from [0..100]");

        let mut guessed: String = String::new();

        io::stdin()
            .read_line(&mut guessed)
            .expect("Failed to read line");

        let guessed:u8 = guessed.trim().parse().expect("Please type a number.");

        match guessed.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
