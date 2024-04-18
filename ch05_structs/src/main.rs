#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someemail@example.com"),
        String::from("someusername123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Invalid because user1.username has been moved to user2.username since it is a String
    // println!("{}", user1.username);
    println!("{:?}", user2);
}
