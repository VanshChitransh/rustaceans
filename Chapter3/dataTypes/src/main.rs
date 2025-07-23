use std::io;
fn main(){
    // Broadly there are two type of data types -> Scalar and compound 
    // Scalar -> Integer, integer literal, float-point(f32, f64 -> more precise), numeric operation, boolean, character(4bytes)
    // Compound -> tuple, Array 

    // tuple have multiple values of multiple types 
    // they are of fixed size

    let tup:(i32, f64, u8) = (500,6.4,1);
    let (x,y,z) = tup;
    println!("This is the value of x, {}", x);
    println!("This is the value of y, {}", y);
    println!("This is the value of z, {}", z);
    println!("");
    println!("We will be using the different syntax for accessing the values of tuple");
    println!();
    println!("This is the value of x, {}", tup.0);
    println!("This is the value of y, {}", tup.1);
    println!("This is the value of z, {}", tup.2);

    // array (2nd coumpound data type)
    // array only have fixed types of values
    // length of array is fixed.

    let a: [u32;5] = [1,2,3,4,5];
    let b: [u32;5] = [1;5];
    println!("{:?}", a);
    println!("{:?}", b);
    println!("");

    for i in 0..a.len(){
        print!("{} ", a[i]);
    }
    println!();
    println!("Now we will print the second array, which is b");
    for i in 0..b.len(){
        print!("{} ", b[i]);
    }

    println!("");
    let trial_variable = 5;
    println!("{:?}", trial_variable);


    println!("Let's play a game");
    println!("Please enter a index")
    let a:[u32;5] = [1,2,3,4,5];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");
    let index: usize = index.trim().parse().expect("Please enter a valid number");
    let element = a[index];
    println!("This is the number at the given index, {}", element);
}