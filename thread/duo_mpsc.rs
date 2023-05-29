use std::thread;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let (tx, rx) = mpsc::channel();

    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);
    let tx1 = tx.clone();

    thread::spawn(move || {
        let mut counter = counter_clone.lock().unwrap();
        for _ in 0..100 {
            *counter += 1;
        }

        tx1.send(*counter).unwrap();
    });

    thread::spawn(move || {
        let mut counter = counter.lock().unwrap();
        for _ in *counter..200 {
            *counter += 1;
        }

        tx.send(*counter).unwrap();
    });

    let res = rx.recv().unwrap();
    let res2 = rx.recv().unwrap();
    println!("Received value: {}",res2);
}
