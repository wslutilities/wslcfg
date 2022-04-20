#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! serde = { version = "1.0.136", features = ["derive"] }
//! serde_json = "1.0.59"
//! configparser = "3.0.0"
//! ```
extern crate serde;
extern crate serde_json;
extern crate configparser;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use serde::Serialize;
use configparser::ini::Ini;

#[derive(Serialize, Deserialize, Debug)]
struct WSLOptions {
    _friendly_name: String,
    default: String,
    def: String,
    tip: String
}

#[derive(Serialize, Deserialize, Debug)]
struct WSLSections {
    _friendly_name: String,
    options: HashMap<String, WSLOptions>
}

type CoreDict = HashMap<String, WSLSections>;

fn main() {
    let default_file =
        File::open("ext/default.json")
            .expect("failed to read file");
    let reader = BufReader::new(default_file);
    let default_set: CoreDict =
        serde_json::from_reader(reader)
            .expect("JSON was not well-formatted");

    let mut config = Ini::new();

    for section in default_set {
        let sect_name = section.0.as_str();
        for item in section.1.options {
            let opt_name = item.0.as_str();
            let opt_var = item.1.default.as_str();
            config.setstr(sect_name, opt_name, Some(opt_var));
        }
    }

    config.write("ext/default.conf").expect("Failed to write to the file");

}