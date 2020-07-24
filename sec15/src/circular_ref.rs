use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

pub fn circular_ref() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial ref count: {}", Rc::strong_count(&a)); // 1
    println!("a next item: {:?}", a.tail().unwrap()); // RefCell { value: Nil }

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a ref count after b creation: {}", Rc::strong_count(&a)); // 2
    println!("b initial ref count: {}", Rc::strong_count(&b)); // 1
    println!("b next item: {:?}", b.tail().unwrap()); // RefCell { value: Cons(5, RefCell { value: Nil }) }

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count after changing: {}", Rc::strong_count(&b)); // 2
    println!("a ref count after changing: {}", Rc::strong_count(&a)); // 2

    // println!("{:?}", a.tail()); // stack overflow
}
