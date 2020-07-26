#[cfg(test)]
mod test {
    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
            )
        }
    }

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    #[test]
    fn raw_pointer() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            assert_eq!(*r1, 5); // => 5
            assert_eq!(*r2, 5); // => 5

            *r2 = 6;

            assert_eq!(*r1, 6); // => 6
            assert_eq!(*r2, 6); // => 6
        }
    }

    #[test]
    fn my_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn my_abs() {
        assert_eq!(unsafe { abs(-3) }, 3);
    }

    #[test]
    fn counter() {
        assert_eq!(unsafe { COUNTER }, 0);
        add_to_count(3);
        assert_eq!(unsafe { COUNTER }, 3);
    }
}
