use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::i18n::*;
use crate::utils::api::{self, Album, MediaItem};
use crate::utils::icons;
use crate::utils::storage;

#[allow(non_snake_case)]
#[component]
pub fn PhotoModal(items: Vec<MediaItem>, index: RwSignal<Option<usize>>) -> impl IntoView {
    let toast = use_context::<RwSignal<String>>();
    let close = move || index.set(None);
    let show_album_picker = RwSignal::new(false);
    let albums = RwSignal::new(Vec::<Album>::new());

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
                                    <div class="flex gap-4 pb-1 items-center">
                                        {let fav_id = item.id.clone();
                                        let fav_display = fav_id.clone();
                                        view! {
                                            <button on:click=move |_| {
                                                storage::toggle_fav(&fav_id);
                                                if let Some(t) = toast {
                                                    let msg = if storage::is_faved(&fav_id) {
                                                        tr(I18nKey::FavSaved)
                                                    } else {
                                                        tr(I18nKey::FavUnsaved)
                                                    };
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
                                                {if storage::is_faved(&fav_display) { "★" } else { "☆" }}
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
                                        <div class="relative">
                                            <button on:click=move |_| {
                                                show_album_picker.set(!show_album_picker.get());
                                                if show_album_picker.get() {
                                                    spawn_local(async move {
                                                        if let Ok(list) = api::fetch_albums().await {
                                                            albums.set(list);
                                                        }
                                                    });
                                                }
                                            }
                                                class="bg-none border-none text-xs text-body cursor-pointer \
                                                       flex items-center gap-1 hover:text-ink transition-colors">
                                                <span inner_html=icons::FOLDER />
                                                {tr(I18nKey::AddToAlbum)}
                                            </button>
                                            {move || show_album_picker.get().then(|| {
                                                let album_list = albums.get();
                                                view! {
                                                    <div class="absolute bottom-full mb-1 left-0 bg-surf3 border border-hl2 \
                                                                rounded-sm shadow-xl min-w-[160px] max-h-[180px] overflow-y-auto z-20">
                                                        {album_list.into_iter().map(|album| {
                                                            let album_id = album.id.clone();
                                                            let album_name = album.name.clone();
                                                            let media_id = item.id.clone();
                                                            let media_name = item.original_name.clone();
                                                            let display_name = album_name.clone();
                                                            let _ = media_name;
                                                            view! {
                                                                <button on:click=move |_| {
                                                                    let aid = album_id.clone();
                                                                    let mid = media_id.clone();
                                                                    let aname = album_name.clone();
                                                                    let mname = media_name.clone();
                                                                    spawn_local(async move {
                                                                        match api::assign_media_to_album(&mid, &aid).await {
                                                                            Ok(_) => {
                                                                                if let Some(t) = toast {
                                                                                    t.set(format!("{} \"{}\" -> \"{}\"", tr(I18nKey::AddedToAlbum), mname, aname));
                                                                                    let t2 = t;
                                                                                    spawn_local(async move {
                                                                                        TimeoutFuture::new(3000).await;
                                                                                        t2.set(String::new());
                                                                                    });
                                                                                }
                                                                            }
                                                                            Err(e) => {
                                                                                if let Some(t) = toast {
                                                                                    t.set(format!("{}: {e}", tr(I18nKey::UploadError)));
                                                                                    let t2 = t;
                                                                                    spawn_local(async move {
                                                                                        TimeoutFuture::new(4000).await;
                                                                                        t2.set(String::new());
                                                                                    });
                                                                                }
                                                                            }
                                                                        }
                                                                    });
                                                                    show_album_picker.set(false);
                                                                }
                                                                    class="w-full text-left px-3 py-2 text-sm text-body \
                                                                           hover:text-ink hover:bg-surf2 transition-colors \
                                                                           border-b border-hl last:border-b-0 flex items-center gap-2">
                                                                    <span class="w-4 h-4 flex items-center justify-center text-gold/50" inner_html=icons::FOLDER />
                                                                    {display_name}
                                                                </button>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                }.into_any()
                                            })}
                                        </div>
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
