#![allow(dead_code, unused_variables)]

// Stage 2
pub struct Credentials{
    // making the fields public.
    pub username: String, 
    pub password: String,
    // Stage 3
}

enum Status{
    Connected, 
    Interupted,
}

fn connect_to_database() -> Status{
    Status::Connected
}

fn get_user(){

}

fn login(cred: Credentials){
    get_user()
}

pub fn authenticate(cred: Credentials){
    if let Status::Connected = connect_to_database(){}
}
// Stage 1


// If i want to use authenticate function then i have to make it public, but it's taking a struct of type Credential as input, so it also need to be public. 
// Hence we made both the fnc and struct public. 