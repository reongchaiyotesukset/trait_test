/* 
pub trait Summary {
       
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/
pub trait  Summary{
    fn test(&self) -> String;
    fn test2();
}

pub struct  Tweet{
    pub username : String,
    pub content: String,
}

impl Summary for Tweet  {
    fn test(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn test2()
    {
        println!("test");
    }
}
//testing
pub struct  abc{
    pub username : String,
    pub lastname :String,
}
impl abc {
    
    pub fn  abc_1() {
         println!("abc_1");
    }
}