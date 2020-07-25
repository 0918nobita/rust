extern crate sec17_2;
use sec17_2::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // トレイトオブジェクトには、オブジェクト安全性が必要
    // - 戻り値の型が Self でない
    // - ジェネリックな型引数がない
}
