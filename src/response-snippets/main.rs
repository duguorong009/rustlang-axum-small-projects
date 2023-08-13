use axum::{
    http::{header, HeaderMap},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /header` goes to `send_only_headers`
        .route("/header", get(send_only_headers))
        // `GET /text` goes to `render_plain_text`
        .route("/text", get(render_plain_text))
        // `GET /json` goes to `render_json`
        .route("/json", get(render_json))
        // `GET /xml` goes to `render_xml`
        .route("/xml", get(render_xml));

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn send_only_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "A rust(axum) web server".parse().unwrap());
    headers
}

async fn render_plain_text() -> String {
    "RENDER_TEXT_OK".to_string()
}

#[derive(Debug, Serialize)]
struct Profile {
    name: String,
    hobbies: Vec<String>,
}

async fn render_json() -> Json<Profile> {
    let profile = Profile {
        name: "Alex".to_string(),
        hobbies: vec!["snowboarding".to_string(), "programming".to_string()],
    };

    Json(profile)
}

async fn render_xml() -> impl IntoResponse {
    let profile = Profile {
        name: "Alex".to_string(),
        hobbies: vec!["snowboarding".to_string(), "programming".to_string()],
    };
    let xml_content = serde_xml_rs::to_string(&profile).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());

    (headers, xml_content)
}
