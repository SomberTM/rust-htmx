use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::models::posts::Post;
use crate::ServerState;

pub mod post;

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsPage {
    posts: Vec<Post>,
}

pub async fn page(State(state): State<ServerState>) -> impl IntoResponse {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    PostsPage { posts }
}
