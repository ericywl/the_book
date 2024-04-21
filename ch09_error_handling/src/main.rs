use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let greet_file_result = File::open("hello.txt");

    let greet_file = match greet_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };

    let greet_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}

struct OurError {
    e: io::Error,
}

impl From<io::Error> for OurError {
    fn from(value: io::Error) -> Self {
        Self { e: value }
    }
}

fn read_username_from_file() -> Result<String, OurError> {
    // Propagate error with ?
    // ? will call From trait method above to convert io::Error to OurError
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // Can just use library provided function also (with some adjustments)
    // fs::read_to_string("hello.txt")
}
