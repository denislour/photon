use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use super::model::CreateAlbum;
use super::service;
use crate::AppState;
use crate::error::AppError;

#[worker::send]
pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateAlbum>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    if body.name.trim().is_empty() {
        return Err(AppError::Validation("name is required".into()));
    }

    let album = service::create(&state.db.0, &body.name, body.description).await?;
    Ok((StatusCode::CREATED, Json(json!({ "album": album }))))
}

#[worker::send]
pub async fn list(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let albums = service::list(&state.db.0).await?;
    Ok(Json(json!({ "albums": albums })))
}

#[worker::send]
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let album = service::get(&state.db.0, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(json!({ "album": album })))
}
