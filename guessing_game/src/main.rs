use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Type your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guessed must be a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too greater!"),
            Ordering::Less => println!("Too smal!"),
            Ordering::Equal => {
                println!("Congratulations, you win!");
                break;
            }
        }
    }
}
