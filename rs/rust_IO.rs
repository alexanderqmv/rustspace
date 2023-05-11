use std::io::Write;

fn main() -> std::io::Result<()>{
    let mut input = String::new();
    print!("Enter your name: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    println!("You entered {}", input.trim());
    Ok(())
}
