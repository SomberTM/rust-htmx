use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    id: uuid::Uuid,
    user_name: String,
    email: String,
    password_hash: String,
}
