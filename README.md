```rs
#[derive(Debug)]
struct RustSpace 
{
   folders_count: u16,
   lang: String,
   name: String,
   stars: u16,
}

fn main() {
  println!("Rust Code Collection");
  let rcc = RustSpace
  {
     folders_count: 5,
     lang: String::from("Rust"),
     name: String::from("RustSpace"),
     stars: 1
  };
  println!("{rcc:?}");
}
```



