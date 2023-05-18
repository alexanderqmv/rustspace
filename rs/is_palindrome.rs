fn main() {
    let str = "racecar".to_string();
    let res = is_palindrome(&str);
    println!("{res}");
}

fn is_palindrome(str: &String) -> bool
{
    let s = str
        .chars()
        .filter(|c| c.is_alphanumeric()).collect::<String>();    
    s.chars().eq(s.chars().rev())    
} 
