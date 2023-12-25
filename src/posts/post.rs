use askama::Template;
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::models::{comments::CommentWithHasChildren, posts::FullPost};
use crate::ServerState;

#[derive(Template)]
#[template(path = "post.html")]
struct PostPage {
    post: FullPost,
    comments: Vec<CommentWithHasChildren>,
}

pub async fn page(
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

#[derive(Template)]
#[template(path = "components/comments.html")]
struct CommentsTemplate {
    comments: Vec<CommentWithHasChildren>,
}

pub async fn replies(
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

#[derive(Template)]
#[template(path = "components/reply.html")]
struct ReplyFormTemplate {
    comment_id: uuid::Uuid,
}

pub async fn reply_form(Path(comment_id): Path<uuid::Uuid>) -> impl IntoResponse {
    ReplyFormTemplate { comment_id }
}

#[derive(Deserialize)]
pub struct PostReply {
    content: String,
}

#[derive(sqlx::FromRow)]
struct PostId {
    post_id: uuid::Uuid,
}

pub async fn post_reply(
    State(state): State<ServerState>,
    Path(comment_id): Path<uuid::Uuid>,
    Json(reply): Json<PostReply>,
) -> impl IntoResponse {
    let post_id = sqlx::query_as::<_, PostId>(
        format!(
            "
            SELECT post_id FROM comments
            WHERE id = '{}'
        ",
            comment_id
        )
        .as_str(),
    )
    .fetch_one(&state.pool)
    .await
    .unwrap()
    .post_id;

    sqlx::query(
        format!(
            "
                INSERT INTO comments (parent_id, post_id, user_id, content)
                VALUES ('{}', '{}', '0dcf059d-a74c-4fdc-8110-0b3646fc1cb7', '{}')
            ",
            comment_id, post_id, reply.content
        )
        .as_str(),
    )
    .execute(&state.pool)
    .await
    .unwrap();

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
            comment_id
        )
        .as_str(),
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    CommentsTemplate { comments }
}
