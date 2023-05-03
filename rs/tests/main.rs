mod tests;

fn main() {
    let add = addition(10,111);
    println!("addition() result: {}", add);

    let my_div = div(10, 0);
    println!("div() result: {:?}",my_div);

}

fn addition(a: i32, b: i32) -> i32
{
    a + b
}

fn div(a: i32, b: i32) -> Result<i32, &'static str>
{
    if a == 0 {
       return  Err("Error: Can not divide by zero!");
    }
    if b == 0 {
        return Err("Error: Numerator is zero!");
    }

    Ok(a / b)
}


