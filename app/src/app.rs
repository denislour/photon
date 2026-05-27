use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::{Header, Toast};
use crate::pages::{AlbumsPage, GalleryPage};
use crate::stores::{AlbumStore, AppCtx, AppStore, MediaStore, ToastStore, UploadStore};

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    let ctx = AppCtx {
        app: AppStore::new(),
        media: MediaStore::new(),
        album: AlbumStore::new(),
        upload: UploadStore::new(),
        toast: ToastStore::new(),
    };
    provide_context(ctx);

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
