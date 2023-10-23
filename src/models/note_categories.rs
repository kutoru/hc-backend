use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::Note;

#[derive(Debug, Deserialize)]
pub struct NoteCategoryPost {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct NoteCategory {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NoteCategoryWithNotes {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct NoteCategoryWithCounts {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub note_count: i64,
}
