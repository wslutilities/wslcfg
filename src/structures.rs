use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct WSLOptions {
    _friendly_name: String,
    default: String,
    def: String,
    tip: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WSLSections {
    _friendly_name: String,
    pub(crate) options: HashMap<String, WSLOptions>
}

pub type CoreDict = HashMap<String, WSLSections>;
