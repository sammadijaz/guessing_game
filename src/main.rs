use std::io::{self, Write};

use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    print!("The secret number is {secret_number}");

    print!("Please input your guess: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {guess}");
}
