#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 area is {:?}", rect1.area());

    let rect2 = Rectangle::square(20);
    println!("rect2 is {:?}", rect2);
    println!("rect2 area is {:?}", rect2.area());

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
