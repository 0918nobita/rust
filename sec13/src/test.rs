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
