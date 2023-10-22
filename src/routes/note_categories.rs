use axum::{Router, routing::get, Extension, response::IntoResponse, http::StatusCode};
use sqlx::SqlitePool;

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/note-categories", get(note_categories_get).post(note_categories_post))
        .route("/note-categories/:id", get(note_category_get).patch(note_category_patch).delete(note_category_delete))
        .layer(Extension(pool))
}

async fn note_categories_get(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn note_categories_post(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn note_category_get(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn note_category_patch(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn note_category_delete(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
