use std::time::Duration;
use std::thread::{JoinHandle};
use std::thread;

#[allow(dead_code)]
fn simple_thread(name: String, values: Vec<i32>) -> JoinHandle<i32> {
    let join_handle = thread::spawn(move || {
        let mut total: i32 = 0;
        for value in values {
            total = total + value;
            thread::sleep(Duration::from_secs(2));
            println!("{}の現在の値: {}", name, total);
        }
        total
    });
    join_handle
}

#[allow(dead_code)]
pub fn execute_thread() {
    let thd1 = simple_thread(String::from("スレッド1"), (1..4).collect());
    let thd2 = simple_thread(String::from("スレッド2"), (2..5).collect());
    let result1 = thd1.join().map_err(|error| panic!("{:?}", error)).unwrap();
    let result2 = thd2.join().map_err(|error| panic!("{:?}", error)).unwrap();
    println!("スレッド1の合計値: {}", result1);
    println!("スレッド2の合計値: {}", result2);
}