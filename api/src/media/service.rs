use chrono::Utc;
use uuid::Uuid;
use worker::*;

use super::model::Media;
use crate::error::AppError;

pub fn validate_mime(mime: &str) -> Result<(), AppError> {
    match mime {
        "image/jpeg" | "image/png" | "image/webp" | "video/mp4" | "video/quicktime" => Ok(()),
        _ => Err(AppError::Validation(format!(
            "unsupported mime type: {mime}"
        ))),
    }
}

pub fn validate_size(size: usize, mime: &str) -> Result<(), AppError> {
    match mime.starts_with("video") {
        true if size > 100_000_000 => Err(AppError::Validation("video exceeds 100mb".into())),
        false if size > 50_000_000 => Err(AppError::Validation("image exceeds 50mb".into())),
        _ => Ok(()),
    }
}

pub fn generate_key(mime: &str) -> String {
    let prefix = match mime.starts_with("video") {
        true => "videos",
        false => "photos",
    };
    let uuid = Uuid::new_v4();
    format!("{}/{}/{}", prefix, Utc::now().format("%Y/%m"), uuid)
}

pub fn extension(mime: &str) -> &str {
    match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "video/mp4" => "mp4",
        "video/quicktime" => "mov",
        _ => "bin",
    }
}

pub async fn insert(db: &D1Database, media: &Media) -> Result<(), AppError> {
    let mut values: Vec<wasm_bindgen::JsValue> = vec![
        media.id.clone().into(),
        media.original_name.clone().into(),
        media.mime_type.clone().into(),
        (media.file_size as f64).into(),
        (media.width.unwrap_or(0) as f64).into(),
        (media.height.unwrap_or(0) as f64).into(),
        media.duration.unwrap_or(0.0).into(),
        media.bucket_path.clone().into(),
    ];
    values.push(match &media.album_id {
        Some(aid) => aid.clone().into(),
        None => wasm_bindgen::JsValue::null(),
    });

    db.prepare(
        "INSERT INTO media (id, original_name, mime_type, file_size,
         width, height, duration, bucket_path, album_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
    )
    .bind(&values)?
    .run()
    .await?;

    Ok(())
}

pub async fn get(db: &D1Database, id: &str) -> Result<Option<Media>, AppError> {
    let result = db
        .prepare("SELECT * FROM media WHERE id = ?1")
        .bind(&[id.into()])?
        .run()
        .await?;

    let rows = result.results::<Media>()?;
    Ok(rows.into_iter().next())
}

pub async fn list(
    db: &D1Database,
    limit: u32,
    offset: u32,
    album_id: Option<String>,
) -> Result<Vec<Media>, AppError> {
    let result = match album_id {
        Some(aid) => {
            db.prepare(
                "SELECT * FROM media WHERE album_id = ?1 ORDER BY created_at DESC LIMIT ?2 OFFSET ?3",
            )
            .bind(&[aid.into(), (limit as f64).into(), (offset as f64).into()])?
            .run()
            .await?
        }
        None => {
            db.prepare(
                "SELECT * FROM media ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
            )
            .bind(&[(limit as f64).into(), (offset as f64).into()])?
            .run()
            .await?
        }
    };

    Ok(result.results::<Media>()?)
}

pub async fn delete(db: &D1Database, id: &str) -> Result<Option<String>, AppError> {
    let media = get(db, id).await?;
    match media {
        Some(m) => {
            db.prepare("DELETE FROM media WHERE id = ?1")
                .bind(&[id.into()])?
                .run()
                .await?;
            Ok(Some(m.bucket_path))
        }
        None => Ok(None),
    }
}

pub async fn upload_r2(storage: &Bucket, key: &str, data: &[u8]) -> Result<(), AppError> {
    storage.put(key, data.to_vec()).execute().await?;
    Ok(())
}

pub async fn get_r2(storage: &Bucket, key: &str) -> Result<Option<Vec<u8>>, AppError> {
    let obj = storage.get(key).execute().await?;
    match obj {
        Some(o) => {
            let bytes = o.body().ok_or(AppError::NotFound)?.bytes().await?;
            Ok(Some(bytes))
        }
        None => Ok(None),
    }
}

pub async fn assign_album(db: &D1Database, media_id: &str, album_id: &str) -> Result<(), AppError> {
    let album_exists = db
        .prepare("SELECT COUNT(*) as count FROM albums WHERE id = ?1")
        .bind(&[album_id.into()])?
        .run()
        .await?;

    if album_exists.results::<serde_json::Value>()?.is_empty() {
        return Err(AppError::NotFound);
    }

    db.prepare("UPDATE media SET album_id = ?1 WHERE id = ?2")
        .bind(&[album_id.into(), media_id.into()])?
        .run()
        .await?;

    Ok(())
}

pub async fn remove_from_album(db: &D1Database, media_id: &str) -> Result<(), AppError> {
    db.prepare("UPDATE media SET album_id = NULL WHERE id = ?1")
        .bind(&[media_id.into()])?
        .run()
        .await?;

    Ok(())
}

pub async fn delete_r2(storage: &Bucket, key: &str) -> Result<(), AppError> {
    storage.delete(key).await?;
    Ok(())
}
