use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    // Print a welcome message
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_number);

    // Main game loop
    loop {
        // Prompt the user for input
        println!("Please input your guess.");

        // Create a mutable string to store user input
        let mut guess: String = String::new();
        
        // Read user input from the standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user input to an unsigned 32-bit integer, handling errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and prompt for input again
        };

        // Print the user's guess
        println!("You guessed: {}", guess);

        // Compare the user's guess with the secret number and provide feedback
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                // Print a winning message and exit the game loop
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
