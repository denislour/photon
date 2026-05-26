use leptos::prelude::*;

use crate::stores::AppCtx;

#[allow(non_snake_case)]
#[component]
pub fn Header() -> impl IntoView {
    let AppCtx { app, .. } = expect_context();

    view! {
        <nav class="sticky top-0 z-30 h-14 bg-navy/88 backdrop-blur-md border-b border-hl">
            <div class="max-w-[1200px] mx-auto px-5 h-full flex items-center gap-3">
                <a href="/" class="flex items-center gap-2.5 shrink-0 no-underline text-ink">
                    <span class="w-7 h-7 rounded-full bg-gold text-navy flex items-center justify-center text-xs font-bold">
                        { "◆" }
                    </span>
                    <span class="text-base font-semibold -tracking-[.3px]">"Photon"</span>
                </a>
                <div class="flex items-center gap-1 ml-auto">
                    <button
                        on:click=move |_| app.set_view_mode("grid")
                        class="w-7 h-7 flex items-center justify-center rounded-sm \
                               text-gold/30 hover:text-gold/70 hover:bg-surf transition-all"
                        title="Grid"
                    >
                        <svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5">
                            <path d="M2 2h3v3H2zM8 2h3v3H8zM2 8h3v3H2zM8 8h3v3H8z"/>
                        </svg>
                    </button>
                    <button
                        on:click=move |_| app.set_view_mode("list")
                        class="w-7 h-7 flex items-center justify-center rounded-sm \
                               text-gold/30 hover:text-gold/70 hover:bg-surf transition-all"
                        title="List"
                    >
                        <svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5">
                            <path d="M2 3.5h9M2 6.5h9M2 9.5h9"/>
                        </svg>
                    </button>
                </div>
            </div>
        </nav>
    }
}
