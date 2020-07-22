use std::fmt::Debug;
use std::ops::Deref;

pub struct MyBox<T: Debug>(T);

impl<T> MyBox<T>
where
    T: Debug,
{
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Debug,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("MyBox({:?}) dropped", self.0);
    }
}

#[cfg(test)]
mod test {
    use super::MyBox;

    #[test]
    fn use_mybox() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);

        // deref coercion
        fn func(_: &str) {}
        let m = MyBox::new(String::from("hello"));
        func(&m);
    }
}
