use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Открываем файл на чтение
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() { 
        let word = line.unwrap();
        if is_palindrome(&word) {
            println!("{}", word);
        }
    }
}

// Функция для проверки, является ли строка палиндромом
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())    
}
