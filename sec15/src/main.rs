use std::cell::RefCell;
use std::rc::Rc;

extern crate sec15;
use sec15::{list_rc, mybox::MyBox};

fn main() {
    drop_mybox();
    ref_count();
    circular_ref();
}

fn drop_mybox() {
    println!("begin");

    #[allow(unused_variables)]
    let mybox_a = MyBox::new("A");

    let mybox_b = MyBox::new("B");

    let _ = MyBox::new("C");
    MyBox::new("D");

    drop(mybox_b);

    println!("end");
}

fn ref_count() {
    use list_rc::ListRc::{self, Cons};

    let a = list_rc![5, 10];
    println!("a: {:?}", a);
    println!("Ref count: {}", Rc::strong_count(&a));

    #[allow(unused_variables)]
    let b = Cons(3, Rc::clone(&a));
    println!("b: {:?}", b);
    println!("Ref count: {}", Rc::strong_count(&a));

    {
        #[allow(unused_variables)]
        let c = Cons(4, Rc::clone(&a));
        println!("c: {:?}", c);
        println!("Ref count: {}", Rc::strong_count(&a));
    }

    println!("Ref count: {}", Rc::strong_count(&a));
}

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

fn circular_ref() {
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
