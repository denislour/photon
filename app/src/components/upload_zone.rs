use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::i18n::*;
use crate::utils::api::{self, fe_log};
use crate::utils::icons;

#[allow(non_snake_case)]
#[component]
pub fn UploadZone(on_upload: RwSignal<bool>) -> impl IntoView {
    let uploading = RwSignal::new(false);
    let progress = RwSignal::new(0u8);
    let dragover = RwSignal::new(false);
    let toast = use_context::<RwSignal<String>>();

    let handle_file = move |file: web_sys::File| {
        let name = file.name();
        let size = file.size();
        let mime = file.type_();
        fe_log(&format!("[Upload] file selected: name={name}, size={size}, mime={mime}"));

        uploading.set(true);
        progress.set(0);
        let p = progress;
        let up = uploading;
        let ou = on_upload;
        let t = toast;

        leptos::task::spawn_local(async move {
            fe_log("[Upload] starting upload_file API call...");
            match api::upload_file(file).await {
                Ok(resp) => {
                    fe_log(&format!("[Upload] success: id={}", resp.id));
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
                    fe_log(&format!("[Upload] ERROR: {e}"));
                    up.set(false);
                    if let Some(msg) = t {
                        msg.set(format!("{}: {e}", tr(I18nKey::UploadError)));
                        TimeoutFuture::new(4000).await;
                        msg.set(String::new());
                    }
                }
            }
        });
    };

    let on_change = move |ev: leptos::ev::Event| {
        fe_log("[Upload] on_change fired");
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
            class="border border-dashed rounded-md p-5 \
                   flex items-center gap-4 flex-wrap transition-all cursor-pointer"
            class:border-gold=dragover
            class:bg-goldsoft=dragover
            class:border-hl2=move || !dragover.get()
        >
            <span class="text-gold/30" inner_html=icons::UPLOAD />
            <div class="flex-1 min-w-[170px]">
                <h3 class="text-sm font-normal text-ink">{tr(I18nKey::UploadHint)}</h3>
                <p class="text-xs text-mute mt-0.5">{tr(I18nKey::UploadFormats)}</p>
            </div>
            <label class="inline-flex items-center gap-1.5 h-8 px-4 rounded-sm \
                          text-xs font-medium bg-gold text-navy \
                          hover:bg-gold/80 transition-all cursor-pointer">
                {tr(I18nKey::UploadButton)}
                <input type="file"
                    accept="image/*,video/*"
                    on:change=on_change
                    class="hidden"
                />
            </label>
            {move || uploading.get().then(|| {
                view! {
                    <div class="w-full">
                        <div class="h-0.5 bg-surf2 rounded overflow-hidden">
                            <div class="h-full bg-gold rounded transition-all duration-300"
                                 style:width=format!("{}%", progress.get())></div>
                        </div>
                        <div class="flex justify-between text-[11px] text-mute mt-1">
                            <span>{tr(I18nKey::UploadProgress)}</span>
                            <span>{move || format!("{}%", progress.get())}</span>
                        </div>
                    </div>
                }.into_any()
            })}
        </div>
    }
}
