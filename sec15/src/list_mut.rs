use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub enum ListMut<T: fmt::Debug> {
    Cons(Rc<RefCell<T>>, Rc<ListMut<T>>),
    Nil,
}

impl<T> fmt::Debug for ListMut<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMut::Cons(ref car, ref cdr) => {
                let car = (*car).borrow();
                let cdr = &*(*cdr);
                write!(f, "Cons({:?}, {:?})", car, cdr)
            }
            ListMut::Nil => write!(f, "Nil"),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn list_mut() -> Result<(), Box<dyn std::error::Error>> {
        use super::ListMut::{Cons, Nil};
        use indoc::indoc;
        use std::cell::RefCell;
        use std::io::Write;
        use std::rc::Rc;

        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        let mut buf = Vec::<u8>::new();
        writeln!(&mut buf, "a after = {:?}", a)?;
        writeln!(&mut buf, "b after = {:?}", b)?;
        writeln!(&mut buf, "c after = {:?}", c)?;

        let output = String::from_utf8(buf)?;
        assert_eq!(
            output,
            indoc!(
                "
                a after = Cons(15, Nil)
                b after = Cons(6, Cons(15, Nil))
                c after = Cons(10, Cons(15, Nil))
            "
            )
        );

        Ok(())
    }
}
