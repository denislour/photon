use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::Header;
use crate::pages::{AlbumsPage, GalleryPage};
use crate::stores::{AppCtx, AppStore, MediaStore};

#[component]
pub fn App() -> impl IntoView {
    let ctx = AppCtx {
        app: AppStore::new(),
        media: MediaStore::new(),
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
        </Router>
    }
}
