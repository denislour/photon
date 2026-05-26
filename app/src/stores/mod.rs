mod app_store;
mod media_store;

pub use app_store::AppStore;
pub use media_store::MediaStore;

#[derive(Clone, Copy)]
pub struct AppCtx {
    pub app: AppStore,
    pub media: MediaStore,
}
