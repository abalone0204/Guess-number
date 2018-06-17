extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut max = 100;
    let mut min = 1;
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("------");
        println!("Current range: {} - {}", min, max);
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess > max || guess < min {
            println!("Out of range");
            continue;
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => {

                min = guess;
            },
            Ordering::Greater => {
                max = guess;
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
