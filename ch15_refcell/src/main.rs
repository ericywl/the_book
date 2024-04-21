#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum AnotherList {
    Cons(i32, RefCell<Rc<AnotherList>>),
    Nil,
}

impl AnotherList {
    fn tail(&self) -> Option<&RefCell<Rc<AnotherList>>> {
        match self {
            AnotherList::Cons(_, item) => Some(item),
            AnotherList::Nil => None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

        let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
        println!("");
    }

    {
        let a = Rc::new(AnotherList::Cons(
            5,
            RefCell::new(Rc::new(AnotherList::Nil)),
        ));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(AnotherList::Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        // Point tail of a to b instead of Nil, creating a cycle
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }
}
