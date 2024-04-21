mod aggregator;

use std::fmt::{Debug, Display};

use aggregator::{NewsArticle, Summary, Tweet};

// Trait as parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound
fn notify_t<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with where clauses
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    let t = t.clone();
    let u = u.clone();
    println!("{}", t);
    println!("{:?}", u);
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditional implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
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

    {
        let t1 = returns_summarizable();
        let t2 = returns_summarizable();

        let p = Pair::new(1, 2);
        p.cmp_display();

        let p = Pair::new(t1, t2);
        // Doesn't work because impl Summary does not implement Display + PartialOrd
        // p.cmp_display();
    }
}
