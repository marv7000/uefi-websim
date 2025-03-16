use std::fs::{self, File};

use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;

mod config;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(landing))
        .route("/device/{device}", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8844").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn landing() -> Html<String> {
    println!("Landing page requested");
    let cfg_path = std::path::Path::new(".").join("cfg");
    println!("Config path: {:?}", cfg_path);

    let mut device_links = String::new();
    let files = fs::read_dir(&cfg_path).unwrap();

    println!("Reading directory: {:?}", cfg_path);
    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("Found file: {:?}", path);

        if path.is_file() && path.extension().map_or(false, |ext| ext == "yaml") {
            let file_name = entry.file_name();
            let p = file_name.to_str().unwrap().replace(".yaml", "");
            println!("Adding device: {}", p);
            device_links += format!("<a href=\"device/{}\">{}</a><br>", p, p).as_str();
        }
    }

    println!("Device links: {}", device_links);
    let template = templates::LandingTemplate { device_links };
    return Html(template.render().unwrap());
}

async fn root(Path(device): Path<String>) -> Response {
    // Find existing config file for the device.
    let file_path = std::path::Path::new(".").join("cfg").join(device + ".yaml");
    if file_path.try_exists().unwrap() {
        // If exists, parse the config and generate the page.
        let mut file = File::open(file_path).unwrap();
        let cfg = config::parse(&mut file);
        return Html(templates::generate_html(&cfg)).into_response();
    } else {
        return StatusCode::NOT_FOUND.into_response();
    }
}
