use std::fs;

use askama::Template;
use convert_case::{Case, Casing};

use crate::config::{Config, Node};

// SVG icons for categories
const SVG_ICONS: &[(&str, &str)] = &[
    (
        "home",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 2L2 12h3v8h14v-8h3L12 2zm0 15c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z"/></svg>"#,
    ),
    (
        "overclock",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 16c1.7 0 3-1.3 3-3s-1.3-3-3-3-3 1.3-3 3 1.3 3 3 3zm0-9c3.3 0 6 2.7 6 6s-2.7 6-6 6-6-2.7-6-6 2.7-6 6-6zm8 9c0 .6-.4 1-1 1h-2c-.6 0-1-.4-1-1s.4-1 1-1h2c.6 0 1 .4 1 1zM8 16c0 .6-.4 1-1 1H5c-.6 0-1-.4-1-1s.4-1 1-1h2c.6 0 1 .4 1 1zm9-9c.4.4.4 1 0 1.4l-1.4 1.4c-.4.4-1 .4-1.4 0-.4-.4-.4-1 0-1.4l1.4-1.4c.4-.4 1-.4 1.4 0zm-11.4 9.8c.4.4.4 1 0 1.4l-1.4 1.4c-.4.4-1 .4-1.4 0-.4-.4-.4-1 0-1.4l1.4-1.4c.4-.4 1-.4 1.4 0zM16 7c0-.6.4-1 1-1h2c.6 0 1 .4 1 1s-.4 1-1 1h-2c-.6 0-1-.4-1-1zM8 7c0-.6.4-1 1-1h2c.6 0 1 .4 1 1s-.4 1-1 1H9c-.6 0-1-.4-1-1zM7 8.4c-.4-.4-.4-1 0-1.4l1.4-1.4c.4-.4 1-.4 1.4 0 .4.4.4 1 0 1.4L8.4 8.4c-.4.4-1 .4-1.4 0zm9.8 9.8c-.4-.4-.4-1 0-1.4l1.4-1.4c.4-.4 1-.4 1.4 0 .4.4.4 1 0 1.4l-1.4 1.4c-.4.4-1 .4-1.4 0z"/></svg>"#,
    ),
    (
        "rgb",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 2c5.5 0 10 4.5 10 10s-4.5 10-10 10S2 17.5 2 12 6.5 2 12 2zm0 18c4.4 0 8-3.6 8-8s-3.6-8-8-8-8 3.6-8 8 3.6 8 8 8zm-5-8c0 2.8 2.2 5 5 5s5-2.2 5-5-2.2-5-5-5-5 2.2-5 5z"/></svg>"#,
    ),
    (
        "fan",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 11c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-2c2.2 0 4 1.8 4 4s-1.8 4-4 4-4-1.8-4-4 1.8-4 4-4zm-6 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm12-4c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zM6 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm12 0c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm-6-6c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2z"/></svg>"#,
    ),
    (
        "boot",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>"#,
    ),
];

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
    pub icon_html: String,
}

fn get_svg_icon(icon_name: &str) -> String {
    return fs::read_to_string(format!("icons/{}.svg", icon_name)).unwrap();
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
        Node::Category {
            name,
            icon: _,
            entries,
        } => {
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
        if let Node::Category {
            name,
            icon,
            entries: _,
        } = cat
        {
            let icon_html = if let Some(icon_name) = icon {
                if config.icons.as_ref().map_or(false, |i| i == "svg") {
                    get_svg_icon(icon_name)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let link_template = CategoryLinkTemplate {
                id: name.to_case(Case::Snake),
                name: name.clone(),
                icon_html: icon_html,
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

    // Add simulation JavaScript if enabled
    let simulation_js = if let Some(sim) = &config.simulation {
        if sim.enabled {
            // Generate JavaScript for dependencies
            let mut dependencies_js = String::new();
            for dep in &sim.dependencies {
                let affects_js = dep
                    .affects
                    .iter()
                    .map(|a| format!("'{}'", a))
                    .collect::<Vec<_>>()
                    .join(", ");

                let ranges_js = dep
                    .ranges
                    .iter()
                    .map(|r| format!("{{ min: {}, max: {}, value: {} }}", r.min, r.max, r.value))
                    .collect::<Vec<_>>()
                    .join(", ");

                dependencies_js.push_str(&format!(
                    "// Dependency: {}\nconst {}_affects = [{}];\nconst {}_ranges = [{}];\n",
                    dep.name,
                    dep.name.to_case(Case::Snake),
                    affects_js,
                    dep.name.to_case(Case::Snake),
                    ranges_js
                ));
            }

            let update_interval = sim.update_interval.to_string();
            let clock_enabled = sim.clock.to_string();
            let thermal_status = sim.thermal_status.to_string();
            let fan_status = sim.fan_status.to_string();

            format!(
                r#"
// Simulation functionality
const updateInterval = {update_interval};
let cpuTemp = parseInt(custom_variables['cpu_temperature']);
let sysTemp = parseInt(custom_variables['system_temperature']);

{dependencies_js}

// Helper function to get value based on ranges
function getValueFromRanges(ranges, value) {{
    for (const range of ranges) {{
        if (value >= range.min && value <= range.max) {{
            return range.value;
        }}
    }}
    return null; // Default if no range matches
}}

function updateSimulation() {{
    // Update clock
    if ({clock_enabled}) {{
        const now = new Date();
        const timeString = now.toLocaleTimeString();
        custom_variables['current_time'] = timeString;
        if (document.getElementById('current_time')) {{
            document.getElementById('current_time').textContent = timeString;
        }}
    }}

    // Update temperatures with random fluctuations
    if ({thermal_status}) {{
        // CPU temperature fluctuation
        const cpuChange = Math.floor(Math.random() * 3) - 1; // -1, 0, or 1
        cpuTemp = Math.max(30, Math.min(85, cpuTemp + cpuChange));
        custom_variables['cpu_temperature'] = cpuTemp + '°C';
        if (document.getElementById('cpu_temperature')) {{
            document.getElementById('cpu_temperature').textContent = cpuTemp + '°C';
        }}

        // System temperature fluctuation
        const sysChange = Math.floor(Math.random() * 3) - 1; // -1, 0, or 1
        sysTemp = Math.max(25, Math.min(75, sysTemp + sysChange));
        custom_variables['system_temperature'] = sysTemp + '°C';
        if (document.getElementById('system_temperature')) {{
            document.getElementById('system_temperature').textContent = sysTemp + '°C';
        }}
    }}

    // Update fan speeds based on temperatures
    if ({fan_status}) {{
        // CPU fan speed based on CPU temperature
        let cpuFanSpeed = 1200; // Default

        // Use dependency ranges if available
        if (typeof cpu_temperature_ranges !== 'undefined' && cpu_temperature_affects) {{
            const rangeValue = getValueFromRanges(cpu_temperature_ranges, cpuTemp);
            if (rangeValue) {{
                cpuFanSpeed = rangeValue;
            }}
        }} else {{
            // Fallback to hardcoded values
            if (cpuTemp >= 76) cpuFanSpeed = 3600;
            else if (cpuTemp >= 61) cpuFanSpeed = 2400;
            else if (cpuTemp >= 46) cpuFanSpeed = 1800;
        }}

        custom_variables['cpu_fan_speed'] = cpuFanSpeed + ' RPM';
        if (document.getElementById('cpu_fan_speed')) {{
            document.getElementById('cpu_fan_speed').textContent = cpuFanSpeed + ' RPM';
        }}

        // Chassis fans based on system temperature
        let chassisFanSpeed = 800; // Default

        // Use dependency ranges if available
        if (typeof system_temperature_ranges !== 'undefined' && system_temperature_affects) {{
            const rangeValue = getValueFromRanges(system_temperature_ranges, sysTemp);
            if (rangeValue) {{
                chassisFanSpeed = rangeValue;
            }}
        }} else {{
            // Fallback to hardcoded values
            if (sysTemp >= 61) chassisFanSpeed = 2400;
            else if (sysTemp >= 46) chassisFanSpeed = 1800;
            else if (sysTemp >= 36) chassisFanSpeed = 1200;
        }}

        custom_variables['chassis_fan_1_speed'] = chassisFanSpeed + ' RPM';
        if (document.getElementById('chassis_fan_1_speed')) {{
            document.getElementById('chassis_fan_1_speed').textContent = chassisFanSpeed + ' RPM';
        }}

        custom_variables['chassis_fan_2_speed'] = chassisFanSpeed + ' RPM';
        if (document.getElementById('chassis_fan_2_speed')) {{
            document.getElementById('chassis_fan_2_speed').textContent = chassisFanSpeed + ' RPM';
        }}
    }}

    setTimeout(updateSimulation, updateInterval);
}}

// Start simulation
console.log("Starting simulation...");
console.log("Update interval:", updateInterval);
console.log("Clock enabled:", {clock_enabled});
console.log("Thermal status enabled:", {thermal_status});
console.log("Fan status enabled:", {fan_status});
setTimeout(updateSimulation, 1000);
"#
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Determine layout class based on configuration
    let layout_class = if let Some(layout) = &config.layout {
        if layout == "horizontal" {
            "layout-horizontal"
        } else {
            "layout-vertical"
        }
    } else {
        "layout-vertical"
    };

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
    <div class="window {}">
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
    <script>{}{}{}</script>
</body>
</html>"#,
        css, layout_class, tabs, body, vars, js, simulation_js
    )
}
