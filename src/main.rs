pub mod models;
pub mod styles;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use models::{
    comments::{Comment, CommentWithHasChildren},
    posts::{FullPost, Post},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use styles::styles;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage;

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsPage {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostPage {
    post: FullPost,
    comments: Vec<CommentWithHasChildren>,
}

async fn index() -> impl IntoResponse {
    IndexPage
}

async fn posts_page(State(state): State<ServerState>) -> impl IntoResponse {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    PostsPage { posts }
}

#[derive(Template)]
#[template(path = "components/comments.html")]
struct CommentsTemplate {
    comments: Vec<CommentWithHasChildren>,
}

async fn replies(
    State(state): State<ServerState>,
    Path(parent_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let comments = sqlx::query_as::<_, CommentWithHasChildren>(
        format!(
            "
                SELECT c.*, 
                    (SELECT COUNT(*) > 0 AS has_children 
                        FROM comments cc 
                        WHERE cc.parent_id = c.id
                    ) 
                FROM comments c
                WHERE parent_id = '{}';
            ",
            parent_id
        )
        .as_str(),
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    CommentsTemplate { comments }
}

async fn post_page(
    State(state): State<ServerState>,
    Path(post_id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let post = sqlx::query_as::<_, FullPost>(
        format!(
            "
            SELECT
                p.id, p.title, p.content,
                u.user_name as author,
                COALESCE(pl.likes, 0) AS likes
            FROM
                posts p
            
            LEFT JOIN (
                SELECT
                    post_id,
                    COUNT(*) AS likes
                FROM
                    post_likes
                WHERE
                    post_id = '{}'
                GROUP BY
                    post_id
            ) pl ON p.id = pl.post_id
            LEFT JOIN users u ON p.user_id = u.id
            WHERE
                p.id = '{}'
        ",
            post_id, post_id
        )
        .as_str(),
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let comments = sqlx::query_as::<_, CommentWithHasChildren>(
        format!(
            "
                SELECT 
                    c.*, 
                    (SELECT COUNT(*) > 0 AS has_children 
                        FROM comments cc 
                        WHERE cc.parent_id = c.id
                    ) 
                FROM comments c
                WHERE parent_id IS NULL
                AND post_id = '{}'
            ",
            post_id
        )
        .as_str(),
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    PostPage { post, comments }
}

#[derive(Clone)]
struct ServerState {
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
        .route("/posts", get(posts_page))
        .route("/posts/:post_id", get(post_page))
        .route("/replies/:parent_id", get(replies))
        .with_state(state);

    let address: &str = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
