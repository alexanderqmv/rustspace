struct Person
{
    name: String,
    age: u32,
}

impl Person
{
    // constructor
    // C++ constructor example:
    // Person(...);
    fn new(name: String, age: u32) -> Person
    {
       return Person { name , age };
    }

    // method, which returns name
    fn get_name(&self) -> &str
    { return &self.name; }
    fn get_age(&self) -> u32
    { return self.age; }
    fn set_age(&mut self, age: u32) { self.age = age; }
}



fn print_person_info(person: &Person)
{
    println!("Name: {}", person.get_name());
    println!("Age:  {}", person.get_age());
}



fn main()
{
    let mut person = Person::new("John Doe".to_string(), 30);
    print_person_info(&person);
    person.set_age(35);
    print_person_info(&person);
}
