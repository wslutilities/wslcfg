use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct WSLOptions {
    pub _friendly_name: String,
    pub default: String,
    pub def: String,
    pub tip: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WSLSections {
    pub _friendly_name: String,
    pub(crate) options: HashMap<String, WSLOptions>
}

pub type CoreDict = HashMap<String, WSLSections>;
