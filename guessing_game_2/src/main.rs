use std::io;
use rand::Rng;

fn main() {
    let mut score = 0;

    'outer: loop {
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        let mut attempts = 0;

        println!("Score: {score}");
        println!("Guess the number!");
        println!("Number is {secret_number}");

        loop {
            println!("Please input guess:");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");

            if guess == secret_number {
                println!("Correct!");
                score += 1;
                break;
            } else if guess < secret_number {
                println!("Too small!");
            } else {
                println!("Too big!");
            }

            attempts += 1;

            if attempts > 3 {
                println!("You lose!");
                println!("Score: {score}");
                break 'outer;
            }
        }
    }
}