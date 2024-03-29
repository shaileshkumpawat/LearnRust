//use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number! ");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    
    println!("The secret number is: {}", secret_number);
    
    println!("Please input your guess ");

    let mut guess = String::new();
    std::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!(("Too Small!")),
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too Big!")
    }
}
