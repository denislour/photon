use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_path: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlbum {
    pub name: String,
    pub description: Option<String>,
}
