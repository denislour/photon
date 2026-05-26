use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::UploadZone;
use crate::stores::AppCtx;
use crate::utils::api;

#[allow(non_snake_case)]
#[component]
pub fn GalleryPage() -> impl IntoView {
    let AppCtx { media, .. } = expect_context();
    let on_upload = RwSignal::new(false);

    let _ = on_upload;

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

    view! {
        <div class="max-w-[1200px] mx-auto px-5">
            <div class="pt-10 pb-5 text-center">
                <h1 class="text-[34px] font-semibold -tracking-[.8px] text-ink mb-1">
                    <em class="not-italic text-gold">"KhangHeo"</em>
                    <span class="font-light text-body/70">" — những khoảnh khắc"</span>
                </h1>
                <p class="text-sm text-mute font-light max-w-[440px] mx-auto mb-5">
                    "Lưu giữ kỷ niệm của bé · Ảnh & video ngắn · An toàn trên mây"
                </p>
            </div>

            <div class="mb-4">
                <UploadZone on_upload=on_upload />
            </div>

            <div id="gallery" class="columns-4 max-[1080px]:columns-3 max-[740px]:columns-2 max-[440px]:columns-1 gap-2">
                {move || media.items().get().into_iter().map(|item| {
                    view! {
                        <div class="break-inside-avoid mb-2 rounded-sm overflow-hidden \
                                    cursor-pointer relative bg-surf border border-hl \
                                    transition-all hover:scale-[1.02] hover:shadow-lg hover:shadow-black/30">
                            <div class="bg-navy/30 h-32 flex items-center justify-center text-body text-xs p-3">
                                {item.original_name.clone()}
                            </div>
                            <div class="absolute bottom-0 left-0 right-0 pt-9 pb-2.5 px-3 \
                                        bg-gradient-to-t from-navy/70 to-transparent text-ink">
                                <div class="text-xs font-medium">{item.original_name}</div>
                                <div class="text-[10px] text-mute mt-0.5">{item.created_at.clone()}</div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
