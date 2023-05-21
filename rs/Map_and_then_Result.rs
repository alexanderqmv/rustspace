use std::num::ParseIntError;
fn main() {
    let result = multiply("10", "2");
    print(result);
    

    let tt = multiply("10t", "2");
    print(tt);

    let result_map = multiply_map("10t", "2");
    print(result_map);
    let tt_map = multiply("10", "2");

    print(tt_map);
}



fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32,ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        
        },
        Err(e) =>  Err(e),
    }
}

fn multiply_map(first_number_str: &str,second_number_str: &str) -> Result<i32, ParseIntError>{
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}





fn print(result: Result<i32,ParseIntError>) {
    match result {
        Ok(n) => println!("n is {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
