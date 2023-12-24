use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub content: String,
}

// SELECT
//     p.id, p.title, p.content,
//     u.user_name as author,
//     COALESCE(pl.likes, 0) AS likes
// FROM
//     posts p
// LEFT JOIN (
//     SELECT
//         post_id,
//         COUNT(*) AS likes
//     FROM
//         post_likes
//     GROUP BY
//         post_id
// ) pl ON p.id = pl.post_id
// LEFT JOIN users u ON p.user_id = u.id;
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct FullPost {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
    pub likes: i64,
}
