use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct FileInfo {
    pub id: i64,
    pub save_id: i64,
    pub hash_name: String,
    pub file_name: String,
    pub file_size: i64,
    pub created: DateTime<Utc>,
}
