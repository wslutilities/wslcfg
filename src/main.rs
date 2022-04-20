use std::env;
use serde_json;
use std::fs::File;
use std::io::BufReader;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file = File::open("src/default.json").expect("failed to read file");
    let reader = BufReader::new(file);
    let default_set: serde_json::Value =
        serde_json::from_reader(reader)
            .expect("JSON was not well-formatted");
    println!("{:?}", default_set);

}
