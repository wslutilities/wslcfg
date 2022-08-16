mod structures;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use configparser::ini::Ini;

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

    default_set
}


fn interact(cmd: &str, _args: Vec<String>) {
    let mut c = Ini::new();

    for section in get_default_set() {
        let sect_name = section.0.as_str();
        for item in section.1.options {
            let opt_name = item.0.as_str();
            let opt_var = item.1.default.as_str();
            c.setstr(sect_name, opt_name, Some(opt_var));
        }
    }

    if Path::new(WSLCONF_LOC).exists(){
        c.load(WSLCONF_LOC).unwrap();
    }

    match cmd {
        "help"=>{
            if _args.len() == 3 {
                let tmp_name = _args[2].as_str();
                let tmp_name_set: Vec<&str> = tmp_name.split('.').collect();
                if tmp_name_set.len() != 2{
                    println!("ERROR: Not a valid settings name");
                    return;
                }
                let config_name = tmp_name_set[0];
                let options_name = tmp_name_set[1];
                for section in get_default_set() {
                    let sect_name = section.0.as_str();
                    if sect_name == config_name {
                        for item in section.1.options {
                            let opt_name = item.0.as_str();
                            if opt_name == options_name {

                                println!("{}.{} ({})", config_name, options_name,
                                         item.1.def);
                                println!("Friendly Name: {} > {}",
                                         section.1._friendly_name,
                                         item.1._friendly_name);
                                println!("Default: {}\n", item.1.default);
                                println!("{}", item.1.tip);
                                return;
                            }
                        }
                    }
                }
                println!("ERROR: Not a valid settings name");
            } else {
                println!("wslcfg [list|set|reset|get]");
            }

        },
        "list"=> {
            for (section, options) in c.get_map().unwrap() {
                let sect_name = section;
                for (item_name, item_var) in options {
                    println!("{}.{}: {}", sect_name, item_name, item_var.unwrap());
                }
            }
        },
/*        "get" => {
            let value = conf.get(section, option).unwrap();
            if valueOnly {
                println!("{}", value);
            } else {
                println!("{}.{}: {}", section, option, value);
            }
        },*/
        _=> {
            println!("Unknown command");
            println!("wslcfg [list|set|reset|get]");
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let cmd = args[1].as_str();

        match cmd{
            "version"=>{
                println!("wslcfg v{}", env!("CARGO_PKG_VERSION"));
            },
            _=>{
                interact(cmd, args.clone());
            }
        }
    } else {
        println!("wslcfg [list|set|reset|get]");
    }
    //This is used for getting type and things
}
