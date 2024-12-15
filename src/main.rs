use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=69);
    println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    let guess: u32 = guess.trim().parse().expect("Please type a number"); //shadow variable of type 32 bit unsigned integer
    println!("Your guess: {}, secret number: {}", guess, secret_number);
    match guess.cmp(&secret_number){
     Ordering::Less => println!("Too small"),
     Ordering::Greater => println!("Too big"),
     Ordering::Equal => println!("Perfect, You win!"),
    }
}
