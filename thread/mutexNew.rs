use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += *num;
        // После получения блокировки мы можем воспринимать возвращённое значение,
        // названное в данном случае num, как изменяемую ссылку на содержащиеся внутри данные.
    }

    println!("m = {:?}", m);
}
