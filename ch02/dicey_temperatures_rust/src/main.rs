use std::io;

fn main() {
    println!("Guess the sum of two dice!");

    println!("Please input your guess (between 2 and 12).");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
