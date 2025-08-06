// fn main(){
//     let str:String = String::from("Hello World");
//     let len: usize = get_len_of_first_break(&str);
//     println!("The length of first break here in {} is at {}", str, len);
// }

// fn get_len_of_first_break(s: &String) -> usize{
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         };
//     }
//     return bytes.len();
// }


fn main(){
    let mut str = String::from("Hello world");
    let len_word = len_of_word(&str);
    str.clear();
    let new = len_word;
    println!("The length of word is {}", new);
}
fn len_of_word(s: &String) -> usize {
    s.len()
}