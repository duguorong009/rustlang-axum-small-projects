use axum::{response::Redirect, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/foo", get(redirect_handler))
        .route("/time", get(time_handler))
        .route("/time1", get(|| time_handler1("%d/%m/%Y %H:%M")));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn redirect_handler() -> Redirect {
    Redirect::temporary("http://example.org")
}

async fn time_handler() -> String {
    chrono::Local::now().format("%d/%m/%Y %H:%M").to_string()
}

async fn time_handler1(fmt: &str) -> String {
    chrono::Local::now().format(fmt).to_string()
}
