extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game!");

    let secrete_number = rand::thread_rng().gen_range(1, 101);

    println!("Secrete number is : {}", secrete_number);

    loop {
        let mut guess = String::new();

        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get the input!");

        println!("You guessed as {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You guessed right :)");
                break;
            },
        };
    }
}
