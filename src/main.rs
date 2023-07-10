use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut database = Database::new().expect("Database creation failed");
    match args.len() {
        1 => {
            let key = &args[0];
            println!("The key is {} and there is no value", key);
            let value = database.get(key.clone());
            match value {
                Ok(value) => {
                    println!("The value associated to the key {} is {}", key, value);
                }
                Err(e) => {
                    print!("Error: {}", e)
                }
            }
        }
        2 => {
            let key = &args[0];
            let value = &args[1];
            println!("The key is {} and value is {}", key, value);
            database.insert(key.clone(), value.clone());
            database.insert(key.to_uppercase(), value.clone());
        }
        _ => {
            // show a help message
            help();
        }
    }
    // database.flush().unwrap(); // now moved to trait "DROP"
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        if std::path::Path::new("kv.db").exists() {
            let contents = std::fs::read_to_string("kv.db")?;
            for line in contents.lines() {
                let mut pair = line.splitn(2, "\t");
                let key = pair.next().expect("msg");
                let value = pair.next().expect("msg");
                map.insert(key.to_owned(), value.to_owned());
            }
        }
        return Ok(Database { map });
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(self, key: String) -> std::io::Result<String> {
        return Ok(self.map.get(&key).expect("get failed").to_string());
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // Flush DB functionality
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        let _ = std::fs::write("kv.db", contents);
    }
}

fn help() {
    println!(
        "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
    );
}
