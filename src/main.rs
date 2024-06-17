use std::fs::{self, File};

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;

mod config;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(landing))
        .route("/device/:device", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8844").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn landing() -> Html<String> {
    let cfg_path = std::path::Path::new(".").join("cfg");
    let mut s = String::new();
    let files = fs::read_dir(cfg_path).unwrap();
    files.for_each(|x| {
        let binding = x.unwrap().file_name();
        let p = binding.to_str().unwrap().replace(".yaml", "");
        s += format!("<a href=\"device/{}\">{}</a>", p, p).as_str();
    });
    let html = format!(include_str!("html/landing.html.in"), s);
    return Html(html);
}

async fn root(Path(device): Path<String>) -> Response {
    // Find existing config file for the device.
    let file_path = std::path::Path::new(".").join("cfg").join(device + ".yaml");
    if file_path.try_exists().unwrap() {
        // If exists, parse the config and generate the page.
        let mut file = File::open(file_path).unwrap();
        let cfg = config::parse(&mut file);
        return Html(config::gen_html(&cfg)).into_response();
    } else {
        return StatusCode::NOT_FOUND.into_response();
    }
}
