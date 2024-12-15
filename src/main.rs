use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=69);
    println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    println!("You guessed {}", guess);
    println!("The secret number is {secret_number}");
}
