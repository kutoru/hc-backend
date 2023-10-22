use axum::{response::IntoResponse, http::StatusCode, Router, routing::{post, patch}, Extension};
use sqlx::SqlitePool;

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/notes", post(notes_post))
        .route("/notes/:id", patch(note_patch).delete(note_delete))
        .layer(Extension(pool))
}

async fn notes_post(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn note_patch(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn note_delete(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
