use std::collections::HashMap;
fn main() {
    let mut book_review = HashMap::new();
    book_review.insert("Adventure Time".to_string(), 1);


    if book_review.contains_key("Adventure Time") { println!("YES"); }
    else { println!("No!"); }
    if book_review.contains_key("Adventure Times") { println!("YES"); }
    else { println!("No!"); }
    book_review.insert("Adventure Times".to_string(), 2);


    if book_review.contains_key("Adventure Times") { println!("YES"); }
    else { println!("No!"); }

    book_review.remove("Adventure Time");
    if book_review.contains_key("Adventure Time") { println!("YES"); }
    else { println!("No!"); }

    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    for item in solar_distance {
        println!("{}:{}", item.0, item.1);
    }

    let to_find = ["Rust", "Adventure Times"];
    for &item in &to_find {
        match book_review.get(item) {
            Some(review) => println!("{item}:{review}"),
            None => println!("{item} is unreviewed!"),
        }
    }
}
