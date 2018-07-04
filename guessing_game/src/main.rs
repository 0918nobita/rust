extern crate rand;

// io 標準ライブラリを読み込む
use std::io;
use rand::Rng;

// エントリーポイント
// 戻り値の型は書いていないため、空のタプルとして扱われる
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // 変数束縛を作る ( mut を使用してミュータブルに )
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)  // &mut guess は guess の参照
    	.expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}

// Rust コードのパッケージを「クレート」という。
