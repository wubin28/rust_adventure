use rand::Rng;
use std::cmp::Ordering;
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

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&sum_of_two_dice) {
        Ordering::Less | Ordering::Greater => println!("You guessed it wrong on the first try"),
        Ordering::Equal => println!("You win!"),
    }
}
