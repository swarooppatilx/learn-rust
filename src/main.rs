use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");
    let mut guess = String::new(); //mutable variable in rust
    let apple = 69; //immutable variable or constant in other languages, by default variables are immutable in rust pretty wierd!
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    println!("You guessed {}", guess);
    println!("The value of apple {apple}");
}
