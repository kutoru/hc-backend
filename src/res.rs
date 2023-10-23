use serde::Serialize;
pub use axum::http::StatusCode;
pub use axum::Json;
pub use crate::error::ResError;

pub type ServerResult<T> = core::result::Result<(StatusCode, Json<ResultBody<T>>), ResError>;

#[derive(Debug, Serialize, Clone)]
pub struct ResultBody<T> {
    pub success: bool,
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[macro_export]
macro_rules! res_body {
    ($success:expr, $msg:expr, $data:expr) => {
        Json(ResultBody { success: $success, msg: $msg, data: $data })
        // Json(json!({ "success": $success, "msg": $msg, "data": $data }))
    }
}
