#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub struct ListIterator<'a, T: Clone> {
    list: &'a List<T>,
}

impl<'a, T> Iterator for ListIterator<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            List::Cons(ref car_ref, cdr) => {
                let car = Some(car_ref.clone());
                self.list = cdr;
                car
            }
            List::Nil => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = ListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator { list: self }
    }
}

#[macro_export]
macro_rules! list {
    () => (List::Nil);
    ( $x:expr $(, $xs:expr)* ) => (List::Cons($x, Box::new(list!($($xs),*))));
}

#[cfg(test)]
mod test {
    use super::List::{self, Cons, Nil};

    #[test]
    fn use_list_macro() {
        let nil: List<i32> = list![];
        let ns = list![1, 2, 3];
        assert_eq!(nil, Nil);
        assert_eq!(
            ns,
            Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
        )
    }

    #[test]
    fn use_list_iter() {
        assert_eq!(Vec::<i32>::new(), list![].into_iter().collect::<Vec<i32>>());
        assert_eq!(
            vec![1, 2, 3],
            list![1, 2, 3].into_iter().collect::<Vec<i32>>()
        );
    }
}
