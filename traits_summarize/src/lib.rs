pub mod foo{

    pub trait Summary {
        fn summarize(&self) -> String;
        // each type implementing this trait must provide
        // its own custom behavior for the body of the method
        
        fn summarize_rm(&self) -> String;

        fn summarize_author(&self) -> String {
            format!("Read more ({})", self.summarize_rm())
        }
        
        fn get_tweet() -> Tweet {
            Tweet {
                username: String::from("horse_ebook"),
                content: String::from(
                    "of course, as you probably already know"
                ),
                reply: false,
                retweet: false,
            }
        } 
    
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

        fn summarize_rm(&self) -> String {
            format!("{}", &self.content[..15])
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize (&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        fn summarize_rm(&self) -> String {
            format!("{}", &self.content[..15])
        }
    }


    // This parameter accepts any type that implements the specified
    // trait
    // In the body we can call any methods on item that come from the Summary
    // trait, such as summarize
    
    // pub fn notify (item: &impl Summary) {
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking! {}", item.summarize());
    }


    // we can specify more than one trait bound
    // pub fn notify(item: &(impl Summary + Display));
    // pub fn notify<T: Summary + Display>(item: &T);
    // pub fn some_function<T, U>(t: &T, u: &U) -> i32
            // where T: Display + Clone,
            //       U: Clone + Debug,

}