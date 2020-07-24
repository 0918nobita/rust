use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

pub struct MyBox<'a, T: Display> {
    content: T,
    logger: Option<&'a Rc<RefCell<Vec<u8>>>>,
}

impl<'a, T> MyBox<'a, T>
where
    T: Display,
{
    pub fn new(x: T, logger: Option<&'a Rc<RefCell<Vec<u8>>>>) -> Self {
        MyBox { content: x, logger }
    }
}

impl<'a, T> Deref for MyBox<'a, T>
where
    T: Display,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.content
    }
}

impl<'a, T> Drop for MyBox<'a, T>
where
    T: Display,
{
    fn drop(&mut self) {
        if let Some(logger) = self.logger {
            write!(logger.borrow_mut(), "{}", self.content).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::MyBox;
    use std::cell::RefCell;
    use std::io::{self, Write};
    use std::rc::Rc;

    #[test]
    fn use_mybox() {
        let x = 5;
        let y = MyBox::new(x, None);
        assert_eq!(5, x);
        assert_eq!(5, *y);

        // deref coercion
        fn func(_: &str) {}
        let m = MyBox::new(String::from("hello"), None);
        func(&m);
    }

    #[test]
    fn drop_mybox() -> io::Result<()> {
        let buf = &Rc::new(RefCell::new(Vec::<u8>::new()));

        fn inner(buf: &Rc<RefCell<Vec<u8>>>) -> io::Result<()> {
            write!(buf.borrow_mut(), "A")?;

            #[allow(unused_variables)]
            let mybox_a = MyBox::new("B", Some(buf));

            let mybox_b = MyBox::new("C", Some(buf));

            let _ = MyBox::new("D", Some(buf));
            MyBox::new("E", Some(buf));

            drop(mybox_b);

            write!(buf.borrow_mut(), "F")?;
            Ok(())
        }

        inner(buf)?;

        assert_eq!(
            "ADECFB",
            String::from_utf8(buf.borrow().clone()).unwrap()
        );
        Ok(())
    }
}
