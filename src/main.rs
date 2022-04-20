mod structures;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use configparser::ini::Ini;
use serde_json;

const WSLCONF_LOC: &str = "wsl.conf";

fn get_default_set() -> structures::CoreDict {
    let default_file =
        File::open("src/default.json")
            .expect("failed to read file");
    let reader = BufReader::new(default_file);
    let default_set: structures::CoreDict =
        serde_json::from_reader(reader)
            .expect("JSON was not well-formatted");

    return default_set;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut conf = Ini::new();
    if Path::new(WSLCONF_LOC).exists(){
        conf.load(WSLCONF_LOC).unwrap();
    }

    let default_set = get_default_set();

    for section in default_set {
        let sect_name = section.0.as_str();
        let sect_opts = section.1;
        for item in sect_opts.options {
            let opt_name = item.0.as_str();
            println!("{} with {}", sect_name, opt_name);
        }
    }
}
