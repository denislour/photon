use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    pub original_name: String,
    pub mime_type: String,
    pub file_size: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<f64>,
    pub bucket_path: String,
    pub album_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub album_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignAlbum {
    pub album_id: String,
}
