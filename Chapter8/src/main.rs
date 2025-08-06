// fn main(){
//     let v = vec![1,2];
//     // let third: &i32 = &v[2];
//     // println!("This is the third element of the vector -> {}", third);
//     let third_element = v.get(2);
//     match third_element{
//         Some(third_element) => println!("This is the third element of the vector from an option -> {}", third_element),
//         None => println!("Mar gaya madarchod.... la la la lalaa la"),
//     }
// }

// from reference and borrowing rule, we can not have muttable and immutable reference in the same scope. 

// fn main(){
//     let mut v = vec![1,2,3,4,5];
//     let first = &v[0];
//     v.push(6);
//     println!("{:?}", first);
    
// }



// fn main(){
//     let mut v = vec![1,2,3,4,5];
//     for i in &v{
//         print!("{i} ");
//     }
//     // println!("{:?}", v);
//     // note the & in for loop....

//     println!("");

//     for i in &mut v{
//         *i += 50;
//         print!("{i} ");
//     }
// }




#[derive(Debug)]
#[allow(dead_code)]
enum MyCustomDataType{
    Int(i32),
    Text(String),
    Float(f32),
    Boolean(bool),
}

fn main(){
    let row = vec![
        MyCustomDataType::Int(5),
        MyCustomDataType::Text(String::from("Hello")),
        MyCustomDataType::Float(3.14),
        MyCustomDataType::Boolean(true),
    ];
    let first = row.get(0);
    match first{
        Some(MyCustomDataType::Int(value)) => println!("This is the first element of the vector -> {}", value),
        Some(other) => println!("This is other -> {:?}", other),
        None => println!("Mar gaya Madarchod la la la lalaa la"),
    };

    let second = row.get(1);
    match second{
        Some(second) => println!("This is the second element of the vector -> {:?}", second),
        None => println!("Mar gaya madarchod la la la lalaa la"),
    };

    let third = row.get(2);
    match third{
        Some(third) => println!("This is the third element of the vector -> {:?}", third),
        None => println!("Mar gaya madarchod la la la lalaa la"),
    }

    let fourth = row.get(3);
    match fourth{
        Some(fourth) => println!("This is the third element of the vector -> {:?}", fourth),
        None => println!("Mar gaya madarchod la la la lalaa la"),
    }
}