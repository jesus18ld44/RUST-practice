use traits_summarize::foo;
use foo::Summary;

fn main() {
    let tw1 = foo::Tweet {
        username: String::from("horse_ebook"),
        content: String::from(
            "of course, as you probably already know"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tw1.summarize());
    println!("{}", tw1.summarize_author());
    println!("len tw1: {}", tw1.content.len());

}