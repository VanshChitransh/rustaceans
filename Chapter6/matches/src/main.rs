// rust warns you when you have a un-initialized enum varient in your code. 
// to stop that warning use 

// #[allow(dead_code)]
// enum Coin{
//     Penny,
//     Nickel, 
//     Dime, 
//     Quarter
// }

// fn main(){
//     // let coin_for_fun = Coin::Penny;
//     // let coin_for_fun = Coin::Nickel;
//     // let coin_for_fun = Coin::Dime;
//     let coin_for_fun = Coin::Quarter;
//     let result = value_in_coin(&coin_for_fun);
//     println!("{}", result);
// }

// fn value_in_coin(coin: &Coin) -> u8{
//     match coin{
//         Coin::Penny => 1, 
//         Coin::Nickel => 5, 
//         Coin::Dime => 10, 
//         Coin::Quarter => 25,
//     }
// }




// Patterns that bind to values 
// #[derive(Debug)]
// #[allow(dead_code)]
// enum IndState{
//     UttarPradesh, 
//     Rajasthan, 
//     UttraKhand,
//     Himachal
// }

// #[allow(dead_code)]
// enum Coin{
//     Penny, 
//     Nickel, 
//     Dime, 
//     Quarter(IndState),
// }

// fn main(){
//     let coin_for_fun = Coin::Quarter(IndState::UttarPradesh);
//     let result = match_coin_to_value(coin_for_fun);
//     println!("{}", result);
// }

// fn match_coin_to_value(coin: Coin) -> u8 {
//     match coin{
//         Coin::Penny => 1, 
//         Coin::Nickel => 5, 
//         Coin::Dime => 10, 
//         Coin::Quarter(state) => {
//             println!("You got a Quarter coin from {:?}", state);
//             25
//         }
//     }
// }




fn plus_one_to_it(x: Option<i32>) -> Option<i32> {
    match x{
        None => None, 
        Some(i) => Some(i+1),
    }
}

fn main(){
    let five = Some(5);
    let six = plus_one_to_it(five);
    let seven = plus_one_to_it(None);

    println!("{:#?}", five);
    println!("{:#?}", six);
    println!("{:#?}", seven);
}
