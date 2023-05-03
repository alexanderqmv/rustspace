#![allow(dead_code)]

enum Operation {
    Substract,
    Add,
    Divide,
    Multiply,
}
enum Cmd {
    Quit
    ,WriteText(String)
    ,ChangeColor(Color)
}
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn main()
{
    let op = Operation::Add;

    println!("{:?}", calc(op, 111,111));
    let cmd = Cmd::WriteText("Bear!".to_string());

    match cmd {
        Cmd::WriteText(message) => println!("MESSAGE: {}", message),
        Cmd::ChangeColor(color) => println!("rgb: {},{},{}", color.r, color.g, color.b),
        Cmd::Quit => print!("quit!"),
    }


}

fn calc(op: Operation, x: i32, y: i32) -> i32{
    match op{
        Operation::Add => x + y,
        Operation::Multiply => x * y,
        _ => 0,
    }
}
