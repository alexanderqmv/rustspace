use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // создаем мьютекс для доступа к счетчику
    let mut handles = vec![]; // создаем вектор для хранения хэндлов потоков

    // первый поток
    let counter_clone = counter.clone(); // клонируем мьютекс для передачи его в поток
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            let mut num = counter_clone.lock().unwrap(); // забираем мьютекс
            *num += 1; // увеличиваем значение счетчика
            println!("Thread 1: counter = {}", *num);
        }
    });
    handles.push(handle1); // добавляем хэндл первого потока в вектор

    // второй поток
    let counter_clone = counter.clone(); // клонируем мьютекс для передачи его в поток
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            let mut num = counter_clone.lock().unwrap(); // забираем мьютекс
            *num += 1; // увеличиваем значение счетчика
            println!("Thread 2: counter = {}", *num);
        }
    });
    handles.push(handle2); // добавляем хэндл второго потока в вектор

    // ждем, пока все потоки завершатся
    for handle in handles {
        handle.join().unwrap();
    }
}
