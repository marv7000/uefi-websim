use convert_case::{Case, Casing};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read};

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

fn node_html(node: &Node) -> String {
    match node {
        Node::Home { vars } => {
            let mut t_elems = String::new();
            for var in vars {
                t_elems += format!(
                    include_str!("html/home_table.html.in"),
                    var,
                    var.to_case(Case::Snake),
                    "default"
                )
                .as_str();
            }
            format!(include_str!("html/home.html.in"), t_elems)
        }
        Node::Category { name, entries } => {
            format!(
                include_str!("html/category.html.in"),
                name.to_case(Case::Snake), // ID
                name,                      // Title
                entries.iter().map(|x| node_html(x)).collect::<String>()  // Nodes
            )
        }
        Node::Menu { name, entries } => {
            format!(
                include_str!("html/menu.html.in"),
                name.to_case(Case::Snake), // ID
                name,                      // Title
                entries.iter().map(|x| node_html(x)).collect::<String>()  // Nodes
            )
        }
        Node::Dropdown {
            name,
            desc,
            default,
            values,
        } => {
            let mut val_list = String::new();
            values.iter().for_each(|x| {
                let mut attr = "";
                // If we have a default value set, mark that entry with "selected"
                if let Some(def) = default {
                    if x == def {
                        attr = "selected";
                    }
                }
                val_list += format!(include_str!("html/dropdown_opt.html.in"), x, attr, x).as_str()
            });

            format!(
                include_str!("html/dropdown.html.in"),
                name,
                name.to_case(Case::Snake), // ID
                name,
                val_list
            )
        }
        Node::Switch {
            name,
            desc,
            default,
        } => {
            let mut attr = "";
            if let Some(def) = default {
                if *def == true {
                    attr = "checked";
                }
            }
            format!(
                include_str!("html/switch.html.in"),
                name,
                name.to_case(Case::Snake), // ID
                attr
            )
        }
        Node::Textbox {
            name,
            desc,
            default,
        } => todo!("textbox"),
        Node::Date {
            name,
            desc,
            default,
        } => todo!("date"),
        Node::Time {
            name,
            desc,
            default,
        } => todo!("time"),
    }
}

pub fn gen_html(config: &Config) -> String {
    let mut body = String::new();
    let mut tabs = String::new();

    for cat in &config.categories {
        if let Node::Category { name, entries: _ } = cat {
            tabs += format!(
                include_str!("html/category_link.html.in"),
                name.to_case(Case::Snake),
                name
            )
            .as_str();
            body += node_html(cat).as_str();
        }
    }

    let mut vars = "let custom_variables = {};".to_owned();
    for (key, value) in &config.vars {
        vars += format!(
            "custom_variables['{}'] = '{}';\n",
            key.to_case(Case::Snake),
            value
        )
        .as_str();
    }

    // Read style HTML and append to result.
    let mut style_html = File::open(format!("styles/{}.html", config.style)).unwrap();
    style_html.read_to_string(&mut body).unwrap();

    // Read style CSS and append to result.
    let mut css = String::new();
    let mut style_css = File::open(format!("styles/{}.css", config.style)).unwrap();
    style_css.read_to_string(&mut css).unwrap();

    return format!(
        include_str!("html/main.html.in"),
        css,
        tabs,
        body,
        vars,
        include_str!("html/main.js")
    );
}
