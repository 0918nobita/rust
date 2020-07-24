use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
enum ListRc<T> {
    Cons(T, Rc<ListRc<T>>),
    Nil,
}

#[macro_export]
macro_rules! list_rc {
    () => (std::rc::Rc::new(ListRc::Nil));
    ( $x:expr $(, $xs:expr)* ) => (std::rc::Rc::new(ListRc::Cons($x, list_rc!($($xs),*))));
}

struct ListRcIterator<'a, T> {
    list: &'a ListRc<T>,
}

impl<'a, T> Iterator for ListRcIterator<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            ListRc::Cons(car_ref, cdr) => {
                let car = Some(car_ref.clone());
                self.list = cdr;
                car
            }
            ListRc::Nil => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a ListRc<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = ListRcIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListRcIterator { list: self }
    }
}

#[cfg(test)]
mod test {
    use super::{
        ListRc::{self, Cons},
        ListRcIterator,
    };
    use std::rc::Rc;

    #[test]
    fn list_rc_iter() {
        let mut iter = ListRcIterator {
            list: &list_rc![1, 2, 3],
        };
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn list_rc_into_iter() {
        assert_eq!(
            Vec::<i32>::new(),
            ListRc::<i32>::Nil.into_iter().collect::<Vec<i32>>()
        );

        let list = list_rc![4, 5, 6];
        let vec: Vec<i32> = list.into_iter().collect();
        assert_eq!(vec, vec![4, 5, 6]);
    }

    #[test]
    fn ref_count() {
        let a = list_rc![5, 10];
        assert_eq!(format!("{:?}", a), "Cons(5, Cons(10, Nil))");
        assert_eq!(Rc::strong_count(&a), 1);

        #[allow(unused_variables)]
        let b = Cons(3, Rc::clone(&a));
        assert_eq!(format!("{:?}", b), "Cons(3, Cons(5, Cons(10, Nil)))");
        assert_eq!(Rc::strong_count(&a), 2);

        {
            #[allow(unused_variables)]
            let c = Cons(4, Rc::clone(&a));
            assert_eq!(format!("{:?}", c), "Cons(4, Cons(5, Cons(10, Nil)))");
            assert_eq!(Rc::strong_count(&a), 3);
        }

        assert_eq!(Rc::strong_count(&a), 2);
    }
}
