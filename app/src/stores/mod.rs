mod album_store;
mod app_store;
mod media_store;
mod toast_store;
mod upload_store;

pub use album_store::AlbumStore;
pub use app_store::AppStore;
pub use media_store::MediaStore;
pub use toast_store::ToastStore;
pub use upload_store::UploadStore;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AppCtx {
    pub app: AppStore,
    pub media: MediaStore,
    pub album: AlbumStore,
    pub upload: UploadStore,
    pub toast: ToastStore,
}
