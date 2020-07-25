extern crate sec17_1;
use sec17_1::AveragedCollection;

// オブジェクト指向プログラムは、オブジェクトで構成される。
// オブジェクトとは、データとそれを処理するプロシージャを内包している。
// このプロシージャは、典型的にメソッドまたはオペレーションと呼ばれる。

fn main() {
    let mut collection = AveragedCollection::new();
    collection.add(10);
    collection.add(30);
    println!("{}", collection.average()); // => 20
    collection.remove().unwrap();
    println!("{}", collection.average()); // => 10
}
