fn vectors() {
    println!("=====VECTORS=====");
    // Using vec! macro to predefine vector
    let v = vec![1, 2, 3];

    // Read using indexing (will panic if out-of-bounds)
    let third = &v[2];
    println!("The third element is {third}");

    // Read using get
    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterate through vector
    for i in &v {
        println!("{i}")
    }

    // Mutable iteration
    let mut v = vec![100, 200, 300];
    for i in &mut v {
        *i += 50; // Dereference
    }
    dbg!(v);

    // Updating vector
    let mut v = Vec::new();
    v.push(5);
}

fn strings() {
    println!("\n=====STRINGS=====");
    // Defining string
    let data = "initial contents"; // &str
    let s = data.to_string(); // String
    println!("{s}");
    let s = String::from("initial contents");
    println!("{s}");

    // Appending to string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // Adding single character to string
    let mut s = "lo".to_string();
    s.push('l');
    println!("{s}");

    // Concatenate with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s}");

    // Using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Slicing strings
    let hello = "こんにちは";
    let s = &hello[0..6]; // Will panic if we try to slice e.g. 0..5 because Japanese characters take up 3 bytes
    println!("{s}");

    // Iterating over strings
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
}

fn hashmaps() {
    println!("\n=====HASHMAPS=====");
    use std::collections::HashMap;

    // Initialize hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // Get by key
    let score = scores.get(&String::from("blue")).copied().unwrap_or(0);

    // Iterate
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}", field_name);

    // Overwrite
    scores.insert(String::from("blue"), 25);
    println!("{:?}", scores);

    // Add only if key doesn't exist
    scores.entry(String::from("green")).or_insert(60);
    scores.entry(String::from("blue")).or_insert(60);
    println!("{:?}", scores);

    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // Dereference
    }
    println!("{:?}", map)
}

fn main() {
    vectors();
    strings();
    hashmaps();
}
