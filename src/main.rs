use std::{cmp::Ordering, io::stdin};

use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");

    let secret_number: u8 = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u8 = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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
