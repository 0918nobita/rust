/*
 * 食事する哲学者 ( 古典的な並行処理問題 )
 *
 * それぞれの哲学者に部屋が与えられた。
 * 共用のダイニングルームでは、丸いテーブルが置かれ、5人それぞれのための席がある。
 * 彼らは半時計回りに座る。
 * 
 * 哲学者の左側にはそれぞれ金のフォークが配られる。
 * 中央には大きなボウルに入ったスパゲッティが常に補充される。
 * 
 * 哲学者は、空腹になったときにダイニングルームに出向き、
 * 自分専用の椅子に座り、左側のフォークを取り上げ、スパゲッティに突き刺す。
 * しかし、絡まりあったスパゲッティを口元まで運ぶには2本目のフォークが必要だった。
 * そのため自分の右側にあるフォークも使う必要があった。
 *
 * 食べ終わったら両側のフォークを元に戻し、席から立ち上がって、思索活動を続ける。
 * もちろん、1 本のフォークは同時に 1 人の哲学者しか使えない。
 * 他の哲学者が食事したければ、フォークが再び戻されるまでに待たなければならない。
 *
 * 次の問題を解く単純なアルゴリズムを考える
 * 1. 哲学者は左側のフォークを取り上げる
 * 2. 続いて右側のフォークを取り上げる
 * 3. 食事をする
 * 4. 2 本のフォークを戻す
 *
 * 次のような一連の出来事を想像する
 * 1. 1 番目の哲学者は、アルゴリズムに従って左側のフォークを取り上げる
 * 2. 2 〃
 * 3. 3 〃
 * 4. 4 〃
 * 5. 5 〃
 * 6. すべてのフォークが取られたので、誰も食事ができない
 *
 * これを解決するプログラムを作成する
 */

use std::thread;
use std::time::Duration;

// 哲学者を表す構造体を定義する
// &str (データを参照する型) ではなく String (データを所有する型)
struct Philosopher {
    name: String,
}

// Philosopher 構造体に対する定義を与える
// ここでは new という関連関数を定義する
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    // &self を取るので、関連関数ではなくメソッドとなる
    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    // Vec<_> の _ は型プレースホルダ (型推論させる)
    // into_iter() 哲学者の所有権を持つイテレータを生成する
    // map の引数: 要素ごとに呼び出されるクロージャ
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // thread::spawn 新しいスレッド上で、渡されたクロージャを実行する
        // move クロージャに付与する「アノテーション」
        //      キャプシャする値の所有権がクロージャ内へと移動される
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
