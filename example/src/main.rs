mod module_a;

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let mut s1 = String::from("hello");
    let len = calc_len(&s1);
    println!("len  : {}", len);
    append_str(&mut s1);
    println!("after: {}", s1);

    let mut s2 = String::from("Rust tutorial");
    // <1> 行のコメント化を解除すると、s2.clear() で可変借用に失敗する
    // s2w という不変参照がまだ生きているため
    let s2w = first_word(&s2);
    println!("s2 (before): {}", s2w);
    s2.clear();
    s2.push_str("Modified");
    println!("s2  (after): {}", s2);
    // println!("{}", s2w); // <1>

    let mut s3 = String::from("Rust");
    with_exclamation(&mut s3);
    println!("s3: {}", s3); // => "Rust!"

    let x = &5;
    let foo = Foo { x };
    println!("x is: {}", foo.x()); // => "x is: 5"

    module_a::func();
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn append_str(s: &mut String) {
    s.push_str(", world!")
}

// 文字列スライスの使用例
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

// ライフタイムパラメータを省略しない場合
fn with_exclamation<'a>(s: &'a mut String) {
    s.push_str("!")
}
