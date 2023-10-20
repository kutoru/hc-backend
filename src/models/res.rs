use serde::Serialize;
pub use axum::http::StatusCode;
pub use axum::Json;

pub type ServerFunctionResponse<T> = (StatusCode, Json<ResultBody<T>>);

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
