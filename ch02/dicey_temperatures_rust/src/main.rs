use rand::Rng;
use std::io;

fn main() {
    println!("Guess the sum of two dice!");

    let sum_of_two_dice = rand::thread_rng().gen_range(2..=12);

    println!("The sum of two dice is: {sum_of_two_dice}");

    println!("Please input your guess (between 2 and 12).");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
