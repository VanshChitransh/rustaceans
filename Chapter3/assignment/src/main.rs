// try building programs to do the following:

// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.



fn main() {
    println!("Hello, world!");
    let fahrenheit: f64 = convert_temperatures_to_fahrenheit(100.0);
    println!("{fahrenheit}");
    println!();
    let result = generate_nth_fibonacci(10);
    println!("Fibonacci till 10, {}", result);
}

fn convert_temperatures_to_fahrenheit(temperatures: f64) -> f64{
    let result = (temperatures * 9.0/5.0) + 32.0;
    result
}

fn generate_nth_fibonacci (n:u64) -> u64{
    if n == 0 {
        return 0; 
    } 
    else if n == 1 {
        return 1;
    }
    else {
         return generate_nth_fibonacci(n-1)+ generate_nth_fibonacci(n-2);
    }
}