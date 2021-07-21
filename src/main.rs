use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "To small".red()),
            Ordering::Greater => println!("{}", "To big".red()),
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            },
        }
    }
}

