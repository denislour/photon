use leptos::prelude::*;

use crate::stores::AppCtx;
use crate::utils::icons;

const NAV_CLASS: &str = "sticky top-0 z-30 h-14 bg-navy/88 backdrop-blur-md border-b border-hl";
const NAV_INNER_CLASS: &str = "max-w-[1200px] mx-auto px-5 h-full flex items-center gap-3";
const LINK_CLASS: &str = "flex items-center gap-2.5 shrink-0 no-underline text-ink";
const LOGO_BOX_CLASS: &str =
    "w-7 h-7 rounded-full bg-gold text-navy flex items-center justify-center";
const TITLE_CLASS: &str = "text-base font-semibold -tracking-[.3px]";
const NAV_LINKS_CLASS: &str = "flex items-center gap-4 ml-auto";
const ALBUM_LINK_CLASS: &str =
    "text-sm text-body hover:text-ink transition-colors no-underline flex items-center gap-1.5";
const ICON_CONTAINER_CLASS: &str = "w-4 h-4 flex items-center justify-center";
const VIEW_BTN_WRAPPER: &str = "flex items-center gap-1";
const VIEW_BTN_CLASS: &str = "w-7 h-7 flex items-center justify-center rounded-sm transition-all";

#[allow(non_snake_case)]
#[component]
pub fn Header() -> impl IntoView {
    let AppCtx { app, .. } = expect_context();

    view! {
        <nav class=NAV_CLASS>
            <div class=NAV_INNER_CLASS>
                <a href="/" class=LINK_CLASS>
                    <span class=LOGO_BOX_CLASS>
                        <span inner_html=icons::LOGO />
                    </span>
                    <span class=TITLE_CLASS>"Photon"</span>
                </a>
                <div class=NAV_LINKS_CLASS>
                    <a href="/albums"
                        class=ALBUM_LINK_CLASS>
                        <span class=ICON_CONTAINER_CLASS inner_html=icons::FOLDER />
                        <span>"Albums"</span>
                    </a>
                    <div class=VIEW_BTN_WRAPPER>
                        <button
                            on:click=move |_| app.set_view_mode("grid")
                            class=VIEW_BTN_CLASS
                            class:bg-surf=move || app.view_mode().get() == "grid"
                            class:text-body=move || app.view_mode().get() != "grid"
                            title="Grid"
                        >
                            <span inner_html=icons::GRID />
                        </button>
                        <button
                            on:click=move |_| app.set_view_mode("list")
                            class=VIEW_BTN_CLASS
                            class:bg-surf=move || app.view_mode().get() == "list"
                            class:text-body=move || app.view_mode().get() != "list"
                            title="List"
                        >
                            <span inner_html=icons::LIST />
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}
