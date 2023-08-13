use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let conn: Connection = redis_connect().expect("redis error");
    let state = AppState {
        conn: Arc::new(Mutex::new(conn)),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/album", get(show_album))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn root(_: State<AppState>) -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Serialize)]
struct Album {
    title: String,
    artist: String,
    price: f64,
    likes: i32,
}

#[derive(Clone)]
struct AppState {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Deserialize)]
struct Params {
    id: usize,
}

async fn show_album(Query(p): Query<Params>, State(state): State<AppState>) -> Json<Album> {
    let mut redis_conn = state.conn.lock().unwrap();

    let title: String = redis_conn
        .hget(format!("album:{}", p.id).as_str(), "title")
        .expect("cannot get title");
    let artist: String = redis_conn
        .hget(format!("album:{}", p.id).as_str(), "artist")
        .expect("cannot get artist");
    let price: f64 = redis_conn
        .hget(format!("album:{}", p.id).as_str(), "price")
        .expect("cannot get price");
    let likes: i32 = redis_conn
        .hget(format!("album:{}", p.id).as_str(), "likes")
        .expect("cannot get likes");

    Json(Album {
        title,
        artist,
        price,
        likes,
    })
}

fn redis_connect() -> redis::RedisResult<Connection> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let conn = client.get_connection()?;

    Ok(conn)
}
