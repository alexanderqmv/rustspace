use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc::channel;


const N: u8 = 3;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let (tx,rx) = channel();
    for _ in 0..N
    {
        let (data,tx) = (Arc::clone(&counter), tx.clone());  
        thread::spawn(move ||
        {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N{
                
            }else {
                tx.send("couldn't get!").unwrap();
            }
        });
        // the lock is unlocked here when `data` goes out of scope
    }
    println!("Result: {:?}",rx.recv().expect("Pidor"));
}
