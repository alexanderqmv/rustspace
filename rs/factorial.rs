
fn main() {
    let r = factorial(30);
    println!("{r}");
}


fn factorial(n: i128) -> i128 {
    return if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
