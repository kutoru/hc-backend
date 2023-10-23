use axum::{http::StatusCode, Router, routing::{post, patch}, Extension, Json, extract::Path};
use sqlx::SqlitePool;

use crate::{models::{NotePost, Note}, res_body, res::*};

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/notes", post(notes_post))
        .route("/notes/:id", patch(note_patch).delete(note_delete))
        .layer(Extension(pool))
}

async fn notes_post(
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<NotePost>,
) -> ServerResult<Note> {

    let note_id = sqlx::query("INSERT INTO notes (category_id, title, text) VALUES (?, ?, ?);")
        .bind(body.category_id).bind(body.title).bind(body.text)
        .execute(&pool).await?
        .last_insert_rowid();

    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?;")
        .bind(note_id)
        .fetch_one(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, Some(note))))
}

async fn note_patch(
    Path(note_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<NotePost>,
) -> ServerResult<Note> {

    sqlx::query("UPDATE notes SET category_id = ?, title = ?, text = ? WHERE id = ?;")
        .bind(body.category_id).bind(body.title).bind(body.text).bind(note_id)
        .execute(&pool).await?;

    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?;")
        .bind(note_id)
        .fetch_one(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, Some(note))))
}

async fn note_delete(
    Path(note_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>
) -> ServerResult<()> {

    sqlx::query("DELETE FROM notes WHERE id = ?;")
        .bind(note_id)
        .execute(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, None)))
}
