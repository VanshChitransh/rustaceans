// use std::collections::HashMap;
// fn main() {
//     println!("Hello, world!");
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Red"), 20);

//     for (key, value) in &scores{
//         println!("{key}: {value}");
//     }

//     println!();

//     scores.entry(String::from("Blue")).or_insert(50);
//     scores.entry(String::from("Red")).or_insert(50);
//     scores.entry(String::from("Yellow")).or_insert(30);

//     for (key, value) in &scores{
//         println!("{key}: {value}");
//     }
// }




// counts how many times each word appears in some text
// hello world wonderful world

use std::collections::HashMap;
fn main(){
    let mut res = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = res.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", res);
}