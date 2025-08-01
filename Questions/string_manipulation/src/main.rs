fn main(){
    let str: String = String::from("Hello World");
    println!("This is the original string, {}", str);
    let str_ref = &str[..];
    let str_ref_vowel = remove_vowels_second(&str_ref);
    let new_str = remove_vowels(str);
    println!("This is the modified string, {}", new_str);
    println!("This is the modified string, {}", str_ref_vowel);

}
fn remove_vowels(s: String) -> String{
    let vowel = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let result: String = s.chars()
        .filter(|c| !vowel.contains(c))
        .collect();
    result    
}

// s.chars() will return you all the characters of the string s like 'H', 'e', 'l', 'l', 'o' and so on. 
// filter is just as js one 
// collect will group them again 


fn remove_vowels_second(s: &str) -> String{
    let vowel = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    s.chars()
        .filter(|c| !vowel.contains(c))
        .collect()
}