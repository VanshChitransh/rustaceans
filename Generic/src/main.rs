// fn main() {
//     println!("Hello, world!");
//     let vec_num = vec![12, 34, 54, 64, 46];
//     let mut largest = &vec_num[0];
//     for i in &vec_num{
//         if i > largest{
//             largest = i;
//         }
//     }
//     println!("This is the largest number from the array -> {}", largest);
//     println!("Now this will be the next array");

//     let vec_num_2 = vec![12, 34, 54, 64, 46];
//     let mut second_largest = &vec_num_2[0];
//     let mut largest_t = &vec_num_2[0];

//     for i in &vec_num_2{
//         if i > largest_t{
//             second_largest = largest_t;
//             largest_t = i;
//         }
//         else if i > second_largest && i != largest_t{
//             second_largest = i;
//         }
//     }
//     println!("This is the second largest -> {}", second_largest);
// }


// fn largest(list: &[i32]) -> &i32{
//     let mut largest = &list[0];
//     for i in list{
//         if i > largest{
//             largest = i;
//         }
//     }
//     largest
// }

// fn main(){
//     let vec_my = vec![1,2,3,4,5];
//     let vec_largest_number = largest(&vec_my);
//     println!("This is the largest number -> {}", vec_largest_number);
//     let vec_my_2 = vec![43,23,51,32,54,01,322];
//     let vec_largest_number_2 = largest(&vec_my_2);
//     println!("This is the second largest number -> {}", vec_largest_number_2);

// }




// fn largest<T> (list: &[T]) -> &T{
//     let mut largest = &list[0];

//     for i in list{
//         if i > largest{
//             largest = i;
//         }
//     }
//     largest
// } 

// fn main(){
//     let vec_1 = vec![1,2,3,4,5];
//     let vec_1_res = largest(&vec_1);

//     let vec_2 = vec!['a', 'b', 'c', 'd', 'e'];
//     let vec_2_res = largest(&vec_2);

//     println!("This is the largest of the vector -> {}", vec_1_res);
//     println!("This is the largest char of the vector -> {}", vec_2_res);
// }






#[derive(Debug)]
struct Point<T>{
    x: T, 
    y: T,
}

// fn main(){
//     let integer = Point {x: 5, y: 5};
//     let float = Point {x: 4.0, y: 2.0};
//     println!("{:?}", integer);
//     println!("{:?}", float);
// }

// we can even use two generics for two different variables. 

#[derive(Debug)]
struct Point2<T, U>{
    x: T, 
    y: U,
}

fn main(){
    let integer = Point2{x: 5, y: 4.0};
    let float = Point2{x: 2.3, y: 4};
    println!("{:?}", integer);
    println!("{:?}", float);
}




