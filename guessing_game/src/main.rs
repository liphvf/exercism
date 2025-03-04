use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                if guess.trim() == "quit" {
                    println!("You quit!");
                    break;
                }

                println!("Please enter a number!");
                continue;
            }
            Ok(num) => num,
        };

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
