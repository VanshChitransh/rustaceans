fn main(){
    // let condition = false;
    // let x = if condition {5} else {6};
    // println!("This is the value of x, {}", x);

    // loop{
    //     println!{"Hare krishna"};
    // }

    // let mut counter = 0;
    // let counter = loop {
    //     counter += 1;
    //     if counter == 10{
    //         break counter*2;
    //     }
    // };
    
    // let result = loop {
    //     counter += 1;
    //     if counter == 10{
    //         break counter*2;
    //     }
    // };
    // println!("This is the final value of counter, {}", result);

    // let mut count = 0; 
    // 'counting_up: loop{
    //     println!("Count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count, {}", count);


    let mut x = 5; 
    while x != 0{
        println!("{x}");
        x -= 1;
    }
    println!();

    println!("");

    let a:[u32;5] = [1,2,3,4,5];
    let mut index = 0;
    while index < 5 {
        println!("This is the value of array's {}th element, {}", index, a[index]);
        index+=1;
    }

    println!();
    println!();

    for element in a {
        println!("The value is: {}", element);
    }

    println!();
    println!();

    for element in (0..6).rev(){
        println!("The value is: {}", element);
    }
}
