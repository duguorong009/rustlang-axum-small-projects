use axum::{
    extract::{self, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bigdecimal::ToPrimitive;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::BigDecimal, Pool, Postgres};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = create_db_pool().await.expect("DB connect error");

    let app = Router::new()
        .route("/", get(root))
        .route("/books", get(get_books))
        .route("/books/create", post(create_book))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[derive(Debug, Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

async fn root(State(_): State<AppState>) -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    price: f64,
}

#[derive(Deserialize)]
struct Params {
    isbn: String,
}

async fn get_books(
    params: Option<Query<Params>>,
    State(state): State<AppState>,
) -> Json<Vec<Book>> {
    match params {
        None => {
            let res = sqlx::query!("SELECT * FROM books")
                .fetch_all(&state.pool)
                .await
                .unwrap();

            let books: Vec<Book> = res
                .into_iter()
                .map(|b| Book {
                    isbn: b.isbn,
                    title: b.title,
                    author: b.author,
                    price: b.price.to_f64().unwrap_or_default(),
                })
                .collect();

            Json(books)
        }
        Some(p) => {
            let res = sqlx::query!("SELECT * FROM books WHERE isbn = $1", p.isbn)
                .fetch_all(&state.pool)
                .await
                .unwrap();

            let books: Vec<Book> = res
                .into_iter()
                .map(|b| Book {
                    isbn: b.isbn,
                    title: b.title,
                    author: b.author,
                    price: b.price.to_f64().unwrap_or_default(),
                })
                .collect();

            Json(books)
        }
    }
}

async fn create_book(
    State(state): State<AppState>,
    extract::Json(b): extract::Json<Book>,
) -> StatusCode {
    let res = sqlx::query!(
        "INSERT INTO books VALUES($1, $2, $3, $4)",
        b.isbn,
        b.title,
        b.author,
        BigDecimal::try_from(b.price).unwrap_or_default(),
    )
    .execute(&state.pool)
    .await
    .unwrap();

    if res.rows_affected() == 0 {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::CREATED
}

async fn create_db_pool() -> Result<AppState, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    Ok(AppState { pool })
}
