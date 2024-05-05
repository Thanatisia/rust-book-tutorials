use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Print a new line
    println!("Guess the number!");

    // Declare and initialize a secret number variable holding a randomized number generated 
    // from a range of numbers between minimum of 1 and a maximum of 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    // Initialize a new string object and make it mutable (in rust, variables are immutable by default)
    let mut guess = String::new();

    // Obtain Standard input from the user, read the line and store the value into the memory address pointing to the variable 'guess'
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Convert the user input string into a real number/integer type 
    // so that we can compare it numerically to the secret number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Print result with format string
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
