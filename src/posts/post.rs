use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

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
