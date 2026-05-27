use axum::body::Body;
use axum::response::Response as AxumResponse;
use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use super::model::{AssignAlbum, ListParams, Media};
use super::service;
use crate::AppState;
use crate::error::AppError;

#[worker::send]
pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name = String::from("unknown");
    let mut file_mime = String::from("application/octet-stream");

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?
    {
        match field.name() {
            Some("file") => {
                file_name = field.file_name().unwrap_or("unknown").to_string();
                file_mime = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                file_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| AppError::Validation(e.to_string()))?
                        .to_vec(),
                );
            }
            _ => {}
        }
    }

    let data = file_data.ok_or(AppError::MissingFile)?;

    service::validate_mime(&file_mime)?;
    service::validate_size(data.len(), &file_mime)?;

    let key = format!(
        "{}.{}",
        service::generate_key(&file_mime),
        service::extension(&file_mime)
    );
    service::upload_r2(&state.storage.0, &key, &data).await?;

    let media = Media {
        id: uuid::Uuid::new_v4().to_string(),
        original_name: file_name,
        mime_type: file_mime,
        file_size: data.len() as i64,
        width: None,
        height: None,
        duration: None,
        bucket_path: key,
        album_id: None,
        created_at: String::new(),
    };

    service::insert(&state.db.0, &media).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "id": media.id, "url": format!("/api/media/{}", media.id) })),
    ))
}

#[worker::send]
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Value>, AppError> {
    let limit = params.limit.unwrap_or(50).min(200);
    let offset = params.offset.unwrap_or(0);
    let items = service::list(&state.db.0, limit, offset, params.album_id).await?;

    Ok(Json(json!({
        "items": items,
        "limit": limit,
        "offset": offset,
    })))
}

#[worker::send]
pub async fn serve(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<AxumResponse<Body>, AppError> {
    let media = service::get(&state.db.0, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    let data = service::get_r2(&state.storage.0, &media.bucket_path)
        .await?
        .ok_or(AppError::NotFound)?;

    let response = AxumResponse::builder()
        .header("Content-Type", media.mime_type)
        .header("Cache-Control", "public, max-age=31536000, immutable")
        .body(Body::from(data))
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(response)
}

#[worker::send]
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let bucket_path = service::delete(&state.db.0, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    service::delete_r2(&state.storage.0, &bucket_path).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[worker::send]
pub async fn assign_album(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<AssignAlbum>,
) -> Result<Json<Value>, AppError> {
    service::assign_album(&state.db.0, &id, &body.album_id).await?;
    Ok(Json(json!({"status": "ok"})))
}

#[worker::send]
pub async fn remove_album(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    service::remove_from_album(&state.db.0, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
