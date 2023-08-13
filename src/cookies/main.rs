use axum::{
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose, Engine as _};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/set", get(set_cookie_handler))
        .route("/get", get(get_cookie_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> &'static str {
    "Hello world!"
}

// TODO: Try to implement the HMAC signature, AES-GCM signature, etc
async fn set_cookie_handler() -> impl IntoResponse {
    let cookie_name = "foo";
    let cookie_value = "Hello, Zoë!";

    // base64 encoding
    let encoded_value = general_purpose::URL_SAFE.encode(cookie_value);

    let cookie = format!("{}={}", cookie_name, encoded_value);

    (HeaderMap::new(), AppendHeaders([(SET_COOKIE, cookie)]))
}

async fn get_cookie_handler(jar: CookieJar) -> Result<String, StatusCode> {
    let cookie_name = "foo";
    let cookie = jar.get(cookie_name);
    let cookie_value = match cookie {
        None => return Err(StatusCode::BAD_REQUEST),
        Some(c) => c.value().to_string(),
    };

    let decoded_value = match general_purpose::URL_SAFE.decode(cookie_value) {
        Ok(v) => String::from_utf8(v).expect("Non-utf8 value"),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(decoded_value)
}
