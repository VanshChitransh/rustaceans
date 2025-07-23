use std::{cmp::Ordering, io};
use rand::Rng;
fn main(){
    println!("Hey welcome to the guessing game!");
    println!("Please guess a number between 0-100");
    // let mut guess = String::new();
    let secret_number = rand::rng().random_range(0..=100);
    loop{
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Error reeading the input, please try again");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {println!("Please enter a valid number"); continue;}
        };
        match guess.cmp(&secret_number){
            Ordering::Equal => {println!("You win"); break;},
            Ordering::Greater => {println!("Too big, try guessing a smaller number")},
            Ordering::Less => {println!("Too small, try guessing a larger number")},
        };
        println!("This is the secret_number, {}", {secret_number});
    }
}