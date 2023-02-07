use traittest::{Summary, Tweet,NewsArticle};

fn main() {
	
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

	let newsarticle = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

   println!("1 new tweet====>>: {}", tweet.summarize());
   println!("1 new newsarticle====>>: {}", newsarticle.summarize());
}
