mod utils;

// comment here
use std::io;
use utils::number;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("You guess: {guess}");

    println!("number {}", number());
}
