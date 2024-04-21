use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    {
        use crate::List::{Cons, Nil};

        let b = Box::new(5);
        println!("b = {}", b);

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));
        // Deref coercion
        hello(&m);
        // hello(&(*m)[..]);
    }

    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
