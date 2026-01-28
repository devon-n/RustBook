use std::io; // Import the input output library
use rand::Rng; // Import random number generator library
use std::cmp::Ordering; // Import comparison library

fn main() {
    	println!("Guess the number!");

	// Generate random number between 1 and 100
	let secret_number = rand::thread_rng().gen_range(1..=100);
	println!("The secret number is : {secret_number}");

    loop {

        println!("Please input your guess.");
        // Create an empty mutable string
        let mut guess = String::new(); 

        io::stdin() // Handle user input
            .read_line(&mut guess) // Read the input, store in guess
            .expect("Failed to read line");
        // read_line returns a Result type which expect handles if there is an error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare guess with secret number
        // Match the case based on less, greater or equal
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
