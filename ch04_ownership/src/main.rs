fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{}", s); // This will print `hello, world!`
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // deep copy

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        println!("The length of {} is {}", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s); // can only create one simultaneous mutable reference
    }

    {
        let s = String::from("hello world");
        let word = first_word(&s);
        println!("{}", word);

        let word = first_word("hello world");
        println!("{}", word);
    }
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    // some_string is a mutable reference to a String
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
