fn check_vectors(arr_one: &mut Vec<i32>, arr_two: &mut Vec<i32>) -> Result<bool, String> {
    if arr_one.len() != arr_two.len() {
        return Err("Cannot compare vectors. Different sizes.".to_string());
    }
    arr_one.sort();
    arr_two.sort();
    for i in 0..arr_one.len() {
        if arr_one[i] != arr_two[i] {
            return Ok(false);
        }
    }
    Ok(true)
}

fn main() {
    let mut v1 = vec![1,2,3,4,5];
    let mut v2 = vec![5,4,3,2,1];

    match check_vectors(&mut v1, &mut v2) {
        Err(error) => println!("Error: {}", error),
        Ok(var) => println!("Result: {}", var),
    };
}
