use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        if let Some(v) = self.value.get(&arg) {
            v.clone()
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, v);
            v.clone()
        }
    }
}
