use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn main() {
    // MutexGuard<T> はスマートポインタ
    // 内部のデータを指す Deref を実装
    // 自動的にロックを解除する Drop を実装

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num: MutexGuard<i32> = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
