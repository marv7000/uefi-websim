use serde::Deserialize;
use std::{collections::HashMap, io::Read};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub style: String,
    pub categories: Vec<Node>,
    pub vars: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Node {
    Home {
        vars: Vec<String>,
    },
    Category {
        name: String,
        entries: Vec<Node>,
    },
    Menu {
        name: String,
        entries: Vec<Node>,
    },
    Dropdown {
        name: String,
        desc: Option<String>,
        default: Option<String>,
        values: Vec<String>,
    },
    Switch {
        name: String,
        desc: Option<String>,
        default: Option<bool>,
    },
    Textbox {
        name: String,
        desc: Option<String>,
        default: Option<String>,
    },
    Date {
        name: String,
        desc: Option<String>,
        default: Option<String>,
    },
    Time {
        name: String,
        desc: Option<String>,
        default: Option<String>,
    },
}

pub fn parse(file: &mut std::fs::File) -> Config {
    // Read file into string.
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    serde_yaml::from_str(&buf).unwrap()
}
