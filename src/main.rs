pub mod models;
pub mod posts;
pub mod styles;

use askama::Template;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use styles::styles;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage;

async fn index() -> impl IntoResponse {
    IndexPage
}

#[derive(Clone)]
pub struct ServerState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/rust-htmx")
        .await
        .expect("Error connecting to postgres db");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Error running migrations");

    let state = ServerState { pool };

    let app = Router::new()
        .route("/", get(index))
        .route("/styles.css", get(styles))
        .route("/posts", get(posts::page))
        .route("/posts/:post_id", get(posts::post::page))
        .route("/replies/:parent_id", get(posts::post::replies))
        .route("/reply/:comment_id", post(posts::post::post_reply))
        .route("/reply-form/:comment_id", get(posts::post::reply_form))
        .with_state(state);

    let address: &str = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
