// io 標準ライブラリを読み込む
use std::io;

// エントリーポイント
// 戻り値の型は書いていないため、空のタプルとして扱われる
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // 変数束縛を作る ( mut を使用してミュータブルに )
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
    	.expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
