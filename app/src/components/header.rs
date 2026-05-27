use leptos::prelude::*;

use crate::stores::AppCtx;
use crate::utils::icons;

#[allow(non_snake_case)]
#[component]
pub fn Header() -> impl IntoView {
    let AppCtx { app, .. } = expect_context();

    view! {
        <nav class="sticky top-0 z-30 h-14 bg-navy/88 backdrop-blur-md border-b border-hl">
            <div class="max-w-[1200px] mx-auto px-5 h-full flex items-center gap-3">
                <a href="/" class="flex items-center gap-2.5 shrink-0 no-underline text-ink">
                    <span class="w-7 h-7 rounded-full bg-gold text-navy flex items-center justify-center">
                        <span inner_html=icons::LOGO />
                    </span>
                    <span class="text-base font-semibold -tracking-[.3px]">"Photon"</span>
                </a>
                <div class="flex items-center gap-4 ml-auto">
                    <a href="/albums"
                        class="text-sm text-body hover:text-ink transition-colors no-underline flex items-center gap-1.5">
                        <span class="w-4 h-4 flex items-center justify-center" inner_html=icons::FOLDER />
                        <span>"Albums"</span>
                    </a>
                    <div class="flex items-center gap-1">
                        <button
                            on:click=move |_| app.set_view_mode("grid")
                            class="w-7 h-7 flex items-center justify-center rounded-sm transition-all"
                            class:bg-surf=move || app.view_mode().get() == "grid"
                            class:text-body=move || app.view_mode().get() != "grid"
                            title="Grid"
                        >
                            <span inner_html=icons::GRID />
                        </button>
                        <button
                            on:click=move |_| app.set_view_mode("list")
                            class="w-7 h-7 flex items-center justify-center rounded-sm transition-all"
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
