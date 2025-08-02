// #![allow(dead_code, unused_variables)]

// // Stage 2
// pub struct Credentials{
//     // making the fields public.
//     pub username: String, 
//     pub password: String,
//     // Stage 3
// }

// enum Status{
//     Connected, 
//     Interupted,
// }

// fn connect_to_database() -> Status{
//     Status::Connected
// }

// fn get_user(){

// }

// fn login(cred: Credentials){
//     get_user()
// }

// pub fn authenticate(cred: Credentials){
//     if let Status::Connected = connect_to_database(){}
// }
// // Stage 1


// If i want to use authenticate function then i have to make it public, but it's taking a struct of type Credential as input, so it also need to be public. 
// Hence we made both the fnc and struct public. 








// Now our code is not structured, we will structure it using Modules. 



mod database{
    pub enum Status{
        Connected, 
        Interupted,
    }

    pub fn connect_to_database() -> Status{
        Status::Connected
    }

    pub fn get_user(){

    }
}

pub mod auth_utils{
    pub fn login(cred: models::Credentials){
        // database::get_user() -> This will cause error, as the login fn will try to find database in it's scope itself, but there is not database in it 
        // We have to ways to handle this. 
        create::database::get_user()
        super::database::get_user()
        // both are the valid ways to call it. 
    }

    // we can even create modules inside modules. 

    mod models{
        pub struct Credentials{
            pub username: String, 
            pub password: String, 
        }
    }
}


pub fn authenticate(cred: auth_utils::models::Credentials){
    if let database::Status::Connected = database::connect_to_database(){
        auth_utils::login(cred);
    }
}



