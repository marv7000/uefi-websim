use serde::Deserialize;
use std::{collections::HashMap, io::Read};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub style: String,
    pub layout: Option<String>,
    pub icons: Option<String>,
    pub simulation: Option<Simulation>,
    pub categories: Vec<Node>,
    pub vars: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Simulation {
    pub enabled: bool,
    pub update_interval: u32,
    pub clock: bool,
    pub cpu_status: bool,
    pub fan_status: bool,
    pub thermal_status: bool,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub affects: Vec<String>,
    pub ranges: Vec<Range>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Range {
    pub min: u32,
    pub max: u32,
    pub value: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Node {
    Home {
        vars: Vec<String>,
    },
    Category {
        name: String,
        icon: Option<String>,
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
