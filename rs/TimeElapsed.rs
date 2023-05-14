use std::time::{Duration,Instant};
use std::thread;


fn foo() 
{
    for i in 0..11{
        thread::sleep(Duration::from_millis(1000));
        println!("{i}");
    }
}
fn main()
{
    let start = Instant::now();
    foo();

    let end = start.elapsed();
    println!("{end:?}");

}
