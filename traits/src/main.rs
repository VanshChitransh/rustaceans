use traits::{SocialPost, Summary, DummaryStruct, Dummy};


fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    println!();

    let dummy = DummaryStruct{
        name: String::from("Vansh"),
        branch: String::from("CS"),
        // room_no: 264, 
        room_no: 226,
        mess_opt_in: false,
    };
    println!("Information about {} -> {}", dummy.name, dummy.summarize());
}




// fn some_name<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32{

// }



// fn some_name<T,U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone 
//     U: Clone + Debug
// {

// }