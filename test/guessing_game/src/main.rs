use std::{
    cmp::Ordering,
    io::{self},
};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The guess number:{}", secret_number);

    loop {
        println!("Please input your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
        println!("your guess {}", guess);
    }
}
