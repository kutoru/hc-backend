use serde:: Serialize;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct FileInfo {
    pub id: i64,
    pub save_id: i64,
    pub hash_name: String,
    pub file_name: String,
    pub file_size: i64,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Save {
    pub id: i64,
    pub text: String,
    pub caption: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SaveWithFiles {
    pub id: i64,
    pub text: String,
    pub caption: String,
    pub created: DateTime<Utc>,
    pub files: Vec<FileInfo>,
}
