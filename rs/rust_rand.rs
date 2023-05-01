use std::io;  //  std::stdin/stdout
use rand::Rng; // std::rand::Rng [thread_rng().gen_range(range)] (v0.3.0)


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Secret_number: {}", secret_number);

    loop {
        println!("Please, make your input: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Could not read the string");

        // Parse the user's input as an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };


        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
