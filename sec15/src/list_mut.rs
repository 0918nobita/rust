use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub enum ListMut<T> {
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

pub struct ListMutIterator<'a, T> {
    list: &'a ListMut<T>,
}

impl<'a, T> Iterator for ListMutIterator<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            ListMut::Cons(car_ref, cdr) => {
                let car = Some(car_ref.borrow().clone());
                self.list = cdr;
                car
            }
            ListMut::Nil => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a ListMut<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = ListMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListMutIterator { list: self }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn list_mut() -> Result<(), Box<dyn std::error::Error>> {
        use super::ListMut::{Cons, Nil};
        use std::cell::RefCell;
        use std::io::Write;
        use std::rc::Rc;

        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        let mut buf = Vec::<u8>::new();
        write!(&mut buf, "b after = {:?}", b)?;
        let output = String::from_utf8(buf)?;
        assert_eq!(output, "b after = Cons(6, Cons(15, Nil))");

        let vec: Vec<i32> = c.into_iter().collect();
        assert_eq!(vec, vec![10, 15]);
        Ok(())
    }
}
