use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::i18n::*;
use crate::stores::{AlbumStore, ToastStore};
use crate::utils::api;
use crate::utils::icons;

const OUTER: &str = "max-w-[1200px] mx-auto px-5 pt-10 pb-12";
const HEADER_ROW: &str = "flex items-center justify-between mb-6";
const HEADING: &str = "text-lg font-semibold text-ink";
const CREATE_BTN: &str = "text-xs font-normal text-body hover:text-ink transition-colors";
const GRID: &str = "grid gap-3";
const CARD: &str = "border border-hl rounded-sm p-4 text-center cursor-pointer bg-transparent hover:border-hl2 hover:bg-surf transition-all duration-150";
const CARD_ICON: &str =
    "w-9 h-9 rounded-full mx-auto mb-2 flex items-center justify-center bg-surf text-gold/60";
const CARD_NAME: &str = "text-sm font-semibold text-ink";
const CARD_LABEL: &str = "text-[11px] text-body/70 mt-0.5";
const ADD_CARD: &str = "border border-dashed border-hl rounded-sm p-4 text-center cursor-pointer hover:border-gold hover:bg-goldsoft transition-all duration-150";
const ADD_ICON: &str = "w-9 h-9 rounded-full mx-auto mb-2 flex items-center justify-center bg-transparent border border-dashed border-hl2 text-gold/25";
const ADD_NAME: &str = "text-sm font-semibold text-ink";
const ADD_LABEL: &str = "text-[11px] text-body/70 mt-0.5";
const MODAL_OVERLAY: &str =
    "fixed inset-0 bg-black/50 backdrop-blur-sm z-40 flex items-center justify-center";
const MODAL_BOX: &str = "bg-surf3 border border-hl2 rounded-md p-6 max-w-sm w-11/12";
const MODAL_TITLE: &str = "text-lg font-semibold text-ink mb-3";
const MODAL_LABEL: &str = "text-xs text-body block mb-1";
const MODAL_INPUT: &str = "w-full h-10 px-3 bg-surf border border-hl rounded-sm text-sm text-ink outline-none focus:border-gold focus:ring-1 focus:ring-gold/30 mb-3";
const BTN_ROW: &str = "flex gap-2 justify-end mt-2";
const CANCEL_BTN: &str = "px-4 py-2 rounded-sm text-xs font-medium bg-surf2 text-body hover:text-ink hover:bg-surf3 transition-all";
const CONFIRM_BTN: &str =
    "px-4 py-2 rounded-sm text-xs font-medium bg-gold text-navy hover:bg-gold/80 transition-all";

#[allow(non_snake_case)]
#[component]
pub fn AlbumsPage() -> impl IntoView {
    let album = expect_context::<AlbumStore>();
    let toast = expect_context::<ToastStore>();

    Resource::new(
        || (),
        move |_| async move {
            album.load();
        },
    );

    let create = move || {
        let name = album.new_name().get();
        if name.trim().is_empty() {
            return;
        }
        spawn_local(async move {
            match api::create_album(&name, None).await {
                Ok(_) => {
                    album.set_show_modal(false);
                    album.set_new_name("");
                    let success = tr(I18nKey::AlbumCreateSuccess);
                    toast.show(&format!("{success} \"{name}\""), 3000);
                    album.load();
                }
                Err(e) => {
                    let err = tr(I18nKey::UploadError);
                    toast.show(&format!("{err}: {e}"), 4000);
                }
            }
        });
    };

    view! {
        <div class=OUTER>
            <div class=HEADER_ROW>
                <h2 class=HEADING>{tr(I18nKey::AlbumsTitle)}</h2>
                <button
                    on:click=move |_| { album.set_new_name(""); album.set_show_modal(true); }
                    class=CREATE_BTN
                >
                    {let create = tr(I18nKey::CreateAlbum); format!("+ {create}")}
                </button>
            </div>

            <div class=GRID style="grid-template-columns:repeat(auto-fill,minmax(170px,1fr))">
                {move || album.items().get().into_iter().map(|a| {
                    view! {
                        <div class=CARD>
                            <div class=CARD_ICON inner_html=icons::FOLDER />
                            <div class=CARD_NAME>{a.name}</div>
                            <div class=CARD_LABEL>{tr(I18nKey::AlbumLabel)}</div>
                        </div>
                    }
                }).collect_view()}

                <div class=ADD_CARD
                    on:click=move |_| { album.set_new_name(""); album.set_show_modal(true); }>
                    <div class=ADD_ICON>
                        <span inner_html=icons::PLUS />
                    </div>
                    <div class=ADD_NAME>{tr(I18nKey::CreateAlbum)}</div>
                    <div class=ADD_LABEL>{tr(I18nKey::AlbumNew)}</div>
                </div>
            </div>
        </div>

        {move || album.show_modal().get().then(|| {
            view! {
                <div class=MODAL_OVERLAY
                    on:click=move |ev| {
                        if ev.target() == ev.current_target() {
                            album.set_show_modal(false);
                        }
                    }>
                    <div class=MODAL_BOX>
                        <h3 class=MODAL_TITLE>{tr(I18nKey::CreateAlbum)}</h3>
                        <label class=MODAL_LABEL>{tr(I18nKey::AlbumNamePlaceholder)}</label>
                        <input type="text"
                            prop:value=move || album.new_name().get()
                            on:input=move |ev| {
                                let input = event_target::<web_sys::HtmlInputElement>(&ev);
                                album.set_new_name(&input.value());
                            }
                            class=MODAL_INPUT
                            placeholder={tr(I18nKey::AlbumNamePlaceholder)}
                        />
                        <div class=BTN_ROW>
                            <button
                                on:click=move |_| album.set_show_modal(false)
                                class=CANCEL_BTN
                            >
                                {tr(I18nKey::ModalClose)}
                            </button>
                            <button
                                on:click=move |_| create()
                                class=CONFIRM_BTN
                            >
                                {tr(I18nKey::AlbumCreate)}
                            </button>
                        </div>
                    </div>
                </div>
            }.into_any()
        })}
    }
}
