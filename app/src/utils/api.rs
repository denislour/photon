use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

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
#[allow(dead_code)]
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
    let file_name = file.name();
    let file_mime = file.type_();
    web_sys::console::log_1(&JsValue::from(format!("[API] upload: {file_name} ({file_mime})")));

    // Use browser native fetch with FormData instead of reqwest multipart
    let form_data = web_sys::FormData::new().map_err(|e| format!("FormData error: {:?}", e))?;
    form_data.append_with_blob("file", &file).map_err(|e| format!("append error: {:?}", e))?;

    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&form_data);

    let request = web_sys::Request::new_with_str_and_init("/api/media", &opts)
        .map_err(|e| format!("Request init error: {:?}", e))?;

    let promise = web_sys::window()
        .unwrap()
        .fetch_with_request(&request);

    let resp = JsFuture::from(promise).await
        .map_err(|e| format!("fetch error: {:?}", e))?;

    let resp = resp.unchecked_into::<web_sys::Response>();
    let status = resp.status();
    web_sys::console::log_1(&JsValue::from(format!("[API] response status: {status}")));

    if status != 201 {
        let text_promise = resp.text().map_err(|e| format!("text error: {:?}", e))?;
        let text = JsFuture::from(text_promise).await
            .map_err(|e| format!("text await error: {:?}", e))?;
        let text = text.as_string().unwrap_or_default();
        web_sys::console::log_1(&JsValue::from(format!("[API] error body: {text}")));
        return Err(text);
    }

    let json_promise = resp.json().map_err(|e| format!("json error: {:?}", e))?;
    let json_val = JsFuture::from(json_promise).await
        .map_err(|e| format!("json await error: {:?}", e))?;

    let result: UploadResponse = serde_wasm_bindgen::from_value(json_val)
        .map_err(|e| format!("deserialize error: {e}"))?;

    web_sys::console::log_1(&JsValue::from("[API] upload success"));
    Ok(result)
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
