use rand::distr::{Distribution, Uniform}; // Needed for sampling from a range
/// A simple guessing game that picks a random number between 0 and 100
/// and prompts the user to guess it until correct.
use rand::rng; // Modern RNG initializer (non-deprecated)
use std::cmp::Ordering; // For comparing guessed vs secret number
use std::io::{self, Write}; // For input/output operations

fn main() {
    println!("Guess the number (0–100)!");

    // Initialize a thread-local random generator
    let mut rng = rng();

    // Create an inclusive distribution from 0 to 100
    // `new_inclusive` returns Result, so we `.expect()` to get Uniform
    let range = Uniform::new_inclusive(0, 100).expect("Failed to create uniform distribution");

    // Sample the secret number from the distribution
    let secret_number = range.sample(&mut rng);

    println!("(DEBUG) Secret number: {}", secret_number);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");
        // flush ensures the prompt appears before input

        let mut guess = String::new(); // Buffer to hold user input

        // Read one line from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Try to parse the input into a `u32`
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // Successfully parsed number
            Err(_) => continue, // On parse failure, restart loop
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  // Guess is lower
            Ordering::Greater => println!("Too big!"), // Guess is higher
            Ordering::Equal => {
                println!("🎉 You win!"); // Correct guess
                break; // Exit loop and program
            }
        }
    }
}
