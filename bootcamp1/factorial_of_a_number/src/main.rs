use std::io;
use std::io::Write;
fn main() {
    print!("Please enter the number -> ");
    io::stdout().flush().expect("Error while clearing the buffer");
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Error while reading the input");
    let number: i32 = input_number.trim().parse().expect("Please enter a vlaid number");
    let factorial = calculate_factorial(number);
    print!("This is the final answer -> {}", factorial);
    io::stdout().flush().expect("Error while clearing the buffer");
}

fn calculate_factorial(n: i32) -> i32{
    if n == 0 || n == 1{
        1
    }else{
        n * calculate_factorial(n-1)
    }
}