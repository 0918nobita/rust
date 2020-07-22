mod list;

pub mod mybox;

#[macro_use]
pub mod list_rc;

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    #[allow(unused_variables)]
    fn ref_cell() {
        let a = RefCell::new(1);
        let b = a.borrow_mut();
        let c = a.borrow_mut();
    }
}
