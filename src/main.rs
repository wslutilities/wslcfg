use std::env;
use std::fs::File;
use std::io::BufReader;

extern crate ini;
use ini::Ini;

use serde_json;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let conf = Ini::load_from_file("/etc/wsl.conf").unwrap();

    let default_file =
        File::open("src/default.json")
            .expect("failed to read file");
    let reader = BufReader::new(default_file);
    let default_set: serde_json::Value =
        serde_json::from_reader(reader)
            .expect("JSON was not well-formatted");

    println!("{:?}", default_set);
}
