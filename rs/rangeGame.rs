use rand::prelude::*;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut range = thread_rng();

    let y: i32 = range.gen_range(0..1000);
    //println!("Generated: {}", y);
    let mut buf = String::new();
    let mut flag = false;

    while !flag {
        print!("Guess the number between 0..1000: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();

        match buf.trim().parse::<i32>() {
            Ok(var) => {
                if var == y {
                    println!("You guessed!");
                    flag = true;
                } else if var < y {
                    println!("More");
                } else if var > y {
                    println!("Less");
                } else {
                    println!("No");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
        buf.clear();
    }
}
