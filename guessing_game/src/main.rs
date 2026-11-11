use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut lives = 5;

    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        if lives == 0 {
            println!("You are out of lives");
            break;
        }
        lives -= 1;
        println!("Please input your guess.  ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
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
