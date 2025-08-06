// fn main() {
//     println!("Hello, world!");
//     let s = "This is the initial string";
//     println!("This is string -> {} ", s);
//     let s1 = String::from("This is s2");
//     println!("{}", s1);
//     let s2 = ("This is s3").to_string();
//     println!("{s2}");
//     let s3 = String::new();
// }
// we can use String::new()
// String::from();
// ("string literal").to_string()
// difference between string literal and String

fn main(){
    // let mut s = ("foo").to_string();
    // s.push_str("bar");
    // println!("s is {}", s);
    // s.push('r');
    // println!("S is {}", s);

    let a = 'a';
    println!("{a}");
    let b = "b";
    println!("{b}");

    // we can add two strings with .push_str for pushing a string or we can use .push for a character to push. 
}



// sometimes we want to concatinate two strings
// for it we have two ways in rust to concatinate two strings. 
// 1-> + (add method)
// 2-> format!() (using format macro)
// the + operator / add metod takes the ownership of the first string passed to it
// suppose if we are doing let s3 = s1 + s2 then the ownership of the s1 would be passed to add function and it will return a whole new string by combining s1 with s2. 
// This is the default implementation of the add method in the standard library of rust. 
// the + operator method/ add method have two flaws, it's confusing to add 5-6 strings and the ownership rule which take place here..
// if we don't want to ownership to get transfered we can use format!()
// it can add any number of arguments given to it and it also don't take ownsership of any of it's parameters. 




// indexing in strings. 
// we can not access any element like this -> 
//    let s1 = String::from("hi");
//    let h = s1[0];
// because strings in rust are utf8 encoded and it's not guranteed that each element will only take exactly one byte. 
// a english character may take only one byte but an emojie can take 3-4 bytes and the string is stored in heap (continuous memory)
// and doing [0] would result in the first byte of that whole emojie which will cause error.
//            let hello = "Здравствуйте";
//            let answer = &hello[0];
// now in this case each element take 2 bytes and doing &hello[0] will return it the first byte of this thing, which will cause issue. 
// so rust don't allow doing this. 


// as UTF-8 encoding store string in bytes of u8 and one character can take upto 4 bytes, so utf-8 encoding does not allow you to have (O(1)) time complexity hence rust does not using indexing. 
//Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. 
