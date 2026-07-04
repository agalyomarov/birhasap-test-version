use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: u16,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum TodoStatus {
    Incomplete,
    Complete,
}
