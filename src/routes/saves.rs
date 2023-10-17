use axum::{Router, routing::get, response::IntoResponse, http::StatusCode, Json, Extension, extract::{Multipart, DefaultBodyLimit}};
use tower_http::limit::RequestBodyLimitLayer;
use std::{fs, io::Write};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::{res_body, models::{res::*, Save, SaveFile, SaveWithFiles}};

pub fn get_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/saves", get(saves_get).post(saves_post))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024 * 1024))  // 10 GB
        .layer(Extension(pool))
}

async fn saves_get(
    Extension(pool): Extension<SqlitePool>
) -> impl IntoResponse {
    let mut list = Vec::<SaveWithFiles>::new();

    let save_list = sqlx::query_as::<_, Save>("SELECT * FROM saves;")
        .fetch_all(&pool).await.unwrap();

    for save in save_list {
        let file_list = sqlx::query_as::<_, SaveFile>("SELECT * FROM saves_files WHERE save_id = ?;")
            .bind(save.id)
            .fetch_all(&pool).await.unwrap();

        list.push(SaveWithFiles {
            id: save.id,
            text: save.text,
            caption: save.caption,
            created: save.created,
            files: file_list,
        });
    }

    (StatusCode::OK, res_body!(true, None, Some(list)))
}

async fn saves_post(
    Extension(pool): Extension<SqlitePool>,
    multipart: Multipart,
) -> impl IntoResponse {

    // Parsing the multipart body
    let (text, caption, files) = parse_multipart(multipart).await;
    if text.is_none() || caption.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            res_body!(true, Some("Missing values in request"), None)
        );
    }

    // Inserting the save
    let result = sqlx::query("INSERT INTO saves (text, caption) VALUES (?, ?);")
        .bind(text.unwrap()).bind(caption.unwrap())
        .execute(&pool).await.unwrap();

    let save_id = result.last_insert_rowid();
    if result.rows_affected() < 1 {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            res_body!(true, Some("Could not insert the save"), None)
        );
    }

    // Inserting the files' info
    if files.len() > 0 {
        let mut query_str = "INSERT INTO saves_files (save_id, hash_name, file_name, file_size) VALUES ".to_owned();
        for i in 0..files.len() {
            if i > 0 {
                query_str += ",";
            }
            query_str += &format!("({},'{}','{}',{})", save_id, files[i].0, files[i].1, files[i].2);
        }

        let rows_inserted = sqlx::query(&query_str)
            .execute(&pool).await.unwrap()
            .rows_affected();

        if rows_inserted != files.len() as u64 {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                res_body!(true, Some("Could not insert some or all files"), None)
            );
        }
    }

    // Sending the response
    let save = sqlx::query_as::<_, Save>("SELECT * FROM saves WHERE id = ?;")
        .bind(save_id)
        .fetch_one(&pool).await.unwrap();

    let files = sqlx::query_as::<_, SaveFile>("SELECT * FROM saves_files WHERE save_id = ?;")
        .bind(save_id)
        .fetch_all(&pool).await.unwrap();

    let save_with_files = SaveWithFiles {
        id: save.id,
        text: save.text,
        caption: save.caption,
        created: save.created,
        files: files,
    };

    (StatusCode::CREATED, res_body!(true, None, Some(save_with_files)))
}

// TODO: make this function do all the input validation stuff and return appropriate errors
async fn parse_multipart(mut multipart: Multipart) -> (Option<String>, Option<String>, Vec<(String, String, i64)>) {
    let mut text = None;
    let mut caption = None;
    let mut files = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap() {

            "text" => {
                let bytes = field.bytes().await.unwrap();
                text = Some(std::str::from_utf8(&bytes).unwrap().to_string());
            },

            "caption" => {
                let bytes = field.bytes().await.unwrap();
                caption = Some(std::str::from_utf8(&bytes).unwrap().to_string());
            },

            "files" => {

                // Get the basic data
                let file_name = field.file_name().unwrap().to_string();
                let file_extension = file_name.split('.').last().unwrap();

                if file_name.len() == 0 {
                    // panic!("Invalid file name");
                    continue;
                }

                let mut hash_name = Uuid::new_v4().to_string();
                if file_extension.len() > 0 {
                    hash_name += &format!(".{file_extension}");
                }

                let file_path = format!("./files/{hash_name}");

                // Save the file
                let data = field.bytes().await.unwrap();
                let file_size = data.len() as i64;

                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_path)
                    .unwrap();

                file.write_all(&data).unwrap();

                // Save this stuff for the database
                files.push((hash_name, file_name, file_size));
            },

            name => println!("Got an unknown field name: `{name}`"),
        }
    }

    (text, caption, files)
}
