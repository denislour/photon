use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub original_name: String,
    pub mime_type: String,
    pub file_size: i64,
    pub bucket_path: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
struct MediaListResponse {
    pub items: Vec<MediaItem>,
}

pub async fn fetch_media() -> Result<Vec<MediaItem>, String> {
    let resp = reqwest::Client::new()
        .get("/api/media")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let body: MediaListResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.items)
}
