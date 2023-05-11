use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    // Первая задача: записываем данные в канал
    let producer = thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    // Вторая задача: считываем данные из канала
    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let data = rx.recv().unwrap();
            println!("Received data: {}", data);
        }
    });

    // Дожидаемся завершения задач
    producer.join().unwrap();
    consumer.join().unwrap();
}
