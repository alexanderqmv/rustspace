use std::fs::File;
use std::io::{Read,Write};



struct FileHandler
{
    file_name: String,
}
impl FileHandler
{
    /*  rcargo  */
    fn new(file_name: &str) -> FileHandler
    {
        FileHandler { file_name: file_name.to_string() }
    }

    fn read_file(&self) -> Result<String, std::io::Error>
    {
        let mut file = File::open(&self.file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn write_file(&self, contents: &str) -> Result<(), std::io::Error>
    {
        let mut file = File::create(&self.file_name)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}



fn main()
{
    let file_handler = FileHandler::new("example.txt");
    let contents = file_handler.read_file().expect("Could not read file");
    println!("File contents: {}", contents);
    let new_contents = "This is new file content! Hello! Rust is top!";
    file_handler.write_file(new_contents).expect("Could not write to file");
    println!("File written successfully! Hello! Rust is top!") ;
}


