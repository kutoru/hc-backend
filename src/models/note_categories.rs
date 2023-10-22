use chrono::{DateTime, Utc};
use serde::Serialize;
use super::Note;

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
