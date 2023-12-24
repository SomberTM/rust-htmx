use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub post_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub content: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct CommentWithHasChildren {
    pub id: uuid::Uuid,
    pub post_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub content: String,
    pub has_children: bool,
}
