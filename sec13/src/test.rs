use super::cacher::Cacher;

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _ = c.value(1);
    let v = c.value(2);
    assert_eq!(v, 2)
}

#[test]
fn memorize_other_function() {
    let mut c = Cacher::new(|s: &str| s.len());
    let _ = c.value("ab");
    let v = c.value("xyz");
    assert_eq!(v, 3)
}

#[derive(Clone, Copy)]
struct Data(i32);

fn consume(data: Data) -> i32 {
    data.0
}

#[test]
fn closures() {
    let x = Data(42);
    let unwrap = move || consume(x);
    // assert!(call_fn0(unwrap) == 42);
    // assert!(call_fn_mut0(unwrap) == 42);
    assert!(call_fn_once0(unwrap) == 42);

    let mut x = Data(-1);
    let update = || {
        x.0 += 1;
        x
    };
    // call_fn0(update);
    let (c, update) = call_fn_mut0f(update);
    assert!(c.0 == 0);
    assert!(call_fn_once0(update).0 == 1);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(call_fn1(equal_to_x, y));
    assert!(call_fn_mut1(equal_to_x, y));
    assert!(call_fn_once1(equal_to_x, y));
}

#[allow(dead_code)]
fn call_fn0<F, A>(f: F) -> A
where
    F: Fn() -> A,
{
    f()
}

fn call_fn1<F, A, B>(f: F, arg: A) -> B
where
    F: Fn(A) -> B,
{
    f(arg)
}

#[allow(dead_code)]
fn call_fn_mut0<F, A>(mut f: F) -> A
where
    F: FnMut() -> A,
{
    f()
}

fn call_fn_mut0f<F, A>(mut f: F) -> (A, F)
where
    F: FnMut() -> A,
{
    (f(), f)
}

fn call_fn_mut1<F, A, B>(mut f: F, arg: A) -> B
where
    F: FnMut(A) -> B,
{
    f(arg)
}

fn call_fn_once0<F, A>(f: F) -> A
where
    F: FnOnce() -> A,
{
    f()
}

fn call_fn_once1<F, A, B>(f: F, arg: A) -> B
where
    F: FnOnce(A) -> B,
{
    f(arg)
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[test]
fn use_macro() {
    assert!(count!() == 0);
    assert!(count!(,) == 1);
    assert!(count!(,,) == 2);
    assert!(count!(,,,) == 3);
}

macro_rules! count_args {
    () => (0usize);
    ( $x:expr ) => (1usize);
    ( $x:expr , $($xs:expr),* ) => (1usize + count_args!($($xs),*));
}

#[test]
fn use_count_args_macro() {
    assert!(count_args!() == 0);
    assert!(count_args!(1) == 1);
    assert!(count_args!(1, 2, 3, 4, 5) == 5);
}
