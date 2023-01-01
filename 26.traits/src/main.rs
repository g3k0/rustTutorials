fn main() {
    /**
     * A trait defines functionality a particular type has and can share 
     * with other types. We can use traits to define shared behavior in 
     * an abstract way. We can use trait bounds to specify that a generic 
     * type can be any type that has certain behavior.
     * Note: Traits are similar to a feature often called interfaces in 
     * other languages, although with some differences.
     */

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

    //--------------
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /**
     * Here is how trait is different form interfaces in other programming languages:
     * traits can have a method implementation, that is a deafult behavior for all the 
     * structs will implement the trait
     */
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // This code prints New article available! (Read more...).
    println!("New article available! {}", article.summarize());

    // ---------------------------------
    pub trait Summary {
        fn summarize_author(&self) -> String;
    
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // -----------------------
    /**
     * Traits as parameters
     */
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // The impl Trait syntax works for straightforward cases but is 
    // actually syntax sugar for a longer form known as a trait bound; 
    // it looks like this:
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // where clauses
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }

    // we can rewrite in this way:
    fn some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        { }

    //---------------------
    /**
     * Returning types that implement traits
     */
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    // However, you can only use impl Trait if you’re returning a single type

    // Using Trait Bounds to Conditionally Implement Methods
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    /**
     * For example, the standard library implements the ToString 
     * trait on any type that implements the Display trait
     */
    impl<T: Display> ToString for T {
        // --snip--
    }
}
