extern crate sec15;
use sec15::mybox::MyBox;

fn main() {
    println!("begin");

    #[allow(unused_variables)]
    let mybox_a = MyBox::new("A");

    let mybox_b = MyBox::new("B");

    let _ = MyBox::new("C");
    MyBox::new("D");

    drop(mybox_b);

    println!("end");
}
