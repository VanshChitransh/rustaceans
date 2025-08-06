use std::io;

fn main() {
    println!("Hello, world!");
    
    // In this example i am writing code with the use of vectors, you will have to write code on your own with the help of array as well.

    let mut input_vector = Vec::new();
    let mut input_number = String::new();

    for i in 1..4{
        println!("Please enter the {}th number", i);
        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read the input");
        let number: i32 = input_number.trim().parse().expect("Please enter a valid number");
        input_vector.push(number);
        input_number.clear();
        // we can even remove this .clear line, if we will take that input_number line into the block scope of if block. 
    }

    let value_1 = match input_vector.get(0){
        Some(value) => value, 
        None => {
            println!("Index out of bound");
            return;
        }
    };

    let value_2 = input_vector.get(1).unwrap();

    let value_3 = input_vector.get(2).expect("Index out of bound");
    // these are the major ways to get the index element from a vector

    if value_1 > value_2 && value_1 > value_3 {
        println!("Largest value is, {}", value_1);
    }
    else if value_2 > value_3 && value_2 > value_1 {
        println!("Largest value is, {}", value_2);
    } else {
        println!("Largest value is, {}", value_3);
    }


    println!("These are the inputs, {:?}", input_vector);
}




// Array implementation hint -> let mut input_array: [i32; 3] = [0; 3];
// input_array[i] = number.trim().parse().expect("Message");
// let num_1 = input_arary[0]; and so on... 
//try this method