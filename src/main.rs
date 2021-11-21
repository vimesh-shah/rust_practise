use std::collections::HashMap;

fn main() {
    // Get data from command line arguments
    let mut args = std::env::args().skip(1);

    let op = args.next().expect("Operation is not there");

    let mut db = Database::new().expect("Unable to initialize database.");

    if op == "get" {
        let key = args.next().expect("Key is not there");
        let value = db.get(key.as_str());

        println!("Value is {}", value);
    }

    if op == "set" {
        let key = args.next().expect("Key is not there");
        let value = args.next().expect("Value is not there");

        db.set(key.as_str(), value.as_str());
        db.write_to_disk();
    }
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // Read test.db file
        let contents = match std::fs::read_to_string("test.db") {
            Ok(c) => c,
            Err(error) => {
                return Err(error);
            }
        };

        // '?' at end bubbles up the error.
        // It is similar to above code
        // let contents = std::fs::read_to_string("test.db")?;

        let mut map: HashMap<String, String> = HashMap::new();

        // Parse the contents
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupted database");

            // Populate map
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map })
    }

    fn get(&self, key: &str) -> String {
        let value = self.map.get(key).expect("Key not found").to_owned();
        value.to_string()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
    }

    fn write_to_disk(&self) {
        let mut contents = String::new();

        for (key, value) in self.map.iter() {
            let item = format!("{}\t{}\n", key, value);
            contents.push_str(item.as_str());
        }

        // Writing formatted content to file
        let write_result = std::fs::write("test.db", contents);

        match write_result {
            Ok(()) => {
                println!("Data saved successfully.");
            }
            Err(e) => {
                println!("Data failed to save.");
                println!("Error : {}", e);
            }
        }
    }
}
