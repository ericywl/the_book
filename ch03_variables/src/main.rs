fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    countdown();
}

fn countdown() {
    for i in (1..4).rev() {
        println!("{i}")
    }
    println!("LIFTOFF!")
}
