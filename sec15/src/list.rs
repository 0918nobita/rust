#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
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
    fn cons_list() {
        let nil: List<i32> = list![];
        let ns = list![1, 2, 3];
        assert_eq!(nil, Nil);
        assert_eq!(
            ns,
            Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
        )
    }
}
