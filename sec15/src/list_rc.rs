use std::rc::Rc;

#[derive(Debug)]
pub enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

#[macro_export]
macro_rules! list_rc {
    () => (std::rc::Rc::new(ListRc::Nil));
    ( $x:expr $(, $xs:expr)* ) => (std::rc::Rc::new(ListRc::Cons($x, list_rc!($($xs),*))));
}
