use worker::*;

use crate::error::AppError;
use crate::media::model::Media;

pub async fn search(db: &D1Database, query: &str, limit: u32) -> Result<Vec<Media>, AppError> {
    let pattern = format!("%{query}%");
    let result = db
        .prepare(
            "SELECT * FROM media WHERE original_name LIKE ?1 ORDER BY created_at DESC LIMIT ?2",
        )
        .bind(&[pattern.into(), (limit as f64).into()])?
        .run()
        .await?;

    Ok(result.results::<Media>()?)
}
