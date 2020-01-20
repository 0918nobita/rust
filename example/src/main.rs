fn main() {
    let mut s1 = String::from("hello");
    let len = calc_len(&s1);
    println!("len: {}", len);
    append_str(&mut s1);
    println!("after: {}", s1)
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn append_str(s: &mut String) {
    s.push_str(", world!")
}
