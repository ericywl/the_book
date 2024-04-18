#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));

        println!("{:?}", home);
        println!("{:?}", loopback);

        if let IpAddr::V6(s) = loopback {
            println!("ip v6 value: {}", s)
        }
    }

    {
        let dice_roll = 9;
        match dice_roll {
            7 => println!("lucky 7"),
            _ => println!("reroll"),
        }
    }
}
