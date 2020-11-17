use futures::executor::block_on;
use futures::join;

async fn learn_song() {
    println!("learn song");
}

async fn sing_song() {
    learn_song().await;
    println!("sing song");
}

async fn dance() {
    println!("dance");
}

async fn async_main() {
    println!("async main");
    let f1 = sing_song();
    let f2 = dance();
    join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
