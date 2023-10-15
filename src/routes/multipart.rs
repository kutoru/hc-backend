use std::{fs, io::Write};

use axum::{Router, extract::{DefaultBodyLimit, Multipart}, routing::post, http::StatusCode, response::IntoResponse, Json};
use tower_http::limit::RequestBodyLimitLayer;
use uuid::Uuid;
use crate::{res_body, models::res::*};

pub fn get_router() -> Router {
    Router::new()
        .route("/multipart-test", post(accept_multipart))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 1024))  // 1 GB
}

async fn accept_multipart(mut multipart: Multipart) -> impl IntoResponse {
    let mut text = None;
    let mut caption = None;
    let mut files: Vec<Vec<String>> = vec![];

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

                let hash = Uuid::new_v4().to_string();
                let file_path = format!("./files/{hash}.{file_extension}");

                // Save the file
                let data = field.bytes().await.unwrap();
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_path)
                    .unwrap();

                file.write_all(&data).unwrap();

                // Save this stuff for the database
                files.push(vec![hash, file_name]);
            },

            name => println!("Got an unknown field name: `{name}`"),
        }

        // println!("Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes", data.len());
    }

    println!("{:#?}, {:#?}, {:#?}", text, caption, files);

    if text.is_none() {
        panic!("Text is none");
    } else if caption.is_none() {
        panic!("Caption is none");
    }

    // Add all the stuff into the database here


    (StatusCode::CREATED, res_body!(true, Some("Operation was successfull but the response is not implemented"), None::<()>))
}
