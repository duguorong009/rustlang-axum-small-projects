use axum::{http::Request, middleware::Next, response::Response, routing::get, Router, ServiceExt};
use tower::Layer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let m1 = axum::middleware::from_fn(middleware_1);
    let m2 = axum::middleware::from_fn(middleware_2);

    let app_with_middleware = m1.layer(m2.layer(app));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app_with_middleware.into_make_service())
        .await
        .unwrap()
}

async fn root() -> &'static str {
    println!("Executing finalHandler");
    "Hello, world!"
}

async fn middleware_1<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("Executing middlewareOne");
    let res = next.run(req).await;
    println!("Executing middlewareOne again");

    res
}

async fn middleware_2<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("Executing middlewareTwo");
    let res = next.run(req).await;
    println!("Executing middlewareTwo again");

    res
}
