use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        print!("\nPlease input your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("The secret number is between 1 and 100!");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed: {guess} is too small!"),
            Ordering::Greater => println!("You guessed: {guess} is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
