use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::i18n::*;
use crate::utils::api;

#[allow(non_snake_case)]
#[component]
pub fn UploadZone(on_upload: RwSignal<bool>) -> impl IntoView {
    let uploading = RwSignal::new(false);
    let progress = RwSignal::new(0u8);
    let toast = use_context::<RwSignal<String>>();

    let handle_file = move |file: web_sys::File| {
        uploading.set(true);
        progress.set(0);
        let p = progress;
        let up = uploading;
        let ou = on_upload;
        let t = toast;

        leptos::task::spawn_local(async move {
            match api::upload_file(file).await {
                Ok(_) => {
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
                        msg.set(format!("{}: {e}", tr(I18nKey::UploadError)));
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
        let files = js_sys::Reflect::get(input, &"files".into()).unwrap();
        let files = files.unchecked_into::<web_sys::FileList>();
        if let Some(file) = files.get(0) {
            handle_file(file);
        }
    };

    view! {
        <div class="border border-dashed border-hl2 rounded-md p-5 \
                    flex items-center gap-4 flex-wrap transition-all \
                    hover:border-gold hover:bg-goldsoft">
            <span class="text-gold/30">
                <svg width="22" height="22" viewBox="0 0 22 22" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                    <path d="M11 3.5v10M6.5 9.5L11 5l4.5 4.5"/>
                    <path d="M3.5 15v2a2 2 0 002 2h11a2 2 0 002-2v-2"/>
                </svg>
            </span>
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
