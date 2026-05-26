use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::{PhotoModal, UploadZone};
use crate::i18n::*;
use crate::stores::AppCtx;
use crate::utils::api;

#[allow(non_snake_case)]
#[component]
pub fn GalleryPage() -> impl IntoView {
    let AppCtx { app, media } = expect_context();
    let on_upload = RwSignal::new(false);
    let selected = RwSignal::new(None::<usize>);

    Effect::new(move |_| {
        if on_upload.get() {
            on_upload.set(false);
            spawn_local(async move {
                if let Ok(items) = api::fetch_media().await {
                    media.set_items(items);
                }
            });
        }
    });

    Effect::new(move |_| {
        spawn_local(async move {
            if let Ok(items) = api::fetch_media().await {
                media.set_items(items);
            }
        });
    });

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "Escape" => selected.set(None),
            "ArrowLeft" => {
                if let Some(idx) = selected.get() {
                    if idx > 0 { selected.set(Some(idx - 1)); }
                }
            }
            "ArrowRight" => {
                if let Some(idx) = selected.get() {
                    if idx + 1 < media.items().get().len() { selected.set(Some(idx + 1)); }
                }
            }
            _ => {}
        }
    };

    let filters = ["all", "photo", "video"];
    let filter_labels = [tr(I18nKey::FilterAll), tr(I18nKey::FilterPhoto), tr(I18nKey::FilterVideo)];

    let is_grid = move || app.view_mode().get() == "grid";

    let on_search = move |ev: leptos::ev::Event| {
        let input = event_target::<web_sys::HtmlInputElement>(&ev);
        media.set_search_query(&input.value());
    };

    view! {
        <div class="max-w-[1200px] mx-auto px-5" on:keydown=on_keydown>
            <div class="pt-10 pb-5 text-center">
                <h1 class="text-[34px] font-semibold -tracking-[.8px] text-ink mb-1">
                    <em class="not-italic text-gold">{tr(I18nKey::AppTitle)}</em>
                    <span class="font-light text-body/70">{" — "}{tr(I18nKey::AppSubtitle)}</span>
                </h1>
                <p class="text-sm text-mute font-light max-w-[440px] mx-auto mb-5">
                    {tr(I18nKey::AppSubtitle)}
                </p>
            </div>

            <div class="mb-4">
                <UploadZone on_upload=on_upload />
            </div>

            <div class="flex gap-1 flex-wrap items-center mb-4">
                {filters.iter().enumerate().map(|(i, f)| {
                    let f = *f;
                    let label = filter_labels[i];
                    let is_active = move || media.filter().get() == f;
                    let is_not = move || !is_active();
                    view! {
                        <button
                            on:click=move |_| media.set_filter(f)
                            class="h-7 px-3.5 rounded-full border text-xs font-normal \
                                   cursor-pointer transition-all whitespace-nowrap \
                                   focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gold/50"
                            class:border-hl2=is_active
                            class:bg-surf=is_active
                            class:text-ink=is_active
                            class:border-transparent=is_not
                            class:text-body=is_not
                        >
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}

                <div class="ml-auto flex items-center gap-2">
                    <input type="text"
                        placeholder={tr(I18nKey::SearchPlaceholder)}
                        prop:value=move || media.search_query().get()
                        on:input=on_search
                        class="h-9 bg-surf border border-hl rounded-full px-4 \
                               text-sm text-ink outline-none placeholder:text-mute \
                               focus:border-gold transition-all w-[180px]"
                    />
                    <button
                        on:click=move |_| app.set_view_mode("grid")
                        class="w-7 h-7 flex items-center justify-center rounded-sm transition-all"
                        class:bg-surf=is_grid
                        class:text-body=move || !is_grid()
                        title={tr(I18nKey::ViewGrid)}
                    >
                        <svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5">
                            <path d="M2 2h3v3H2zM8 2h3v3H8zM2 8h3v3H2zM8 8h3v3H8z"/>
                        </svg>
                    </button>
                    <button
                        on:click=move |_| app.set_view_mode("list")
                        class="w-7 h-7 flex items-center justify-center rounded-sm transition-all"
                        class:bg-surf=move || !is_grid()
                        class:text-body=is_grid
                        title={tr(I18nKey::ViewList)}
                    >
                        <svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5">
                            <path d="M2 3.5h9M2 6.5h9M2 9.5h9"/>
                        </svg>
                    </button>
                </div>
            </div>

            {move || {
                let items = media.filtered_items().get();
                let grid = is_grid();

                if items.is_empty() {
                    return view! { <div class="text-center text-body py-20">{tr(I18nKey::EmptyGallery)}</div> }.into_any();
                }

                if grid {
                    view! {
                        <div class="columns-4 max-[1080px]:columns-3 max-[740px]:columns-2 max-[440px]:columns-1 gap-2">
                            {items.into_iter().enumerate().map(|(_gi, item)| {
                                view! {
                                    <div class="break-inside-avoid mb-2 rounded-sm overflow-hidden \
                                                cursor-pointer relative bg-surf border border-hl \
                                                transition-all hover:scale-[1.02] hover:shadow-lg hover:shadow-black/30"
                                        on:click=move |_| selected.set(Some(_gi))>
                                        <div class="bg-navy/30 h-32 flex items-center justify-center text-body">
                                            {if item.mime_type.starts_with("video") {
                                                view! { <svg width="18" height="18" viewBox="0 0 18 18" fill="currentColor" class="opacity-40"><polygon points="6,3 15,9 6,15"/></svg> }.into_any()
                                            } else {
                                                view! { <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor" stroke-width="1.5" class="opacity-40"><rect x="3" y="4" width="12" height="10" rx="1.5"/><circle cx="7.5" cy="8.5" r="2"/><path d="M3 12l4.5-4.5 3 3 3-2.5 4.5 4"/></svg> }.into_any()
                                            }}
                                        </div>
                                        <div class="absolute bottom-0 left-0 right-0 pt-9 pb-2.5 px-3 \
                                                    bg-gradient-to-t from-navy/70 to-transparent text-ink">
                                            <div class="text-xs font-medium">{item.original_name.clone()}</div>
                                            <div class="text-[10px] text-mute mt-0.5">{item.created_at.clone()}</div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex flex-col gap-2">
                            {items.into_iter().enumerate().map(|(_gi, item)| {
                                view! {
                                    <div class="flex items-center gap-3 p-2 rounded-sm \
                                                cursor-pointer bg-surf border border-hl \
                                                transition-all hover:border-hl2"
                                        on:click=move |_| selected.set(Some(_gi))>
                                        <div class="w-12 h-12 shrink-0 bg-navy/30 rounded \
                                                    flex items-center justify-center">
                                            {if item.mime_type.starts_with("video") {
                                                view! { <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" class="opacity-40"><polygon points="5,2 13,8 5,14"/></svg> }.into_any()
                                            } else {
                                                view! { <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" class="opacity-40"><rect x="2" y="3" width="12" height="10" rx="1.5"/><circle cx="6.5" cy="7.5" r="1.5"/><path d="M2 11l4-4 2.5 2.5 2-2L15 12"/></svg> }.into_any()
                                            }}
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <div class="text-sm font-medium text-ink truncate">{item.original_name.clone()}</div>
                                            <div class="text-xs text-mute">{item.created_at.clone()}</div>
                                        </div>
                                        <div class="text-[10px] text-body shrink-0">{item.mime_type}</div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }}

            <PhotoModal items=media.items().get() index=selected />
        </div>
    }
}
