//use std::collections::VecDeque;
//use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


fn foo() -> i8{
    42
}


fn main()
{

}
#[cfg(test)]
#[test]
#[ignore]
fn test1()
{
    let handle = thread::spawn(|| {
        for i in 0..10
        {
            thread::sleep(Duration::from_millis(1000));
            println!("[WORK]\t - \t{i}");
        }
    });
    assert!(handle.join().is_ok());
}

#[test]
fn test2()
{
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let result = foo();
        tx.send(result).unwrap();
    });

    let thread_result = rx.recv().unwrap();
    assert_eq!(thread_result, 42);
    assert!(handle.join().is_ok());
}
