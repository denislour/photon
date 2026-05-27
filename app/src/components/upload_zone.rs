use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::i18n::*;
use crate::utils::api;
use crate::utils::icons;

const DIV_CLASS: &str = "border border-dashed rounded-md p-5 \
                          flex items-center gap-4 flex-wrap transition-all cursor-pointer";
const ICON_CLASS: &str = "text-gold/30";
const HINT_WRAPPER_CLASS: &str = "flex-1 min-w-[170px]";
const HINT_TITLE_CLASS: &str = "text-sm font-normal text-ink";
const HINT_DESC_CLASS: &str = "text-xs text-mute mt-0.5";
const LABEL_CLASS: &str = "inline-flex items-center gap-1.5 h-8 px-4 rounded-sm \
                           text-xs font-medium bg-gold text-navy \
                           hover:bg-gold/80 transition-all cursor-pointer";
const HIDDEN_CLASS: &str = "hidden";
const PROGRESS_WRAPPER_CLASS: &str = "w-full";
const PROGRESS_BAR_TRACK_CLASS: &str = "h-0.5 bg-surf2 rounded overflow-hidden";
const PROGRESS_BAR_FILL_CLASS: &str = "h-full bg-gold rounded transition-all duration-300";
const PROGRESS_INFO_CLASS: &str = "flex justify-between text-[11px] text-mute mt-1";

#[allow(non_snake_case)]
#[component]
pub fn UploadZone(on_upload: RwSignal<bool>) -> impl IntoView {
    let uploading = RwSignal::new(false);
    let progress = RwSignal::new(0u8);
    let dragover = RwSignal::new(false);
    let toast = use_context::<RwSignal<String>>();

    let handle_file = move |file: web_sys::File| {
        let _name = file.name();
        let _size = file.size();
        let _mime = file.type_();
        uploading.set(true);
        progress.set(0);
        let p = progress;
        let up = uploading;
        let ou = on_upload;
        let t = toast;

        leptos::task::spawn_local(async move {
            match api::upload_file(file).await {
                Ok(_resp) => {
                    p.set(100);
                    up.set(false);
                    ou.set(true);
                    if let Some(msg) = t {
                        msg.set(tr(I18nKey::UploadSuccess).into());
                        TimeoutFuture::new(3000).await;
                        msg.set(String::new());
                    }
                }
                Err(e) => {
                    up.set(false);
                    if let Some(msg) = t {
                        let err = tr(I18nKey::UploadError);
                        msg.set(format!("{err}: {e}"));
                        TimeoutFuture::new(4000).await;
                        msg.set(String::new());
                    }
                }
            }
        });
    };

    let on_change = move |ev: leptos::ev::Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_ref::<web_sys::HtmlInputElement>();
        let files_val = js_sys::Reflect::get(input, &"files".into()).unwrap();
        let files = files_val.unchecked_into::<web_sys::FileList>();
        if let Some(file) = files.get(0) {
            handle_file(file);
        }
    };

    let on_dragover = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        dragover.set(true);
    };

    let on_dragleave = move |_ev: leptos::ev::DragEvent| {
        dragover.set(false);
    };

    let on_drop = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        dragover.set(false);
        if let Some(data) = ev.data_transfer() {
            if let Some(files) = data.files() {
                if let Some(file) = files.get(0) {
                    handle_file(file);
                }
            }
        }
    };

    view! {
        <div
            on:dragover=on_dragover
            on:dragleave=on_dragleave
            on:drop=on_drop
            class=DIV_CLASS
            class:border-gold=dragover
            class:bg-goldsoft=dragover
            class:border-hl2=move || !dragover.get()
        >
            <span class=ICON_CLASS inner_html=icons::UPLOAD />
            <div class=HINT_WRAPPER_CLASS>
                <h3 class=HINT_TITLE_CLASS>{tr(I18nKey::UploadHint)}</h3>
                <p class=HINT_DESC_CLASS>{tr(I18nKey::UploadFormats)}</p>
            </div>
            <label class=LABEL_CLASS>
                {tr(I18nKey::UploadButton)}
                <input type="file"
                    accept="image/*,video/*"
                    on:change=on_change
                    class=HIDDEN_CLASS
                />
            </label>
            {move || uploading.get().then(|| {
                let pct = progress.get();
                view! {
                    <div class=PROGRESS_WRAPPER_CLASS>
                        <div class=PROGRESS_BAR_TRACK_CLASS>
                            <div class=PROGRESS_BAR_FILL_CLASS
                                 style:width=format!("{pct}%")></div>
                        </div>
                        <div class=PROGRESS_INFO_CLASS>
                            <span>{tr(I18nKey::UploadProgress)}</span>
                            <span>{move || { let pct = progress.get(); format!("{pct}%") }}</span>
                        </div>
                    </div>
                }.into_any()
            })}
        </div>
    }
}
