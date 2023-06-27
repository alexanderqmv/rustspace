use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Write;

#[derive(PartialEq, Clone)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Database {
    db: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { db: Vec::new() }
    }

    fn add(&mut self, user: User) {
        self.db.push(user);
    }

    fn rm(&mut self, user: &User) {
        if let Some(index) = self.db.iter().position(|x| *x == *user) {
            self.db.remove(index);
        } else {
            println!("Could not find the key!");
        }
    }

    fn show(&self) {
        for (i, k) in self.db.iter().enumerate() {
            let result = self.hash(&k.password);
            println!("{i} - {}: {}", k.username, result);
        }
    }

    fn hash(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(input);
        let result = hasher.result_str();
        result
    }

    fn export(&self, file: &str) -> std::io::Result<()> {
        let mut file = File::create(file)?;
        for i in self.db.iter() {
            let mut format = format!("({:#?}, {:#?})\n", i.username, self.hash(&i.password));
            file.write_all(format.as_bytes())?;
        }

        Ok(())
    }
    fn pop(&mut self) {
        if !self.db.is_empty() {
            self.db.pop();
        } else {
            return;
        }
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.username, self.password)
    }
}

fn main() {
    let mut database = Database::new();

    let user = User {
        username: "jawid".to_string(),
        password: "jawid_helsinki1997".to_string(),
    };

    let user2 = User {
        username: "Maria".to_string(),
        password: "m.javakhishvili!!!".to_string(),
    };
    let user1_clone = user.clone();
    database.add(user);
    database.add(user2);
    database.show();
    println!();
    println!();
    database.show();
    database
        .export("c:/users/qumar/desktop/rust_program_example.txt")
        .unwrap();
}
