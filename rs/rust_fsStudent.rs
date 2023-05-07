use std::fs::File;
use std::io::{Write, Read};

struct Student {
    name: String,
    surname: String,
    age: u8,
    average_mark: u8,
}

impl Student {
    fn new(name: String, surname: String, age: u8, average_mark: u8) -> Student {
        Student {
            name,
            surname,
            age,
            average_mark,
        }
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_surname(&self) -> String {
        self.surname.to_string()
    }

    fn get_age(&self) -> u8 {
        self.age
    }

    fn get_avg_mark(&self) -> u8 {
        self.average_mark
    }

    fn export(&self, data: String, to: String) -> std::io::Result<()> {
        let mut file = File::create(to)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

fn main() {
    let student = Student::new("Alex".to_string(), "Robakidze".to_string(), 11, 235);

    println!("Student\nname: {}", student.get_name());
    println!("surname: {}", student.get_surname());
    println!("age: {}", student.get_age());
    println!("average marks: {}", student.get_avg_mark());

    let file_name = "foo.txt".to_string();
    let file_content = student.get_name().to_string();

    if let Err(e) = student.export(file_content, file_name) {
        eprintln!("Error: {}", e);
    }
}
