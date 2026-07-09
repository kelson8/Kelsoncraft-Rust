use std::cmp::Ordering;
use std::io;
use rand::Rng;

// Taken from the Rust docs here
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// Disables the unused error for this.
#[allow(unused)]
pub fn number_guesser() {
    // println!("Guess the number!");

    // Generate a random number
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess.");

        // By default, variables in rust are immutable, meaning they cannot be modified.
        // Adding mut to it allows it to be changed elsewhere.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // println!("You guessed: {}", &guess);
        // Wow, rust has a python like f string syntax, I didn't know that.
        println!("You guessed: {guess}");

        // This compares the two values, and also uses the Rust
        // alternative for switch statements.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
                {
                    println!("You win!");
                    break;
                }
        }
    }

}