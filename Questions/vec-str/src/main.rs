fn main() {
    println!("Hello, world!");
    let mut vec_str: Vec<String> = vec!["hello".to_string(), "vansh".to_string(), "how".to_string(), "are".to_string(), "you".to_string(), "doing".to_string()];
    println!("This is the string, {:?}", vec_str);
    my_own_function(&mut vec_str);
    println!("This is the string now, {:?}", vec_str);
}

fn my_own_function(s: &mut Vec<String>){
    for word in s.iter_mut(){
        if let Some(first_char) = word.get_mut(0..1){
            let capitalized = first_char.to_uppercase();
            if word.len() > 1 {
                let rest = &word[1..];
                *word = format!("{}{}", capitalized, rest);
            } else{
                *word = capitalized.to_string();
            }
        }
    }
}