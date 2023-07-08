fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key is not present");

    let value = arguments.next().expect("Value is not present");
    println!("The key is {} and the value is {}", key, value);
    let content = format!("{}\t{}\n", key, value);

    // check if file exist
    // if not then create a file
    // if yes then add data to the same file
    if !std::path::Path::new("kv.db").exists() {
        std::fs::write("kv.db", content).unwrap();
    } else {
        let contents = std::fs::read_to_string("kv.db").expect("oops");
        std::fs::write("kv.db", [contents, content].concat()).unwrap();
    }
    let database = Database::new().expect("Database creation failed");
    // let value = database.get(key).unwrap();
    // println!("value is {}", value)
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut pair = line.splitn(2, "\t");
            let key = pair.next().expect("msg");
            let value = pair.next().expect("msg");
            map.insert(key.to_owned(), value.to_owned());
        }
        return Ok(Database { map: map });
    }
    fn insert(mut self, key: String, value: String) -> Result<bool, std::io::Error> {
        self.map.insert(key, value).expect("insertion failed");
        return Ok(true);
    }
    fn get(self, key: String) -> Result<String, std::io::Error> {
        let value = self.map.get(&key).expect("get failed");
        return Ok(value.to_owned());
    }
}
