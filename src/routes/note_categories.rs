use axum::{Router, routing::get, Extension, http::StatusCode, Json, extract::Path};
use sqlx::SqlitePool;

use crate::{models::{NoteCategoryPost, NoteCategory, NoteCategoryWithNotes, Note, NoteCategoryWithCounts}, res::*, res_body};

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/note-categories", get(note_categories_get).post(note_categories_post))
        .route("/note-categories/:id", get(note_category_get).patch(note_category_patch).delete(note_category_delete))
        .layer(Extension(pool))
}

async fn note_categories_get(
    Extension(pool): Extension<SqlitePool>
) -> ServerResult<Vec<NoteCategoryWithCounts>> {

    let category_list = sqlx::query_as::<_, NoteCategoryWithCounts>(
        "SELECT nc.*, COUNT(n.id) AS note_count
        FROM note_categories AS nc
        LEFT JOIN notes AS n ON n.category_id = nc.id
        GROUP BY nc.id;"
    )
        .fetch_all(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, Some(category_list))))
}

async fn note_categories_post(
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<NoteCategoryPost>,
) -> ServerResult<NoteCategory> {

    let category_id = sqlx::query("INSERT INTO note_categories (title, description) VALUES (?, ?);")
        .bind(body.title).bind(body.description)
        .execute(&pool).await?
        .last_insert_rowid();

    let category = sqlx::query_as::<_, NoteCategory>("SELECT * FROM note_categories WHERE id = ?;")
        .bind(category_id)
        .fetch_one(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, Some(category))))
}

async fn note_category_get(
    Path(category_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> ServerResult<NoteCategoryWithNotes> {

    let category = sqlx::query_as::<_, NoteCategory>("SELECT * FROM note_categories WHERE id = ?;")
        .bind(category_id)
        .fetch_one(&pool).await?;

    let note_list = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE category_id = ?;")
        .bind(category_id)
        .fetch_all(&pool).await?;

    let category_with_notes = NoteCategoryWithNotes {
        id: category.id,
        title: category.title,
        description: category.description,
        created: category.created,
        notes: note_list,
    };

    Ok((StatusCode::OK, res_body!(true, None, Some(category_with_notes))))
}

async fn note_category_patch(
    Path(category_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<NoteCategoryPost>,
) -> ServerResult<NoteCategory> {

    sqlx::query("UPDATE note_categories SET title = ?, description = ? WHERE id = ?;")
        .bind(body.title).bind(body.description).bind(category_id)
        .execute(&pool).await?;

    let category = sqlx::query_as::<_, NoteCategory>("SELECT * FROM note_categories WHERE id = ?;")
        .bind(category_id)
        .fetch_one(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, Some(category))))
}

async fn note_category_delete(
    Path(category_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> ServerResult<()> {

    sqlx::query("DELETE FROM notes WHERE category_id = ?;")
        .bind(category_id)
        .execute(&pool).await?;

    sqlx::query("DELETE FROM note_categories WHERE id = ?;")
        .bind(category_id)
        .execute(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, None)))
}
