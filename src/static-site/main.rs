use axum::{extract::Path, response::Html, routing::get, Router};
use minijinja::render;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Items {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct Profile {
    full_name: String,
    items: Vec<Items>,
}

// basic handler that responds with a static string
async fn root() -> Html<&'static str> {
    Html("Hello, World!")
}

// html template handler
async fn get_profile(Path(profile_name): Path<String>) -> Html<String> {
    const PROFILE_TEMPLATE: &'static str = r#"
    <!doctype html>

    <html lang="en">
    <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <title>A Basic HTML5 Template</title>
    <meta name="description" content="A basic HTML5 Template for new projects.">
    <meta name="author" content="Woile">
    </head>

    <body>
        <h1>Profile of {{ profile.full_name|title }}</h1>
        <p>This is a template example to show some functionality</p>
        <h2>Items</h3>
        <ul>
            {% for item in profile.items %}
            <li>{{ item.name }} ({{ item.id }})</li>
            {% endfor %}
        <ul>
    </body>
    </html>
    "#;

    let orders_example = vec![
        Items {
            id: 1,
            name: "Article banana".into(),
        },
        Items {
            id: 2,
            name: "Article apple".into(),
        },
    ];
    let profile_example = Profile {
        full_name: profile_name,
        items: orders_example,
    };
    let r = render!(PROFILE_TEMPLATE, profile => profile_example);

    Html(r)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/:profile_name", get(get_profile));

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
