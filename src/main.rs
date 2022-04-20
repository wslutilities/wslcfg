mod structures;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use configparser::ini::Ini;
use serde_json;

#[cfg(not(debug_assertions))]
const WSLCONF_LOC: &str = "/etc/wsl.conf";

#[cfg(debug_assertions)]
const WSLCONF_LOC: &str = "tests/wsl.conf";

fn get_default_set() -> structures::CoreDict {
    let default_file =
        File::open("ext/default.json")
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
    let default_set = get_default_set();

    for section in default_set {
        let sect_name = section.0.as_str();
        for item in section.1.options {
            let opt_name = item.0.as_str();
            let opt_var = item.1.default.as_str();
            conf.setstr(sect_name, opt_name, Some(opt_var));
        }
    }

    if Path::new(WSLCONF_LOC).exists(){
        conf.load(WSLCONF_LOC).unwrap();
    }

    for (section, options) in conf.get_map().unwrap() {
        let sect_name = section;
        for (item_name, item_var) in options {
            println!("{} - {}: {}", sect_name, item_name, item_var.unwrap());
        }
    }

    //This is used for getting type and things
}
