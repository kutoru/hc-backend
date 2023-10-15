use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Deserialize)]
pub struct NewSaveFile {
    pub save_id: i64,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct NewSave {
    pub text: String,
    pub caption: String,
    pub files: Option<Vec<NewSaveFile>>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SaveFile {
    pub uuid: String,
    pub save_id: i64,
    pub filename: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Save {
    pub id: i64,
    pub text: String,
    pub caption: String,
    pub created: DateTime<Utc>,
}

pub struct SaveWithFiles {
    pub id: i64,
    pub text: String,
    pub caption: String,
    pub created: DateTime<Utc>,
    pub files: Option<Vec<SaveFile>>,
}
