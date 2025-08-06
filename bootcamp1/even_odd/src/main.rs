use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter the number");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    if number % 2 == 0 {
        println!("The number you entered, which is {}, is even.", number);
    } else {
        println!("The number you entered, which is {}, is odd.", number);
    }
    // '/' will give us the quotent 
    // '%' will give us the remainder
}
