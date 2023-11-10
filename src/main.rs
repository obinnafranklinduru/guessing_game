// Import necessary libraries
use rand::Rng; // For random number generation
use std::cmp::Ordering; // For comparing values
use std::io; // For user input
use colored::*; // For terminal text coloring

// Define a constant range for the secret number
const SECRET_NUMBER_RANGE: std::ops::RangeInclusive<u32> = 1..=100;

// Function to get user input for guessing the number
fn get_user_guess() -> u32 {
    loop {
        // Prompt the user to input their guess
        println!("Please input your guess.");

        // Read the user's input from the console
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the input into a u32, handling errors
        match guess.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Main function where the game logic is implemented
fn main() {
    // Print the initial message to start the game
    println!("Guess the number!");

    // Generate a random secret number within the defined range
    let secret_number = rand::thread_rng().gen_range(SECRET_NUMBER_RANGE);

    // Main game loop
    loop {
        // Get the user's guess
        let user_guess = get_user_guess();

        // Print the user's guessed number
        println!("You guessed: {}", user_guess);

        // Compare the user's guess with the secret number and provide feedback
        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".yellow()),
            Ordering::Equal => {
                // Print a success message if the guess is correct and exit the loop
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
