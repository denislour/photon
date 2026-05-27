use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::{PhotoModal, UploadZone};
use crate::i18n::*;
use crate::stores::{AppCtx, UploadStore};
use crate::utils::api;
use crate::utils::icons;

const OUTER: &str = "max-w-[1200px] mx-auto px-5";
const HEADER_SECTION: &str = "pt-10 pb-5 text-center";
const HEADING: &str = "text-[34px] font-semibold -tracking-[.8px] text-ink mb-1";
const SUBTITLE: &str = "text-sm text-body/70 font-light max-w-[440px] mx-auto";
const UPLOAD_WRAPPER: &str = "mb-4";
const TOOLBAR: &str = "flex gap-1 flex-wrap items-center mb-4";
const FILTER_BTN: &str = "h-7 px-3.5 rounded-full border text-xs font-normal cursor-pointer transition-all whitespace-nowrap focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gold/50";
const SEARCH_INPUT: &str = "h-9 bg-surf border border-hl rounded-full px-4 text-sm text-ink outline-none placeholder:text-mute focus:border-gold transition-all w-[180px]";
const VIEW_BTN: &str = "w-7 h-7 flex items-center justify-center rounded-sm transition-all";
const EMPTY_MSG: &str = "text-center text-body py-20";
const GRID_COLS: &str =
    "columns-4 max-[1080px]:columns-3 max-[740px]:columns-2 max-[440px]:columns-1 gap-2";
const GRID_CARD: &str = "break-inside-avoid mb-2 rounded-sm overflow-hidden cursor-pointer relative bg-surf border border-hl transition-all hover:scale-[1.02] hover:shadow-lg hover:shadow-black/30";
const GRID_PREVIEW: &str = "bg-navy/30 h-52 flex items-center justify-center overflow-hidden";
const PLAY_ICON: &str = "opacity-40";
const GRID_IMG: &str = "w-full h-full object-cover";
const GRID_INFO: &str = "p-2 border-t border-hl bg-surf";
const GRID_NAME: &str = "text-xs font-medium text-ink truncate";
const GRID_DATE: &str = "text-[10px] text-mute";
const LIST_ROW: &str = "flex items-center gap-3 p-2 rounded-sm cursor-pointer bg-surf border border-hl transition-all hover:border-hl2";
const LIST_PREVIEW: &str =
    "w-12 h-12 shrink-0 bg-navy/30 rounded flex items-center justify-center overflow-hidden";
const LIST_INFO: &str = "flex-1 min-w-0";
const LIST_NAME: &str = "text-sm font-medium text-ink truncate";
const LIST_DATE: &str = "text-xs text-mute";
const LIST_TYPE: &str = "text-[10px] text-body shrink-0";
const EMPHASIS: &str = "not-italic text-gold";
const HEAD_SUBTITLE: &str = "font-light text-body/70";
const TOOLBAR_RIGHT: &str = "ml-auto flex items-center gap-2";
const LIST_COL: &str = "flex flex-col gap-2";

#[allow(non_snake_case)]
#[component]
pub fn GalleryPage() -> impl IntoView {
    let AppCtx { app, media, .. } = expect_context();
    let upload = expect_context::<UploadStore>();

    Effect::new(move |_| {
        if upload.on_upload().get() {
            upload.set_on_upload(false);
            spawn_local(async move {
                if let Ok(items) = api::fetch_media().await {
                    media.set_items(items);
                }
            });
        }
    });

    {
        let m = media;
        spawn_local(async move {
            if let Ok(items) = api::fetch_media().await {
                m.set_items(items);
            }
        });
    }

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| match ev.key().as_str() {
        "Escape" => media.set_selected(None),
        "ArrowLeft" => {
            if let Some(idx) = media.selected().get() {
                if idx > 0 {
                    media.set_selected(Some(idx - 1));
                }
            }
        }
        "ArrowRight" => {
            if let Some(idx) = media.selected().get() {
                if idx + 1 < media.items().get().len() {
                    media.set_selected(Some(idx + 1));
                }
            }
        }
        _ => {}
    };

    let filters = ["all", "photo", "video"];
    let filter_labels = [
        tr(I18nKey::FilterAll),
        tr(I18nKey::FilterPhoto),
        tr(I18nKey::FilterVideo),
    ];
    let is_grid = move || app.view_mode().get() == "grid";

    let on_search = move |ev: leptos::ev::Event| {
        let input = event_target::<web_sys::HtmlInputElement>(&ev);
        media.set_search_query(&input.value());
    };

    view! {
        <div class=OUTER on:keydown=on_keydown>
            <div class=HEADER_SECTION>
                <h1 class=HEADING>
                    <em class=EMPHASIS>{tr(I18nKey::AppTitle)}</em>
                    <span class=HEAD_SUBTITLE>{" — "}{tr(I18nKey::AppSubtitle)}</span>
                </h1>
                <p class=SUBTITLE>{tr(I18nKey::AppSubtitle)}</p>
            </div>

            <div class=UPLOAD_WRAPPER><UploadZone /></div>

            <div class=TOOLBAR>
                {filters.iter().enumerate().map(|(i, f)| {
                    let f = *f;
                    let label = filter_labels[i];
                    let is_active = move || media.filter().get() == f;
                    let is_not = move || !is_active();
                    view! {
                        <button on:click=move |_| media.set_filter(f)
                            class=FILTER_BTN
                            class:border-hl2=is_active class:bg-surf=is_active
                            class:text-ink=is_active class:border-transparent=is_not class:text-body=is_not>
                            {label}
                        </button>
                    }
                }).collect_view()}

                <div class=TOOLBAR_RIGHT>
                    <input type="text" placeholder={tr(I18nKey::SearchPlaceholder)}
                        prop:value=move || media.search_query().get()
                        on:input=on_search
                        class=SEARCH_INPUT />
                    <button on:click=move |_| app.set_view_mode("grid")
                        class=VIEW_BTN
                        class:bg-surf=is_grid class:text-body=move || !is_grid() title={tr(I18nKey::ViewGrid)}>
                        <span inner_html=icons::GRID />
                    </button>
                    <button on:click=move |_| app.set_view_mode("list")
                        class=VIEW_BTN
                        class:bg-surf=move || !is_grid() class:text-body=is_grid title={tr(I18nKey::ViewList)}>
                        <span inner_html=icons::LIST />
                    </button>
                </div>
            </div>

            {move || {
                let all_items = media.filtered_items().get();
                let grid = is_grid();
                if all_items.is_empty() {
                    return view! { <div class=EMPTY_MSG>{tr(I18nKey::EmptyGallery)}</div> }.into_any();
                }

                if grid {
                    view! {
                        <>
                            <div class=GRID_COLS>
                                {all_items.iter().enumerate().map(|(gi, item)| {
                                    let gi = gi;
                                    view! {
                                        <div class=GRID_CARD
                                            on:click=move |_| media.set_selected(Some(gi))>
                                            <div class=GRID_PREVIEW>
                                                {if item.mime_type.starts_with("video") {
                                                    view! { <span class=PLAY_ICON inner_html=icons::PLAY /> }.into_any()
                                                } else {
                                                    view! { <img src=api::media_url(&item.id) alt="" class=GRID_IMG /> }.into_any()
                                                }}
                                            </div>
                                            <div class=GRID_INFO>
                                                <div class=GRID_NAME>{item.original_name.clone()}</div>
                                                <div class=GRID_DATE>{item.created_at.clone()}</div>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                            <PhotoModal items=all_items.clone() index=media.selected() />
                        </>
                    }.into_any()
                } else {
                    view! {
                        <>
                            <div class=LIST_COL>
                                {all_items.iter().enumerate().map(|(gi, item)| {
                                    let gi = gi;
                                    view! {
                                        <div class=LIST_ROW
                                            on:click=move |_| media.set_selected(Some(gi))>
                                            <div class=LIST_PREVIEW>
                                                {if item.mime_type.starts_with("video") {
                                                    view! { <span class=PLAY_ICON inner_html=icons::PLAY /> }.into_any()
                                                } else {
                                                    view! { <img src=api::media_url(&item.id) alt="" class=GRID_IMG /> }.into_any()
                                                }}
                                            </div>
                                            <div class=LIST_INFO>
                                                <div class=LIST_NAME>{item.original_name.clone()}</div>
                                                <div class=LIST_DATE>{item.created_at.clone()}</div>
                                            </div>
                                            <div class=LIST_TYPE>{item.mime_type.clone()}</div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                            <PhotoModal items=all_items.clone() index=media.selected() />
                        </>
                    }.into_any()
                }
            }}
        </div>
    }
}
