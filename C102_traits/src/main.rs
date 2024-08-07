pub mod aggregator;
use crate::aggregator::{Desc, NewsArticle, NewsArticle_default, Summary, Tweet};
use std::fmt::Debug;
use std::fmt::Display;

// Traits as Parameters
// type that implements the Summary trait
pub fn notify(item: &(impl Summary + Desc)) {
    println!("Breaking news! {}", item.summarize());
    println!("Count: {}", item.count());
}

// which is equivalent to the following
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    return 1;
}
fn some_function1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 1;
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle_default {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article)
}
