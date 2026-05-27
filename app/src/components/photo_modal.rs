use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::i18n::*;
use crate::stores::{AlbumStore, ToastStore};
use crate::utils::api::{self, Album, MediaItem};
use crate::utils::icons;
use crate::utils::storage;

const OVERLAY: &str = "\
    fixed inset-0 bg-black/60 backdrop-blur-sm \
    z-40 flex items-center justify-center";

const PANEL: &str = "\
    bg-surf3 border border-hl2 rounded-md max-w-3xl w-[94%] \
    max-h-[88vh] flex flex-col relative shadow-2xl";

const CLOSE_BTN: &str = "\
    absolute top-2.5 right-2.5 w-7 h-7 rounded-full \
    bg-surf border border-hl text-body flex items-center \
    justify-center z-10 hover:text-ink hover:bg-surf2 transition-all";

const NAV_BTN_L: &str = "\
    absolute top-1/2 -translate-y-1/2 left-2.5 w-8 h-8 rounded-full \
    bg-navy/70 border border-hl text-body flex items-center \
    justify-center z-10 hover:text-ink hover:bg-navy/90 transition-all";

const NAV_BTN_R: &str = "\
    absolute top-1/2 -translate-y-1/2 right-2.5 w-8 h-8 rounded-full \
    bg-navy/70 border border-hl text-body flex items-center \
    justify-center z-10 hover:text-ink hover:bg-navy/90 transition-all";

const ACTION_BTN: &str = "\
    bg-none border-none cursor-pointer text-sm \
    flex items-center gap-1 transition-opacity";

const DELETE_BTN: &str = "\
    bg-none border-none text-danger opacity-50 cursor-pointer \
    text-sm flex items-center gap-1 hover:opacity-100 transition-opacity";

const ICON_BTN: &str = "\
    bg-none border-none text-xs text-body cursor-pointer \
    flex items-center gap-1 hover:text-ink transition-colors";

const PICKER_ITEM: &str = "\
    w-full text-left px-3 py-2 text-sm text-body \
    hover:text-ink hover:bg-surf2 transition-colors \
    border-b border-hl last:border-b-0 flex items-center gap-2";

const THUMB_WRAP: &str = "\
    w-9 h-9 rounded border cursor-pointer shrink-0 \
    overflow-hidden transition-colors duration-200 \
    flex items-center justify-center bg-navy/30";

const THUMB_IMG: &str = "w-full h-full object-cover";

const PICKER_PANEL: &str = "\
    absolute bottom-full mb-1 left-0 bg-surf3 border border-hl2 \
    rounded-sm shadow-xl min-w-[160px] max-h-[180px] overflow-y-auto z-20";

const PICKER_ICON: &str = "\
    w-4 h-4 flex items-center justify-center text-gold/50";

const CONTENT_AREA: &str = "\
    flex-1 overflow-y-auto px-12 py-5 flex flex-col items-center gap-3";

const MODAL_IMG: &str = "\
    w-full min-h-[200px] rounded-sm object-contain bg-navy/30";

const META_ROW: &str = "\
    flex flex-wrap items-center justify-center gap-3 w-full py-1";

const META_BODY: &str = "text-body font-medium text-sm";

const META_TITLE: &str = "text-ink font-medium text-sm";

const META_DATE: &str = "text-mute text-xs";

const ACTION_ROW: &str = "flex gap-4 pb-1 items-center";

const RELATIVE: &str = "relative";

const STRIP: &str = "\
    flex gap-1.5 px-4 pb-3.5 pt-2.5 overflow-x-auto \
    justify-center border-t border-hl";

fn media_thumb(mime_type: String, url: String) -> impl IntoView {
    if mime_type.starts_with("video") {
        view! { <span inner_html=icons::PLAY /> }.into_any()
    } else {
        view! { <img src=url alt="" class=THUMB_IMG /> }.into_any()
    }
}

#[allow(non_snake_case)]
#[component]
fn AlbumPickerModal(
    albums: Vec<Album>,
    media_id: String,
    media_name: String,
    on_close: std::rc::Rc<dyn Fn()>,
) -> impl IntoView {
    view! {
        <div class=PICKER_PANEL>
            {albums.into_iter().map(|album| {
                let aid = album.id.clone();
                let aname = album.name.clone();
                let display = album.name.clone();
                let mid = media_id.clone();
                let mname = media_name.clone();
                let oc = on_close.clone();
                let added = tr(I18nKey::AddedToAlbum);
                let err_label = tr(I18nKey::UploadError);
                view! {
                    <button on:click=move |_| {
                        let aid = aid.clone();
                        let mid = mid.clone();
                        let aname = aname.clone();
                        let mname = mname.clone();
                        spawn_local(async move {
                            match api::assign_media_to_album(&mid, &aid).await {
                                Ok(_) => {
                                    let toast = expect_context::<ToastStore>();
                                    toast.show(&format!("{added} \"{mname}\" -> \"{aname}\""), 3000);
                                }
                                Err(e) => {
                                    let toast = expect_context::<ToastStore>();
                                    toast.show(&format!("{err_label}: {e}"), 4000);
                                }
                            }
                        });
                        oc();
                    } class=PICKER_ITEM>
                        <span class=PICKER_ICON inner_html=icons::FOLDER />
                        {display}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn PhotoModal(items: Vec<MediaItem>, index: RwSignal<Option<usize>>) -> impl IntoView {
    let album_store = expect_context::<AlbumStore>();
    let toast = expect_context::<ToastStore>();
    let show_picker = RwSignal::new(false);

    let close = move || {
        show_picker.set(false);
        index.set(None);
    };

    let on_overlay = move |ev: leptos::ev::MouseEvent| {
        if ev.target() == ev.current_target() {
            close();
        }
    };

    let _toggle_picker = move |_media_item: &MediaItem| {
        show_picker.set(!show_picker.get());
        if show_picker.get() {
            album_store.load();
        }
    };

    view! {
        {move || {
            let idx = index.get()?;
            let total = items.len();
            let item = items[idx].clone();
            let src = api::media_url(&item.id);

            let on_fav = {
                let fid = item.id.clone();
                move |_| {
                    storage::toggle_fav(&fid);
                    let msg = if storage::is_faved(&fid) {
                        tr(I18nKey::FavSaved)
                    } else {
                        tr(I18nKey::FavUnsaved)
                    };
                    toast.show(msg, 3000);
                }
            };

            let on_delete = {
                move |_| {
                    toast.show(tr(I18nKey::ToastDeleted), 3000);
                    close();
                }
            };

            let on_picker_toggle = move |_| {
                show_picker.set(!show_picker.get());
                if show_picker.get() {
                    album_store.load();
                }
            };

            let on_close_picker = {
                let sp = show_picker;
                move || sp.set(false)
            };

            Some(view! {
                <div class=OVERLAY on:click=on_overlay>
                    <div class=PANEL>
                        <button on:click=move |_| close() class=CLOSE_BTN>
                            <span inner_html=icons::CLOSE />
                        </button>

                        {if idx > 0 {
                            view! {
                                <button on:click=move |_| index.set(Some(idx - 1))
                                    class=NAV_BTN_L>
                                    <span inner_html=icons::CHEVRON_LEFT />
                                </button>
                            }.into_any()
                        } else { ().into_any() }}

                        {if idx + 1 < total {
                            view! {
                                <button on:click=move |_| index.set(Some(idx + 1))
                                    class=NAV_BTN_R>
                                    <span inner_html=icons::CHEVRON_RIGHT />
                                </button>
                            }.into_any()
                        } else { ().into_any() }}

                        <div class=CONTENT_AREA>
                            <img src=src alt=item.original_name.clone() class=MODAL_IMG />
                            <div class=META_ROW>
                                <span class=META_BODY>{format!("{}/{}", idx + 1, total)}</span>
                                <span class=META_TITLE>{item.original_name.clone()}</span>
                                <span class=META_DATE>{item.created_at.clone()}</span>
                            </div>

                            <div class=ACTION_ROW>
                                <button on:click=on_fav class=ACTION_BTN>
                                    {if storage::is_faved(&item.id) { "★" } else { "☆" }}
                                </button>

                                <button on:click=on_delete class=DELETE_BTN>
                                    {tr(I18nKey::ModalDelete)}
                                </button>

                                <div class=RELATIVE>
                                    <button on:click=on_picker_toggle class=ICON_BTN>
                                        <span inner_html=icons::FOLDER />
                                        {tr(I18nKey::AddToAlbum)}
                                    </button>
                                    {move || show_picker.get().then(|| {
                                        let al = album_store.items().get();
                                        let mid = item.id.clone();
                                        let mn = item.original_name.clone();
                                        view! {
                                            <AlbumPickerModal
                                                albums=al
                                                media_id=mid
                                                media_name=mn
                                                on_close=std::rc::Rc::new(on_close_picker.clone())
                                            />
                                        }.into_any()
                                    })}
                                </div>
                            </div>
                        </div>

                        <div class=STRIP>
                            {items.iter().enumerate().map(|(i, ph)| {
                                let is_active = i == idx;
                                let thumb_src = api::media_url(&ph.id);
                                view! {
                                    <div on:click=move |_| index.set(Some(i))
                                        class=THUMB_WRAP
                                        class:border-gold=is_active
                                        class:border-transparent=!is_active>
                                        {media_thumb(ph.mime_type.clone(), thumb_src)}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            })
        }}
    }
}
