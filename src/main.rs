extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Enter the guess");

    let secret_number = generate_random_number();

    println!("The secret number is {}", secret_number);

    interactive_loop(&secret_number);
}

fn interactive_loop(secret_number: &u32) {
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}

fn generate_random_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}
