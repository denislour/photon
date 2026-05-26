use axum::{extract::State, Json};
use serde::Deserialize;

use crate::error::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct LogBody {
    pub msg: String,
}

pub async fn log(
    _state: State<AppState>,
    Json(body): Json<LogBody>,
) -> Result<&'static str, AppError> {
    worker::console_log!("[FE] {}", body.msg);
    Ok("ok")
}
