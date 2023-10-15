use axum::{Router, routing::get, response::IntoResponse, http::StatusCode, Json, Extension};
use sqlx::SqlitePool;
use crate::{models::saves::{NewSave, Save}, res_body, models::res::ServerResult};

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/saves", get(saves_get).post(saves_post))
        .layer(Extension(pool))
}

async fn saves_get(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    let list = sqlx::query_as::<_, Save>("SELECT * FROM saves;")
        .fetch_all(&pool).await.unwrap();

    (StatusCode::OK, res_body!(true, None, Some(list)))
}

async fn saves_post(
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<NewSave>,
) -> impl IntoResponse {
    let result = sqlx::query("INSERT INTO saves (text, caption, filename) VALUES (?, ?, ?);")
        .bind(body.text).bind(body.caption).bind(body.filename)
        .execute(&pool).await.unwrap();

    if result.rows_affected() < 1 {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            res_body!(true, Some("Could not insert any rows"), None)
        );
    }

    let save = sqlx::query_as::<_, Save>("SELECT * FROM saves WHERE id = ?;")
        .bind(result.last_insert_rowid())
        .fetch_one(&pool).await.unwrap();

    (StatusCode::CREATED, res_body!(true, None, Some(save)))
}
