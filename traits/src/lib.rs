pub trait Summary{
    fn summarize(&self) -> String;
}

pub trait Dummy{
    fn summarize(&self) -> String{
        String::from("(Read more..)")
    }
}

pub struct NewsArticle{
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost{
    pub username: String, 
    pub content: String, 
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}


pub struct DummaryStruct{
    pub name: String, 
    pub branch: String, 
    pub room_no: u8, 
    pub mess_opt_in: bool,
}

impl Dummy for DummaryStruct{
    // we used the default method and now we will be overriding it. 
    fn summarize(&self) -> String{
        format!("Name of student: {}, Branch of student: {}, room_no of student: {}, student eat's from: {}", self.name, self.branch, self.room_no, self.mess_opt_in)
    }
}