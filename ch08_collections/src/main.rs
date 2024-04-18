fn vectors() {
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

fn main() {
    vectors();
    strings();
}
