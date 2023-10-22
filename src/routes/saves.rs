use axum::{Router, routing::{get, patch}, http::StatusCode, Json, Extension, extract::{Multipart, DefaultBodyLimit, Path}, handler::Handler};
use tokio::io::AsyncWriteExt;
use tower_http::limit::RequestBodyLimitLayer;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::{res_body, models::{res::*, Save, FileInfo, SaveWithFiles, SavePatch}, error::{ServerResult, ResError}};

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/saves", get(saves_get).post(
            saves_post
                .layer(DefaultBodyLimit::disable())
                .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024 * 1024))  // 10 GB
        ))
        .route("/saves/:id", patch(save_patch).delete(save_delete))
        .layer(Extension(pool))
}

async fn saves_get(
    Extension(pool): Extension<SqlitePool>
) -> ServerResult<Vec<SaveWithFiles>> {
    let mut list = Vec::<SaveWithFiles>::new();

    let save_list = sqlx::query_as::<_, Save>("SELECT * FROM saves;")
        .fetch_all(&pool).await?;

    for save in save_list {
        let file_list = sqlx::query_as::<_, FileInfo>("SELECT * FROM files WHERE save_id = ?;")
            .bind(save.id)
            .fetch_all(&pool).await?;

        list.push(SaveWithFiles {
            id: save.id,
            text: save.text,
            caption: save.caption,
            created: save.created,
            files: file_list,
        });
    }

    Ok((StatusCode::OK, res_body!(true, None, Some(list))))
}

async fn saves_post(
    Extension(pool): Extension<SqlitePool>,
    multipart: Multipart,
) -> ServerResult<SaveWithFiles> {

    // Parsing the multipart body
    let (text, caption, files) = parse_multipart(multipart).await?;

    // Inserting the save
    let save_id = sqlx::query("INSERT INTO saves (text, caption) VALUES (?, ?);")
        .bind(text).bind(caption)
        .execute(&pool).await?
        .last_insert_rowid();

    // Inserting the files' info
    if files.len() > 0 {
        let mut query_str = "INSERT INTO files (save_id, hash_name, file_name, file_size) VALUES ".to_owned();
        for i in 0..files.len() {
            if i > 0 {
                query_str += ",";
            }
            query_str += &format!("({},'{}','{}',{})", save_id, files[i].0, files[i].1, files[i].2);
        }

        sqlx::query(&query_str)
            .execute(&pool).await?;
    }

    // Sending the response
    let save = sqlx::query_as::<_, Save>("SELECT * FROM saves WHERE id = ?;")
        .bind(save_id)
        .fetch_one(&pool).await?;

    let files = sqlx::query_as::<_, FileInfo>("SELECT * FROM files WHERE save_id = ?;")
        .bind(save_id)
        .fetch_all(&pool).await?;

    let save_with_files = SaveWithFiles {
        id: save.id,
        text: save.text,
        caption: save.caption,
        created: save.created,
        files: files,
    };

    Ok((StatusCode::CREATED, res_body!(true, None, Some(save_with_files))))
}

async fn parse_multipart(
    mut multipart: Multipart
) -> Result<(String, String, Vec<(String, String, i64)>), ResError> {
    let mut text = None;
    let mut caption = None;
    let mut files = vec![];

    while let Some(field) = multipart.next_field().await? {
        match field.name().unwrap() {

            "text" => {
                let bytes = field.bytes().await?;
                text = Some(std::str::from_utf8(&bytes)?.into());
            },

            "caption" => {
                let bytes = field.bytes().await?;
                caption = Some(std::str::from_utf8(&bytes)?.into());
            },

            "files" => {

                // Get the basic data
                let file_name = field.file_name().unwrap_or("").to_string();
                let file_extension = file_name.split('.').last().unwrap_or("");

                if file_name.len() == 0 {
                    continue;
                }

                let mut hash_name = Uuid::new_v4().to_string();
                if file_extension.len() > 0 {
                    hash_name += &format!(".{file_extension}");
                }

                let file_path = format!("./files/{hash_name}");

                // Save the file
                let data = field.bytes().await?;
                let file_size = data.len() as i64;

                let mut file = tokio::fs::OpenOptions::new()
                    .create(true).write(true).open(file_path).await?;

                file.write_all(&data).await?;

                // Save this stuff for the database
                files.push((hash_name, file_name, file_size));
            },

            name => println!("Got an unknown field name: `{name}`"),
        }
    }

    if text.is_none() || caption.is_none() {
        return Err(ResError::MissingFields("Missing fields".into()));
    }

    Ok((text.unwrap(), caption.unwrap(), files))
}

async fn save_patch(
    Path(save_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<SavePatch>,
) -> ServerResult<Save> {

    sqlx::query("UPDATE saves SET text = ?, caption = ? WHERE id = ?;")
        .bind(body.text).bind(body.caption).bind(save_id)
        .execute(&pool).await?;

    let save = sqlx::query_as::<_, Save>("SELECT * FROM saves WHERE id = ?;")
        .bind(save_id)
        .fetch_one(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, Some(save))))
}

async fn save_delete(
    Path(save_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> ServerResult<()> {

    let files = sqlx::query_as::<_, FileInfo>("SELECT * FROM files WHERE save_id = ?;")
        .bind(save_id)
        .fetch_all(&pool).await?;

    sqlx::query("DELETE FROM files WHERE save_id = ?;")
        .bind(save_id)
        .execute(&pool).await?;

    for file_info in files {
        let file_path = format!("./files/{}", file_info.hash_name);
        tokio::fs::remove_file(file_path).await?;
    }

    sqlx::query("DELETE FROM saves WHERE id = ?;")
        .bind(save_id)
        .execute(&pool).await?;

    Ok((StatusCode::OK, res_body!(true, None, None)))
}
