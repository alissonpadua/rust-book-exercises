use rand::distr::{Distribution, Uniform};
use rand::rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let mut rng = rng();
    let range = Uniform::new_inclusive(0, 100).expect("ai meu cu");
    let secret_number = range.sample(&mut rng);

    println!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
    }
}
