use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Deserialize)]
pub struct NewSave {
    pub text: String,
    pub caption: String,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Save {
    pub id: i64,
    pub text: String,
    pub caption: String,
    pub filename: Option<String>,
    pub created: DateTime<Utc>,
}
