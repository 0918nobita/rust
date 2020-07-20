use super::cacher::Cacher;

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _ = c.value(1);
    let v = c.value(2);
    assert_eq!(v, 2)
}
