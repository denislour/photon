use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub original_name: String,
    pub mime_type: String,
    pub file_size: i64,
    pub bucket_path: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_path: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
struct MediaListResponse {
    pub items: Vec<MediaItem>,
}

#[derive(Debug, Deserialize)]
struct AlbumListResponse {
    pub albums: Vec<Album>,
}

#[derive(Debug, Deserialize)]
struct AlbumCreateResponse {
    pub album: Album,
}

#[derive(Debug, Serialize)]
struct CreateAlbumBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub id: String,
    pub url: String,
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

pub async fn upload_file(file: web_sys::File) -> Result<UploadResponse, String> {
    let blob: web_sys::Blob = file.clone().into();
    let buf = js_sys::Uint8Array::new(&blob);
    let bytes = buf.to_vec();

    let part = Part::bytes(bytes)
        .file_name(file.name())
        .mime_str(&file.type_())
        .map_err(|e| e.to_string())?;

    let form = Form::new().part("file", part);

    let resp = reqwest::Client::new()
        .post("/api/media")
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let text = resp.text().await.map_err(|e| e.to_string())?;
        return Err(text);
    }

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_albums() -> Result<Vec<Album>, String> {
    let resp = reqwest::Client::new()
        .get("/api/albums")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let body: AlbumListResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.albums)
}

pub async fn create_album(name: &str, description: Option<String>) -> Result<Album, String> {
    let body = CreateAlbumBody {
        name: name.into(),
        description,
    };

    let resp = reqwest::Client::new()
        .post("/api/albums")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let text = resp.text().await.map_err(|e| e.to_string())?;
        return Err(text);
    }

    let body: AlbumCreateResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.album)
}
