use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::{Header, Toast};
use crate::pages::{AlbumsPage, GalleryPage};
use crate::stores::{AlbumStore, AppCtx, AppStore, MediaStore, ToastStore, UploadStore};

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    let app = AppStore::new();
    let media = MediaStore::new();
    let album = AlbumStore::new();
    let upload = UploadStore::new();
    let toast = ToastStore::new();

    let ctx = AppCtx {
        app,
        media,
        album,
        upload,
        toast,
    };
    provide_context(ctx);
    provide_context(app);
    provide_context(media);
    provide_context(album);
    provide_context(upload);
    provide_context(toast);

    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=|| view! { "404" }>
                    <Route path=path!("") view=GalleryPage />
                    <Route path=path!("albums") view=AlbumsPage />
                </Routes>
            </main>
            <Toast />
        </Router>
    }
}
