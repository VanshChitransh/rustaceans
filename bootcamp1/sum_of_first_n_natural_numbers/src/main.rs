use std::io;
use std::io::Write;

fn main() {
    print!("Please enter the number -> ");
    io::stdout().flush().expect("Error while clearing the buffer");
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Error while reading the input");
    let number: i32 = input_number.trim().parse().expect("Please enter a valid number");
    let mut sum: i32 = 0; 
    for i in 1..number+1{
        sum += i;
    }
    println!("This is the final sum {}", sum);
}
