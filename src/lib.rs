use rand::Rng;
use std::cmp::Ordering;
use std::io;
use wasm_bindgen::prelude::*;

pub fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100...");

    fn pick_new_number() -> i32 {
        // Generate a random number between 1 and 100
        let number = rand::thread_rng().gen_range(1..=100);
        println!("Debug: The secret number is {}", number);
        number
    }

    let mut secret_number: i32 = pick_new_number();
    let mut guess_as_number: i32 = 0;

    let mut score: i32 = 0;
    let mut round: i32 = 1;
    let mut attempts: i32 = 0;
    let max_attempts: i32 = 3;

    loop {
        attempts += 1;
        println!(
            "Round #{}. This is attempt {} of {}.",
            round, attempts, max_attempts
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if "exit" == guess {
            println!("You have exited the game");
            break;
        } else if "score" == guess {
            println!("Your current score is {}.", score)
        } else {
            guess_as_number = match guess.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("You need to enter a number my friend.");
                    attempts -= 1;
                    continue;
                }
            }
        }

        if guess_as_number != 0 {
            match guess_as_number.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too low buddy!");
                    score -= 1;
                    if attempts >= max_attempts {
                        println!(
                            "😞 Game over! The number was {}.\n New game!\n",
                            secret_number
                        );
                        secret_number = pick_new_number();
                        attempts = 0;
                        round += 1;
                    }
                }
                Ordering::Greater => {
                    println!("Too high buddy!");
                    score -= 1;
                    if attempts >= max_attempts {
                        println!(
                            "😞 Game over! The number was {}.\n New game!\n",
                            secret_number
                        );
                        secret_number = pick_new_number();
                        attempts = 0;
                        round += 1;
                    }
                }
                Ordering::Equal => {
                    score += 1;
                    if attempts > 1 {
                        println!(
                            "🎉 You nailed it on the first attempt! \nYour current score is {}.\n",
                            score
                        );
                    } else {
                        println!(
                            "🎉 You nailed it in {} attempts! \nYour current score is {}.\n",
                            attempts, score
                        );
                    }
                    secret_number = pick_new_number();
                    attempts = 0;
                    round += 1;
                }
            }
        }
    }
}
