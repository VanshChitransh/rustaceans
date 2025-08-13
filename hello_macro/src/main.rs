use hello_macro::HelloMacro;
struct Pancake;
impl HelloMacro for Pancake{
    fn hello_macro(){
        println!("Hello, Macro! My name is pancake");
    }
}
fn main(){
    Pancake::hello_macro();
}