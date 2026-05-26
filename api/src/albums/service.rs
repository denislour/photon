use uuid::Uuid;
use worker::*;
use wasm_bindgen::JsValue;

use super::model::Album;
use crate::error::AppError;

pub async fn create(db: &D1Database, name: &str, description: Option<String>) -> Result<Album, AppError> {
    let id = Uuid::new_v4().to_string();
    let mut values: Vec<JsValue> = vec![id.clone().into(), name.into()];
    let desc_val = description.clone();
    values.push(description.map_or(JsValue::null(), JsValue::from));

    db.prepare(
        "INSERT INTO albums (id, name, description, created_at) VALUES (?1, ?2, ?3, datetime('now'))",
    )
    .bind(&values)?
    .run()
    .await?;

    Ok(Album {
        id,
        name: name.into(),
        description: desc_val,
        cover_path: None,
        created_at: String::new(),
    })
}

pub async fn list(db: &D1Database) -> Result<Vec<Album>, AppError> {
    let result = db
        .prepare("SELECT * FROM albums ORDER BY created_at DESC")
        .bind(&[] as &[JsValue])?
        .run()
        .await?;

    Ok(result.results::<Album>()?)
}

pub async fn get(db: &D1Database, id: &str) -> Result<Option<Album>, AppError> {
    let result = db
        .prepare("SELECT * FROM albums WHERE id = ?1")
        .bind(&[id.into()])?
        .run()
        .await?;

    let rows = result.results::<Album>()?;
    Ok(rows.into_iter().next())
}
