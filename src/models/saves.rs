use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use super::FileInfo;

#[derive(Debug, Deserialize)]
pub struct SavePatch {
    pub text: String,
    pub caption: String,
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
