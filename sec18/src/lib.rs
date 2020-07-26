#[cfg(test)]
mod test {
    #[test]
    fn multi_patterns() {
        fn inner<'a>(x: i32) -> &'a str {
            match x {
                1 | 2 => "A",
                3 => "B",
                _ => "C",
            }
        }

        assert_eq!("A", inner(1));
        assert_eq!("A", inner(2));
        assert_eq!("B", inner(3));
        assert_eq!("C", inner(5));
    }

    #[test]
    fn num_range_pattern() {
        fn inner<'a>(x: i32) -> &'a str {
            match x {
                1..=5 => "one through five",
                _ => "something else",
            }
        }

        let msg = "one through five";
        assert_eq!(msg, inner(1));
        assert_eq!(msg, inner(2));
        assert_eq!(msg, inner(3));
        assert_eq!(msg, inner(4));
        assert_eq!(msg, inner(5));
        assert_eq!("something else", inner(7));
    }

    #[test]
    fn char_range_pattern() {
        fn inner<'a>(c: char) -> &'a str {
            match c {
                'a'..='j' => "early ASCII letter",
                'k'..='z' => "late ASCII letter",
                _ => "something else",
            }
        }

        let early = "early ASCII letter";
        let late = "late ASCII letter";
        assert_eq!(early, inner('d'));
        assert_eq!(early, inner('i'));
        assert_eq!(late, inner('m'));
        assert_eq!(late, inner('z'));
        assert_eq!("something else", inner('!'));
    }

    struct Point {
        x: i32,
        y: i32,
    }

    use rand::Rng;

    #[test]
    fn destructure_structs() {
        let mut rng = rand::thread_rng();
        let x_val = rng.gen();
        let y_val = rng.gen();
        let p = Point { x: x_val, y: y_val };
        let Point { x, y } = p;
        assert_eq!(x, x_val);
        assert_eq!(y, y_val);
    }

    #[test]
    fn destructure_refs() {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];
        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        assert_eq!(sum_of_squares, 135); // 0 + 26 + 109
    }

    #[test]
    fn ignore_rest() {
        let mut rng = rand::thread_rng();
        let x_val: i32 = rng.gen();
        let y_val: i32 = rng.gen();
        let p = Point { x: x_val, y: y_val };
        let Point { x, .. } = p;
        assert_eq!(x, x_val);
        let Point { y, .. } = p;
        assert_eq!(y, y_val);
    }

    #[test]
    fn ref_mut_pattern() {
        let mut msg = Some(String::from("Hello"));
        match msg {
            Some(ref mut text) => (*text).push_str(", world!"),
            None => (),
        }
        assert_eq!("Hello, world!", msg.unwrap().as_str())
    }

    #[test]
    fn subpatterns() {
        fn inner<'a>(x: Option<i32>) -> String {
            match x {
                Some(num @ 1..=3) => format!("[{}]", num),
                Some(_) => String::from("B"),
                None => String::from("C"),
            }
        }

        assert_eq!("[1]", inner(Some(1)));
        assert_eq!("[2]", inner(Some(2)));
        assert_eq!("[3]", inner(Some(3)));
        assert_eq!("B", inner(Some(5)));
        assert_eq!("C", inner(None));
    }
}
