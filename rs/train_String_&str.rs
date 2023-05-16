use std::collections::TryReserveError;
 




fn process_data(data: &str) -> Result<String, TryReserveError> {
    // If the capacity overflows, or the allocator
    // reports a failure, then an error is returned.
    let mut output = String::new();
    output.try_reserve(data.len())?;
    output.push_str(data);
    
    Ok(output)
}

fn main() {
    let mut string = String::new();
    string.reserve(20);    
    println!("reserved: {}", string.capacity());
    string = "hello,world,how_are_you? I am good broou".to_string();
    println!("used: {}", string.len());
   
    println!("cap: {}", string.capacity());
    ///////////////////////////////////////
    
    let data = match process_data("hello") {
        Ok(str) => str,
        Err(why) => { 
            println!("{:?}", why);  return;
        }
    };

    println!("Data: {data}");
    if let Ok(data) = process_data("hello") {
        println!("I got you pidor!");
    }else {
        println!("Noooo pLzz!");
    }
    //////////////////////////////////////// 

    let my_str = "malboro";
    //assert!(my_str.contains("nana"));
    println!("contains: {}", my_str.contains("malbo"));
    for (i,byte) in my_str.chars().enumerate() {
        println!("{i}:{byte}");
    }

    let mut outputter = String::new();
    let tryresrv = outputter.try_reserve(10);
    if let Ok(()) = tryresrv {
        outputter = "hello!".to_string();
        println!("Reserved successfully: ([value: {} ], [bytes: {:#?}])",
                outputter,
                outputter.trim().as_bytes());
    }else {
        eprintln!("Couldn't reserve (10)");
    }

    ////////////////////////////////////////////  
    let mut some_str = String::from("Hello, World!");
    let split_off = some_str.split_off(7);
    println!("some_str: {some_str} -- {split_off}");

}
