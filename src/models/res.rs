use serde::Serialize;
pub use axum::Json;

#[derive(Debug, Serialize, Clone)]
pub struct ServerResult<T> {
    pub success: bool,
    pub msg: Option<&'static str>,
    pub data: Option<T>,
}

#[macro_export]
macro_rules! res_body {
    ($success:expr, $msg:expr, $data:expr) => {
        Json(ServerResult { success: $success, msg: $msg, data: $data })
        // Json(json!({ "success": $success, "msg": $msg, "data": $data }))
    }
}
