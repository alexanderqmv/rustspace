use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use serde::{Serialize};
use serde_json::{Result, to_string_pretty};


#[derive(Debug,Serialize)]
struct Database {
    id: u32,
    name: &'static str,
    surname: &'static str,

    age: u8,
    email: &'static str,
    
    address: &'static str,
}


impl Database {
    
    fn new(id: u32, name: &'static str, surname: &'static str, age: u8, email: &'static str, address: &'static str) -> Self {
        Self {
            id,
            name,
            surname,
            age,
            email,
            address,
        }
    }

    fn write(&self,path: String, data: &String) -> std::io::Result<()>
    {
        let op = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        let mut buf_writer = BufWriter::new(op);
        buf_writer.write_all(data.as_bytes())?;
        Ok(())
    }

    fn read(&self, path: String) -> String 
    {
        let fread = std::fs::read_to_string(&path).unwrap();
        fread 
    }

    fn to_json(&self, db: &Database) -> Result<String>
    {
        let json = to_string_pretty(&db)?;
        Ok(json)
    }
}





fn main() {   
    let mut db = Database::new(
        1,
        "John",
        "Doe",
        39,
        "johndoe@gmail.com",
        "01 The Builder Ricky street");
  
    let ser = match db.to_json(&db){
        Ok(t) => t,
        Err(why) => format!("Error: {why}"),
    }; 
    db.write("output.txt".to_string(), &ser).unwrap();
    let read = db.read("c:/users/qumar/desktop/rust/output.txt".to_string());
    println!("read: {read}");
}
