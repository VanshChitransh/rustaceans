#[warn(unused_imports)]
use chapter7::authenticate;
use chapter7::Credentials;


// we imported both the things which we made public in the lib.rs file. 

fn main() {
    let cred: Credentials = Credentials{
        username: "Vansh".to_string(),
        password: "myPass".to_string(),
    };
    println!("This is my name, {}", cred.username);
    println!("This is my password, {}", cred.password);
    // this will give error, as username and password are private field. 
    // altough we have made struct public but it's field are still private. 
}
