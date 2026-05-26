use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::i18n::*;
use crate::utils::api::{self, MediaItem};
use crate::utils::icons;

fn is_faved(id: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("photon_favs").ok())
        .flatten()
        .map(|v| v.contains(id))
        .unwrap_or(false)
}

fn toggle_fav(id: &str) {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
    if let Some(s) = storage {
        let current = s.get_item("photon_favs").ok().flatten().unwrap_or_default();
        let next = if current.contains(id) {
            current.replace(id, "").replace(",,", ",").trim_matches(',').to_string()
        } else {
            let mut r = current;
            if !r.is_empty() { r.push(','); }
            r.push_str(id);
            r
        };
        let _ = s.set_item("photon_favs", &next);
    }
}

#[allow(non_snake_case)]
#[component]
pub fn PhotoModal(items: Vec<MediaItem>, index: RwSignal<Option<usize>>) -> impl IntoView {
    let toast = use_context::<RwSignal<String>>();
    let close = move || index.set(None);

    view! {
        {move || {
            match index.get() {
                None => None,
                Some(idx) => {
                    let total = items.len();
                    let item = items[idx].clone();
                    let src = api::media_url(&item.id);

                    Some(view! {
                        <div class="fixed inset-0 bg-black/60 backdrop-blur-sm z-40 flex items-center justify-center"
                            on:click=move |ev| {
                                if ev.target() == ev.current_target() { close(); }
                            }>
                            <div class="bg-surf3 border border-hl2 rounded-md max-w-3xl w-[94%] \
                                        max-h-[88vh] flex flex-col relative shadow-2xl">
                                <button on:click=move |_| close()
                                    class="absolute top-2.5 right-2.5 w-7 h-7 rounded-full \
                                        bg-surf border border-hl text-body flex items-center \
                                        justify-center z-10 hover:text-ink hover:bg-surf2 transition-all">
                                    <span inner_html=icons::CLOSE />
                                </button>

                                {if idx > 0 {
                                    view! { <button on:click=move |_| index.set(Some(idx - 1))
                                        class="absolute top-1/2 -translate-y-1/2 left-2.5 w-8 h-8 \
                                            rounded-full bg-navy/70 border border-hl text-body \
                                            flex items-center justify-center z-10 hover:text-ink \
                                            hover:bg-navy/90 transition-all">
                                        <span inner_html=icons::CHEVRON_LEFT />
                                    </button> }.into_any()
                                } else { ().into_any() }}

                                {if idx + 1 < total {
                                    view! { <button on:click=move |_| index.set(Some(idx + 1))
                                        class="absolute top-1/2 -translate-y-1/2 right-2.5 w-8 h-8 \
                                            rounded-full bg-navy/70 border border-hl text-body \
                                            flex items-center justify-center z-10 hover:text-ink \
                                            hover:bg-navy/90 transition-all">
                                        <span inner_html=icons::CHEVRON_RIGHT />
                                    </button> }.into_any()
                                } else { ().into_any() }}

                                <div class="flex-1 overflow-y-auto px-12 py-5 flex flex-col items-center gap-3">
                                    <img src=src alt=item.original_name.clone()
                                        class="w-full min-h-[200px] rounded-sm object-contain bg-navy/30"
                                    />
                                    <div class="flex flex-wrap items-center justify-center gap-3 w-full py-1">
                                        <span class="text-body font-medium text-sm">{format!("{}/{}", idx + 1, total)}</span>
                                        <span class="text-ink font-medium text-sm">{item.original_name.clone()}</span>
                                        <span class="text-mute text-xs">{item.created_at.clone()}</span>
                                    </div>
                                    <div class="flex gap-4 pb-1">
                                        {let fav_id = item.id.clone();
                                        let fav_display = fav_id.clone();
                                        view! {
                                            <button on:click=move |_| {
                                                    toggle_fav(&fav_id);
                                                    if let Some(t) = toast {
                                                        let msg = if is_faved(&fav_id) { "Da luu" } else { "Da bo luu" };
                                                        t.set(msg.into());
                                                        let t2 = t;
                                                        spawn_local(async move {
                                                            TimeoutFuture::new(3000).await;
                                                            t2.set(String::new());
                                                        });
                                                    }
                                                }
                                                class="bg-none border-none cursor-pointer text-sm \
                                                       flex items-center gap-1 transition-opacity">
                                                {if is_faved(&fav_display) { "★" } else { "☆" }}
                                            </button>
                                        }}
                                        <button on:click=move |_| {
                                            if let Some(t) = toast {
                                                t.set(tr(I18nKey::ToastDeleted).into());
                                                let t2 = t;
                                                spawn_local(async move {
                                                    TimeoutFuture::new(3000).await;
                                                    t2.set(String::new());
                                                });
                                            }
                                            close();
                                        } class="bg-none border-none text-danger opacity-50 cursor-pointer \
                                            text-sm flex items-center gap-1 hover:opacity-100 transition-opacity">
                                            {tr(I18nKey::ModalDelete)}
                                        </button>
                                    </div>
                                </div>

                                <div class="flex gap-1.5 px-4 pb-3.5 pt-2.5 overflow-x-auto \
                                            justify-center border-t border-hl">
                                    {items.iter().enumerate().map(|(i, ph)| {
                                        let is_active = i == idx;
                                        let thumb_src = api::media_url(&ph.id);
                                        view! {
                                            <div on:click=move |_| index.set(Some(i))
                                                class="w-9 h-9 rounded border cursor-pointer shrink-0 \
                                                    overflow-hidden transition-colors duration-200 \
                                                    flex items-center justify-center bg-navy/30"
                                                class:border-gold=is_active
                                                class:border-transparent=!is_active>
                                                {if ph.mime_type.starts_with("video") {
                                                    view! { <span inner_html=icons::PLAY /> }.into_any()
                                                } else {
                                                    view! { <img src=thumb_src alt="" class="w-full h-full object-cover" /> }.into_any()
                                                }}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </div>
                    })
                }
            }
        }}
    }
}
