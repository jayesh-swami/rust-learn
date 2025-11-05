use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let random_number = rand::rng().random_range(1..=100);

    println!("The secret number is {random_number}!");

    loop {
        println!("Guess a number! Input your guess - ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
