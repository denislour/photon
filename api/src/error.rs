use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation: {0}")]
    Validation(String),
    #[error("not found")]
    NotFound,
    #[error("missing file")]
    MissingFile,
    #[error("internal: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::Validation(m) => (StatusCode::BAD_REQUEST, m.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            AppError::MissingFile => (StatusCode::BAD_REQUEST, "missing file".into()),
            AppError::Internal(e) => {
                worker::console_log!("[BE ERROR] {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("internal: {e}"))
            }
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}

impl From<worker::Error> for AppError {
    fn from(e: worker::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}
