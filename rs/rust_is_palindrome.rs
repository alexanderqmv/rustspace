fn is_palindrome(s: &str) -> Option<bool> {
    if s.is_empty() {
        return None;
    }
    for i in 0..s.len() / 2 {
        if s.chars().nth(i) != s.chars().nth(s.len() - i - 1) {
            return Some(false);
        }
    }
    Some(true)
}

fn main() {
    let input = "racecar";
    match is_palindrome(input) {
        Some(true) => println!("'{}' is a palindrome!", input),
        Some(false) => println!("'{}' is not a palindrome.", input),
        None => println!("The input string is empty."),
    }
}
