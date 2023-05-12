# RustSpace
Rust Code Collection 


# Instructions & Review (Ru)

## Многопоточность
Синхронизация данных между потоками в Rust обеспечивается с помощью механизмов синхронизации, которые предоставляются в стандартной библиотеке языка. Некоторые из них:
### 1. Mutex
Mutex - это механизм синхронизации, который позволяет доступ к общим данным только одному потоку в определенный момент времени. Для создания Mutex в Rust используется структура Mutex и методы lock и unlock:
```rs
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    {
        let mut num = mutex.lock().unwrap();
        *num += 1;
    } // блокировка Mutex освобождается автоматически после выхода из блока
    println!("The mutex contains: {:?}", mutex);
}

```
### 2. RwLock
RwLock - это механизм синхронизации, который позволяет нескольким потокам получать доступ к общим данным для чтения, но только одному потоку для записи. Для создания RwLock в Rust используется структура RwLock и методы read и write:
```rs
use std::sync::RwLock;

fn main() {
    let rwlock = RwLock::new(0);
    {
        let num = rwlock.read().unwrap();
        println!("Read lock acquired: {:?}", *num);
    } // блокировка чтения RwLock освобождается автоматически после выхода из блока
    {
        let mut num = rwlock.write().unwrap();
        *num += 1;
        println!("Write lock acquired: {:?}", *num);
    } // блокировка записи RwLock освобождается автоматически после выхода из блока
}

```
