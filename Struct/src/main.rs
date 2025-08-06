struct User{
    first_name: String, 
    last_name: String, 
    email: String, 
    id: u32, 
    phone_number: u64,
    age: u8, 
}

struct Student{
    name: String, 
    college_name: String, 
    branch_name: String, 
    year: i8, 
    floor_number: i8,
}

struct Student2{
    name: String, 
    year: i8, 
    floor_number: i8,
}

fn main() {
    // println!("Hello, world!");
    let user1 = User{
        first_name: "Vansh".to_string(),
        last_name: "Chitransh".to_string(),
        email: "VanshChitransh32@gmail.com".to_string(),
        id: 10015,
        phone_number: 7668522358, 
        age: 20
    };

    println!("first_name -> {}", user1.first_name);
    println!("last_name -> {}", user1.last_name);
    println!("email -> {}", user1.email);
    println!("id -> {}", user1.id);
    println!("phone_number -> {}",user1.phone_number);
    println!("age -> {}", user1.age);


    let vansh = Student{
        name: "Vansh".to_string(),
        college_name: "Scaler".to_string(),
        branch_name: "CS".to_string(),
        year: 2, 
        floor_number: 2
    };

    // println!("Name of the student is -> {}", vansh.name);
    // println!("Name of college is  -> {}", vansh.college_name);
    // println!("branch is -> {}", vansh.branch_name);
    // println!("student of {}nd year", vansh.year);
    // println!("they live on {}nd floor", vansh.floor_number);
    // println!();

    let sid = Student{
        name: "Siddhanth".to_string(),
        ..vansh
    };

    // println!("Name of the student is -> {}", sid.name);
    // println!("Name of college is  -> {}", sid.college_name);
    // println!("branch is -> {}", sid.branch_name);
    // println!("student of {}nd year", sid.year);
    // println!("they live on {}nd floor", sid.floor_number);
    // println!();


    let tan = Student{
        name: "Tanish".to_string(),
        ..sid
    };

    // println!("Name of the student is -> {}", tan.name);
    // println!("Name of college is  -> {}", tan.college_name);
    // println!("branch is -> {}", tan.branch_name);
    // println!("student of {}nd year", tan.year);
    // println!("they live on {}nd floor", tan.floor_number);
    // println!();

    let bihari = Student{
        name: "Bihari".to_string(),
        ..tan
    };

    println!();
    println!("Name of the student is -> {}", bihari.name);
    println!("Name of college is  -> {}", bihari.college_name);
    println!("branch is -> {}", bihari.branch_name);
    println!("student of {}nd year", bihari.year);
    println!("they live on {}nd floor", bihari.floor_number);
    println!();


    // try running the code...
    // what happened? 
    // ownership rule!???
    // String is stored on Heap and does not have .clone() so it will be moved. 




    let Vansh = Student2{
        name: "Vansh".to_string(),
        year: 2, 
        floor_number: 2,
    };

    println!("Name of the student is -> {}", Vansh.name);
    println!("student of {}nd year", Vansh.year);
    println!("they live on {}nd floor", Vansh.floor_number);
    println!();

    let Sid = Student2{
        name: "Siddhanth".to_string(),
        ..Vansh
    };

    println!("Name of the student is -> {}", Sid.name);
    println!("student of {}nd year", Sid.year);
    println!("they live on {}nd floor", Sid.floor_number);
    println!();

    let Tan = Student2{
        name: "Tanish".to_string(),
        ..Vansh
    };

    println!("Name of the student is -> {}", Tan.name);
    println!("student of {}nd year", Tan.year);
    println!("they live on {}nd floor", Tan.floor_number);
    println!();

    let Bihari = Student2{
        name: "Bihari".to_string(),
        ..Vansh
    };

    println!("Name of the student is -> {}", Bihari.name);
    println!("student of {}nd year", Bihari.year);
    println!("they live on {}nd floor", Bihari.floor_number);
    println!();

}


// fun example!!



// we can decalre struct and re use them, but be carefull for differnt data types. 
// data types like String are stored on heap and does not follow .clone() hence they will cause issue. 


// Very imp line below!!!
//Each struct you define is its own type, even though the fields within the struct might have the same types. 
