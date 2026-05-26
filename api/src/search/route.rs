use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use serde_json::json;

use super::service;
use crate::error::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub limit: Option<u32>,
}

#[worker::send]
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    if params.q.trim().is_empty() {
        return Ok(Json(json!({ "items": [], "query": "" })));
    }

    let limit = params.limit.unwrap_or(20).min(100);
    let items = service::search(&state.db.0, &params.q, limit).await?;

    Ok(Json(json!({ "items": items, "query": params.q })))
}
