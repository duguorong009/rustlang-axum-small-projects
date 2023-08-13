use std::{net::SocketAddr, path::PathBuf};

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;

#[tokio::main]
async fn main() {
    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("localhost.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("localhost-key.pem"),
    )
    .await
    .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/unprotected", get(unprotected_handler))
        .route("/protected", get(protected_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn protected_handler() -> &'static str {
    "This is the protected handler"
}

async fn unprotected_handler() -> &'static str {
    "This is the unprotected handler"
}
