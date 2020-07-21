#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! count_args {
    () => (0usize);
    ( $x:expr ) => (1usize);
    ( $x:expr , $($xs:expr),* ) => (1usize + count_args!($($xs),*));
}

#[macro_export]
macro_rules! assert_impl {
    ( $expr:expr , $trait:path $(, $type:tt)* ) => {
        {
            fn eval<T: $trait, $($type),*>(_: T) {}
            eval($expr);
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn use_macro() {
        assert!(count!() == 0);
        assert!(count!(,) == 1);
        assert!(count!(,,) == 2);
        assert!(count!(,,,) == 3);
    }

    #[test]
    fn use_count_args_macro() {
        assert!(count_args!() == 0);
        assert!(count_args!(1) == 1);
        assert!(count_args!(1, 2, 3, 4, 5) == 5);
    }

    #[derive(Clone, Copy)]
    struct Data(i32);

    #[test]
    fn use_assert_impl() {
        assert_impl!(100, PartialOrd);

        let data = Data(42);
        assert_impl!(data, Clone);
        assert_impl!(data, Copy);
        // assert_impl!(Data(42), PartialOrd); // PartialOrd is not implemented for Data

        let mut data = Data(-1);
        let update = || {
            data.0 += 1;
            data
        };
        // assert_impl!(update, Fn() -> A, A); // closure doesn't implement Fn
        assert_impl!(update, FnMut() -> A, A);
    }
}
