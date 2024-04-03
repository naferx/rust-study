
use std::io;

fn main() {
    println!("Guessing the number");
    println!("Enter a number: ");

    let mut number = String::new();
    io::stdin()
    .read_line( &mut number)
    .expect("Failed to get a number");

    println!("Your number is {number}");   
    
}