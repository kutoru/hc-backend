use axum::{response::{IntoResponse, Response}, http::{StatusCode, header::InvalidHeaderValue}, Json};
use sqlx::error::ErrorKind;
use crate::{res_body, res::ResultBody};

#[derive(Debug)]
pub enum ResError {
    MissingFields(String),  // one or more fields in headers or body are missing
    InvalidFields(String),  // one or more fields are present but have invalid values
    NotFound(String),  // when getting, patching or deleting something that doesn't exist
    BadRequest(String),  // when the issue with the request is too hard to explain

    FSError(String),
    DBError(String),
    ServerError(String),  // anything else
}

impl core::fmt::Display for ResError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for ResError {}

impl IntoResponse for ResError {
    fn into_response(self) -> Response {
        match self {
            Self::MissingFields(msg) => (StatusCode::UNPROCESSABLE_ENTITY, get_body(msg)),
            Self::InvalidFields(msg) => (StatusCode::UNPROCESSABLE_ENTITY, get_body(msg)),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, get_body(msg)),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, get_body(msg)),

            Self::FSError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, get_body(msg)),
            Self::DBError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, get_body(msg)),
            Self::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, get_body(msg)),
        }.into_response()
    }
}

fn get_body(msg: String) -> Json<ResultBody<()>> {
    res_body!(false, Some(msg), None)
}

fn get_msg<T: std::fmt::Debug>(value: T) -> String {
    let msg = format!("{:?}", value);
    println!("{msg}");
    msg
}

impl From<sqlx::error::Error> for ResError {
    fn from(value: sqlx::error::Error) -> Self {
        let msg = get_msg(&value);
        match value {
            sqlx::Error::Database(e) => match e.kind() {
                ErrorKind::ForeignKeyViolation => Self::BadRequest(msg),
                _ => Self::DBError(msg),
            },
            sqlx::Error::RowNotFound => Self::NotFound(msg),
            _ => Self::DBError(msg),
        }
    }
}

impl From<axum::extract::multipart::MultipartError> for ResError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        Self::InvalidFields(get_msg(value))
    }
}

impl From<std::str::Utf8Error> for ResError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::InvalidFields(get_msg(value))
    }
}

impl From<std::io::Error> for ResError {
    fn from(value: std::io::Error) -> Self {
        Self::FSError(get_msg(value))
    }
}

impl From<InvalidHeaderValue> for ResError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::ServerError(get_msg(value))
    }
}
