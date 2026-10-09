use std::thread;
use std::time::Duration;
// https://doc.rust-lang.org/book/ch16-01-threads.html

// Testing with threads in Rust

// I may use this for something in the future

// The delays for the testing threads.
const THREAD1_DELAY_MS: u64 = 1000;
const THREAD2_DELAY_MS: u64 = 1000;

fn main() {
    // let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(THREAD2_DELAY_MS));
            }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(THREAD1_DELAY_MS));
    }

    handle.join().unwrap();

}
