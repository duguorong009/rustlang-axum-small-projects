use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, BoxError, ServiceBuilder};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
    );

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> &'static str {
    "Hello, world!"
}
