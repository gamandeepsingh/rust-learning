use rand::random_range;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");
    let sec_num = random_range(1..101);
    println!("Secret Num: {sec_num}");
    
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type Correct number");
                continue;
            },
        };
        match guess.cmp(&sec_num) {
            Ordering::Less => {
                println!("{}","Too small".red());
            }
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
            Ordering::Greater => {
                println!("{}","Too Large".red());
            }
        }
    }
}
