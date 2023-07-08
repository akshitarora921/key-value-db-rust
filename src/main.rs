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
   
}