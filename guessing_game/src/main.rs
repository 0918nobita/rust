extern crate rand;

// io 標準ライブラリを読み込む
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// エントリーポイント
// 戻り値の型は書いていないため、空のタプルとして扱われる
fn main() {
    println!("Guess the number!");

    // rand::thread_rng() スレッドにローカルな乱数生成器のコピーを取得する
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 変数束縛を作る ( mut を使用してミュータブルに )
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)  // &mut guess は guess の参照
    	    .expect("Failed to read line");

        // すでに定義してある guess を新しい定義で隠す ( シャドーイング )
        // 文字列の trim メソッドは、文字列の最初と最後にある空白を取り除く
        // 文字列の parse メソッドは、文字列を何かの数値へとパースする
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number");
    
        println!("You guessed: {}", guess);

        // cmp() 自分と比較可能なものの参照を受け取り、Ordering を返す
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You Win!");
                break;
            }
        }
    }
}
