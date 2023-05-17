#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}


fn main() {
    let person1 = Person 
    { 
        name: "Bob".to_string(),
        age: 39,
    };
    let person2 = Person 
    {
        name: "Bob".to_string(),
        age: 39,
    };
    let checker = person1 == person2;
    
    println!("person1 == person2: {checker}");
}   
