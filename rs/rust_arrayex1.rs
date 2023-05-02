 fn main(){
    let id = [1,2,3,4,5];
    let id = [1;5];
    let mut lang : [&str;5];
    lang = ["c#", "python", "java", "rust", "c++"];

    println!("lang : {}", lang[0]);
    lang[0] ="javascript";
    for l in lang.iter() {
        println!("lang : {}", l);
    }
}


