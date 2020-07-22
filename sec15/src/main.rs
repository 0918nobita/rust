use std::rc::Rc;

extern crate sec15;
use sec15::{list_rc, mybox::MyBox};

fn main() {
    drop_mybox();
    ref_count();
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
