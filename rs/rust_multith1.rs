use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;


fn main()
{
    let thread1: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread1.join().unwrap();

    for i in 1..5
    {
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

}




