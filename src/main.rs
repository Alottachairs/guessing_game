use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("What is the number?");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guesses = 0u32;
    loop {
    println!("Please input your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim()
        .parse()
        .expect("Please type a number!");
    
    num_guesses += 1;

    println!("You guessed: {guess}");
    println!("number of guesses: {}", num_guesses);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
