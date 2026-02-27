use std::io::{self, Write};

fn main() {
    println!("Guess the Number!");

    print!("Please input your guess: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {guess}");
}
