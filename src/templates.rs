use askama::Template;
use convert_case::{Case, Casing};

use crate::config::{Config, Node};

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate {
    pub device_links: String,
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate {
    pub css: String,
    pub tabs: String,
    pub body: String,
    pub vars: String,
    pub js: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub table_rows: String,
}

#[derive(Template)]
#[template(path = "home_table.html")]
pub struct HomeTableRowTemplate {
    pub var_name: String,
    pub var_id: String,
    pub var_value: String,
}

#[derive(Template)]
#[template(path = "category.html")]
pub struct CategoryTemplate {
    pub id: String,
    pub title: String,
    pub nodes: String,
}

#[derive(Template)]
#[template(path = "menu.html")]
pub struct MenuTemplate {
    pub id: String,
    pub title: String,
    pub nodes: String,
}

#[derive(Template)]
#[template(path = "dropdown.html")]
pub struct DropdownTemplate {
    pub name: String,
    pub id: String,
    pub title: String,
    pub options: String,
}

#[derive(Template)]
#[template(path = "dropdown_opt.html")]
pub struct DropdownOptionTemplate {
    pub value: String,
    pub selected: String,
    pub display: String,
}

#[derive(Template)]
#[template(path = "switch.html")]
pub struct SwitchTemplate {
    pub name: String,
    pub id: String,
    pub checked: String,
}

#[derive(Template)]
#[template(path = "category_link.html")]
pub struct CategoryLinkTemplate {
    pub id: String,
    pub name: String,
}

pub fn render_node(node: &Node) -> String {
    match node {
        Node::Home { vars } => {
            let mut table_rows = String::new();
            for var in vars {
                let template = HomeTableRowTemplate {
                    var_name: var.clone(),
                    var_id: var.to_case(Case::Snake),
                    var_value: "default".to_string(),
                };
                table_rows += &template.render().unwrap();
            }

            let home_template = HomeTemplate { table_rows };
            home_template.render().unwrap()
        }
        Node::Category { name, entries } => {
            let template = CategoryTemplate {
                id: name.to_case(Case::Snake),
                title: name.clone(),
                nodes: entries.iter().map(|x| render_node(x)).collect(),
            };
            template.render().unwrap()
        }
        Node::Menu { name, entries } => {
            let template = MenuTemplate {
                id: name.to_case(Case::Snake),
                title: name.clone(),
                nodes: entries.iter().map(|x| render_node(x)).collect(),
            };
            template.render().unwrap()
        }
        Node::Dropdown {
            name,
            desc: _,
            default,
            values,
        } => {
            let mut options = String::new();
            for value in values {
                let mut selected = "";
                if let Some(def) = default {
                    if value == def {
                        selected = "selected";
                    }
                }

                let option_template = DropdownOptionTemplate {
                    value: value.clone(),
                    selected: selected.to_string(),
                    display: value.clone(),
                };
                options += &option_template.render().unwrap();
            }

            let dropdown_template = DropdownTemplate {
                name: name.clone(),
                id: name.to_case(Case::Snake),
                title: name.clone(),
                options,
            };
            dropdown_template.render().unwrap()
        }
        Node::Switch {
            name,
            desc: _,
            default,
        } => {
            let mut checked = "";
            if let Some(def) = default {
                if *def {
                    checked = "checked";
                }
            }

            let switch_template = SwitchTemplate {
                name: name.clone(),
                id: name.to_case(Case::Snake),
                checked: checked.to_string(),
            };
            switch_template.render().unwrap()
        }
        Node::Textbox {
            name: _,
            desc: _,
            default: _,
        } => todo!("textbox"),
        Node::Date {
            name: _,
            desc: _,
            default: _,
        } => todo!("date"),
        Node::Time {
            name: _,
            desc: _,
            default: _,
        } => todo!("time"),
    }
}

pub fn generate_html(config: &Config) -> String {
    let mut body = String::new();
    let mut tabs = String::new();

    for cat in &config.categories {
        if let Node::Category { name, entries: _ } = cat {
            let link_template = CategoryLinkTemplate {
                id: name.to_case(Case::Snake),
                name: name.clone(),
            };
            tabs += &link_template.render().unwrap();
            body += &render_node(cat);
        }
    }

    let mut vars = "let custom_variables = {};".to_owned();
    for (key, value) in &config.vars {
        vars += &format!(
            "custom_variables['{}'] = '{}';\n",
            key.to_case(Case::Snake),
            value
        );
    }

    // Read style HTML and append to result.
    let mut style_html = std::fs::File::open(format!("styles/{}.html", config.style)).unwrap();
    std::io::Read::read_to_string(&mut style_html, &mut body).unwrap();

    // Read style CSS and append to result.
    let mut css = String::new();
    let mut style_css = std::fs::File::open(format!("styles/{}.css", config.style)).unwrap();
    std::io::Read::read_to_string(&mut style_css, &mut css).unwrap();

    // Read main.js
    let js = include_str!("main.js");

    // Manually create the HTML instead of using the template
    // This avoids issues with the editor reformatting the template syntax
    format!(
        r#"<!doctype html>
<html>
<head>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>{}</style>
</head>
<body>
    <div class="window">
        <div class="tab">{}</div>
        {}
    </div>
    <div class="config_panel">
        <form onsubmit="import_vars(event)" id="submitter">
            <input type="file" name="filename" accept="application/yaml">
            <input type="submit" value="Import Configuration">
        </form>
        <form onsubmit="download('export.yaml', export_vars())">
            <input type="submit" value="Export Configuration">
        </form>
    </div>
    <script>{}{}</script>
</body>
</html>"#,
        css, tabs, body, vars, js
    )
}
