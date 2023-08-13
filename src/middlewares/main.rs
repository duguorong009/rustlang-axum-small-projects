use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::get,
    Router, ServiceExt,
};
use tower::Layer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    // this can be any `tower::Layer`
    let middleware = axum::middleware::from_fn(enforce_json_handler);

    // apply the layer around the whole `Router`
    // this way the middleware will run before `Router` receives the request
    let app_with_middleware = middleware.layer(app);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app_with_middleware.into_make_service())
        .await
        .unwrap()
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn enforce_json_handler<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let content_type = req.headers().get("Content-Type");

    match content_type {
        None => return Err(StatusCode::BAD_REQUEST),
        Some(v) => {
            if v != "application/json" {
                return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
            }
        }
    };

    Ok(next.run(req).await)
}
