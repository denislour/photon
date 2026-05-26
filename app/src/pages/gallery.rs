use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn GalleryPage() -> impl IntoView {
    view! {
        <div class="max-w-[1200px] mx-auto px-5 pt-10 pb-12">
            <h1 class="text-[34px] font-semibold -tracking-[.8px] text-ink text-center mb-1">
                <em class="not-italic text-gold">"KhangHeo"</em>
                <span class="font-light text-body/70">" — những khoảnh khắc"</span>
            </h1>
            <p class="text-sm text-mute font-light text-center max-w-[440px] mx-auto mb-8">
                "Lưu giữ kỷ niệm của bé · Ảnh & video ngắn · An toàn trên mây"
            </p>
            <div class="text-center text-body py-20">
                "Gallery coming soon..."
            </div>
        </div>
    }
}
