use std::io;
use std::io::Write;

fn main() {
    print!("Please enter the number -> ");
    io::stdout().flush().expect("Error while flushing the output");
    // think why we used flush here? 
    // hint -> buffer? 
    // what is buffer?? 
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Error while reading the input");
    let number: i32 = input_number.trim().parse().expect("Please enter a valid number");
    for i in 1..=10{
        println!("{} x {} = {}", number, i, number*i);
    }
}
