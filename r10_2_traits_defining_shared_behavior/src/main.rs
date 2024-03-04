use std::fmt::{Debug, Display};

use aggregator::{Summary, Tweet, NewsArticle};

pub mod aggregator;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

pub fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// Unable to return two types
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
