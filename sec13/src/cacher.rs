use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, A, B>
where
    T: Fn(A) -> B,
    A: Copy + Eq + Hash,
    B: Copy,
{
    calculation: T,
    value: HashMap<A, B>,
}

impl<T, A, B> Cacher<T, A, B>
where
    T: Fn(A) -> B,
    A: Copy + Eq + Hash,
    B: Copy,
{
    pub fn new(calculation: T) -> Cacher<T, A, B> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: A) -> B {
        if let Some(v) = self.value.get(&arg) {
            v.clone()
        } else {
            let v: B = (self.calculation)(arg);
            self.value.insert(arg, v);
            v.clone()
        }
    }
}
