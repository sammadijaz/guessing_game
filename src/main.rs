use std::io;

fn main() {
    print!("Guess the Number!");

    print!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    
    print!("You guessed: {guess}");
}