use axum::{Router, routing::get, extract::Path, Extension, response::IntoResponse, body::StreamBody, http::{HeaderMap, header, StatusCode}};
use sqlx::SqlitePool;
use tokio_util::io::ReaderStream;

use crate::{error::ResError, models::FileInfo};

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/files/:id", get(file_get).delete(file_delete))
        .layer(Extension(pool))
}

async fn file_get(
    Path(file_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>
) -> Result<impl IntoResponse, ResError> {
    let file_info = sqlx::query_as::<_, FileInfo>("SELECT * FROM files WHERE id = ?;")
        .bind(file_id)
        .fetch_one(&pool).await?;

    let file_path = format!("./files/{}", file_info.hash_name);
    let file = tokio::fs::File::open(file_path).await?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", file_info.file_name).parse()?,
    );

    Ok((headers, body))
}

// TODO: delete the file from disk and from the db
async fn file_delete(
    Path(file_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
