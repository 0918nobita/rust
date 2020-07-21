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

#[test]
fn closures() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(call_fn1(equal_to_x, y));
    assert!(call_fn_mut1(equal_to_x, y));
    assert!(call_fn_once1(equal_to_x, y));
}

fn call_fn1<F, A, B>(f: F, arg: A) -> B
where
    F: Fn(A) -> B,
{
    f(arg)
}

fn call_fn_mut1<F, A, B>(mut f: F, arg: A) -> B
where
    F: FnMut(A) -> B,
{
    f(arg)
}

fn call_fn_once1<F, A, B>(f: F, arg: A) -> B
where
    F: FnOnce(A) -> B,
{
    f(arg)
}

#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut v1_iter2 = v1.into_iter();
    assert_eq!(v1_iter2.next(), Some(1));
    assert_eq!(v1_iter2.next(), Some(2));
    assert_eq!(v1_iter2.next(), Some(3));
    assert_eq!(v1_iter2.next(), None);
}

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // sum: consuming adaptor
    assert_eq!(total, 6);

    let v1_iter = v1.iter();
    let v2: Vec<_> = v1_iter
        .map(|x| x + 1) // map: iterator adaptor
        .collect(); // collect: consuming adaptor
    assert_eq!(v2, vec![2, 3, 4]);
}
