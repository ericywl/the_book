fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // First elision rule
    fn level(&self) -> i32 {
        3
    }

    // Third elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // Valid
    {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    // Not valid
    // {
    //     let string1 = String::from("long string is long");
    //     let result;
    //     {
    //         let string2 = String::from("xyz");
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     println!("The longest string is {}", result);
    // }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        // Static lifetime, lives through the whole program
        let s: &'static str = "I have a static lifetime.";
    }
}
