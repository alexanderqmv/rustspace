// Define a custom trait with a single method
trait Log {
    fn log(&self);
}

// Implement the trait for a struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Implement the Log trait for the Person struct
impl Log for Person {
    fn log(&self) {
        println!("Person: {{ name: {}, age: {} }}", self.name, self.age);
    }
}

// Implement the trait for a different struct
#[derive(Debug)]
struct Animal {
    species: String,
    age: u32,
}

// Implement the Log trait for the Animal struct
impl Log for Animal {
    fn log(&self) {
        println!("Animal: {{ species: {}, age: {} }}", self.species, self.age);
    }
}

// Define a function that accepts any type that implements the Log trait
fn log<T>(object: &T)
    where
        T: Log,
{
    object.log();
}

fn main() {
    let person = Person { name: "Alice".into(), age: 30 };
    let animal = Animal { species: "Dog".into(), age: 5 };

    // Call the log function with both structs
    log(&person);
    log(&animal);
}
