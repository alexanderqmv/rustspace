use std::slice;
// ^^^ you can comment the string
use std::str;




fn first_before_space(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}

fn main() {
    let string_slice = "helloworld!";
    let res = first_before_space(&string_slice);
    println!("Got: {res:?}");

}
