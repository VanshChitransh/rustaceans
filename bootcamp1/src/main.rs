// fn main(){
//     let x:String = String::from("Hello");
//     println!("{}", x);
//     let char1 = x.chars().nth(0);
//     match char1{
//         Some(x) => println!("{:?}", x),
//         None => println!("No character at the given index"),
//     }
//     println!("{:?}", char1);
// }

// fn main(){
//     for i in 0..11{
//         print!("{} ", i);
//     }
// }

fn main(){
    let sentence: String = String::from("Hello from rust");
    let first_word = get_first_word(sentence);
    print!("{}", first_word);
}
fn get_first_word(sentence: String) -> String{
    let mut ans = String::from("");
    for i in sentence.chars(){
        ans.push_str(i.to_string().as_str());
        if i == ' ' {
            break;
        }
    }
    ans
}