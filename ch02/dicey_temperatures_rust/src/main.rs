use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the sum of two dice!");

    let sum_of_two_dice = rand::thread_rng().gen_range(2..=12);

    println!("The sum of two dice is: {sum_of_two_dice}");

    let mut previous_guess: Option<u32> = None;
    let mut guess = String::new();

    loop {
        println!("Please input your guess (between 2 and 12).");

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match previous_guess {
            None => match guess.cmp(&sum_of_two_dice) {
                Ordering::Less | Ordering::Greater => {
                    println!("You guessed it wrong on the first try.");
                    previous_guess = Some(guess);
                }
                Ordering::Equal => println!("You win!"),
            },
            Some(prev) => match guess.cmp(&sum_of_two_dice) {
                Ordering::Less | Ordering::Greater => {
                    let previous_diff = (prev as i32 - sum_of_two_dice as i32).abs();
                    let current_diff = (guess as i32 - sum_of_two_dice as i32).abs();

                    if current_diff < previous_diff {
                        println!("Hotter.");
                    } else if current_diff > previous_diff {
                        println!("Colder.");
                    } else {
                        println!("Neither cold nor hot.");
                    }

                    previous_guess = Some(guess);
                }
                Ordering::Equal => println!("You win!"),
            },
        }
    }
}
