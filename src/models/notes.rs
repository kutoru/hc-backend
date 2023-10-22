use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Note {
    pub id: i64,
    pub category_id: i64,
    pub title: String,
    pub text: String,
    pub created: DateTime<Utc>,
}
