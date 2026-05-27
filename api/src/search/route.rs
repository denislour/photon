use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::service;
use crate::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub limit: Option<u32>,
}

#[worker::send]
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Value>, AppError> {
    if params.q.trim().is_empty() {
        return Ok(Json(json!({ "items": [], "query": "" })));
    }

    let limit = params.limit.unwrap_or(20).min(100);
    let items = service::search(&state.db.0, &params.q, limit).await?;

    Ok(Json(json!({ "items": items, "query": params.q })))
}
