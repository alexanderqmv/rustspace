use std::sync::{Arc, Mutex};
use std::thread;

fn increment_count(count: Arc<Mutex<i32>>) {
    let mut num = count.lock().unwrap();
    *num += 1;
    println!("Count: {}", *num);
}

fn main() {
    let count = Arc::new(Mutex::new(0));

    let mut threads = vec![];
    for _ in 0..2 {
        let count = Arc::clone(&count);
        let thread = thread::spawn(move || {
            increment_count(count);
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
