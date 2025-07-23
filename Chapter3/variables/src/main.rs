fn main(){
    let x = 5; 
    let x = x + 1;
    // gloabl scope
    println!("Value of x is {}", x);
    {
        let x = x * 2; 
        // local scope
        println!("This is the value of x {}", x);
    }
    let x = x + 2;
    println!("This is the final value of x {}", x);
}

// let 
// let mut 
// const -> immutable, type annotation, can be declared in global or local scope, Can't be assigned/bind to a value which can be computed at run time. Altough it can be assigned to a value which can be computed at compile time!! (Run time, No...... Compile time, Yes)
// shadowing 