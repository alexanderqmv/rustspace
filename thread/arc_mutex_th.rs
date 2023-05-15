use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let threads = Arc::new(Mutex::new(Vec::new()));
    for _ in 0..2{
        let threads_clone = Arc::clone(&threads);
        threads_clone.lock().unwrap().push(thread::spawn(|| {
            for k in 0..10 {
                thread::sleep(Duration::from_millis(500));
                println!("{k}");
            }
        }));
    }    


    for thread in threads.lock().unwrap().drain(..) {
        thread.join().unwrap();
    }
    println!("sizeOf: {}", threads.lock().unwrap().len());

}
