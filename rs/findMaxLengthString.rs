fn max_length(str: Vec<&str>) -> Option<&str> {
    if str.len() == 0 {
        return None; 
    }
    let mut longest = str[0];
    for i in str.iter() 
    {
        if longest.len() < i.len() {
            longest = i; 
        }
    }
    Some(longest)
}


#[cfg(test)]
mod tests
{
    use super::max_length;
    #[test]
    fn max_length_test() {
        let str = vec!["hello","world!", "I am Alex!"];
        let res = max_length(str).unwrap();
        assert_eq!(res, "I am Alex!");
    }
}
