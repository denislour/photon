use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::i18n::*;
use crate::utils::api;

#[allow(non_snake_case)]
#[component]
pub fn AlbumsPage() -> impl IntoView {
    let albums = RwSignal::new(Vec::<api::Album>::new());
    let show_modal = RwSignal::new(false);
    let new_name = RwSignal::new(String::new());
    let toast = use_context::<RwSignal<String>>();

    let load = move || {
        spawn_local(async move {
            if let Ok(list) = api::fetch_albums().await {
                albums.set(list);
            }
        });
    };

    load();

    let create = move || {
        let name = new_name.get();
        if name.trim().is_empty() { return; }
        spawn_local(async move {
            match api::create_album(&name, None).await {
                Ok(_) => {
                    show_modal.set(false);
                    new_name.set(String::new());
                    if let Some(t) = toast {
                        t.set(format!("{} \"{name}\"", tr(I18nKey::AlbumCreateSuccess)));
                        let t2 = t;
                        set_timeout(move || t2.set(String::new()), std::time::Duration::from_secs(3));
                    }
                    load();
                }
                Err(e) => {
                    if let Some(t) = toast {
                        t.set(format!("{}: {e}", tr(I18nKey::UploadError)));
                        let t2 = t;
                        set_timeout(move || t2.set(String::new()), std::time::Duration::from_secs(4));
                    }
                }
            }
        });
    };

    view! {
        <div class="max-w-[1200px] mx-auto px-5 pt-10 pb-12">
            <div class="flex items-center justify-between mb-6">
                <h2 class="text-lg font-semibold text-ink">{tr(I18nKey::AlbumsTitle)}</h2>
                <button
                    on:click=move |_| { new_name.set(String::new()); show_modal.set(true); }
                    class="text-xs font-normal text-body hover:text-ink transition-colors"
                >
                    {format!("+ {}", tr(I18nKey::CreateAlbum))}
                </button>
            </div>

            <div class="grid gap-3" style="grid-template-columns:repeat(auto-fill,minmax(170px,1fr))">
                {move || albums.get().into_iter().map(|album| {
                    view! {
                        <div class="border border-hl rounded-sm p-4 text-center cursor-pointer \
                                    bg-transparent hover:border-hl2 hover:bg-surf transition-all duration-150">
                            <div class="w-9 h-9 rounded-full mx-auto mb-2 flex items-center justify-center \
                                        bg-surf text-gold/60">
                                <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                                    <path d="M2 4.5v8a1 1 0 001 1h10a1 1 0 001-1V6a1 1 0 00-1-1H8.5L7 3.5H3a1 1 0 00-1 1z"/>
                                </svg>
                            </div>
                            <div class="text-sm font-semibold text-ink">{album.name}</div>
                            <div class="text-[11px] text-body/70 mt-0.5">{tr(I18nKey::AlbumLabel)}</div>
                        </div>
                    }
                }).collect::<Vec<_>>()}

                <div class="border border-dashed border-hl rounded-sm p-4 text-center cursor-pointer \
                            hover:border-gold hover:bg-goldsoft transition-all duration-150"
                    on:click=move |_| { new_name.set(String::new()); show_modal.set(true); }>
                    <div class="w-9 h-9 rounded-full mx-auto mb-2 flex items-center justify-center \
                                bg-transparent border border-dashed border-hl2 text-gold/25">
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                            <path d="M8 3v10M3 8h10"/>
                        </svg>
                    </div>
                    <div class="text-sm font-semibold text-ink">{tr(I18nKey::CreateAlbum)}</div>
                    <div class="text-[11px] text-body/70 mt-0.5">{tr(I18nKey::AlbumNew)}</div>
                </div>
            </div>
        </div>

        {move || show_modal.get().then(|| {
            view! {
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-40 flex items-center justify-center"
                    on:click=move |ev| {
                        if ev.target() == ev.current_target() {
                            show_modal.set(false);
                        }
                    }>
                    <div class="bg-surf3 border border-hl2 rounded-md p-6 max-w-sm w-11/12">
                        <h3 class="text-lg font-semibold text-ink mb-3">{tr(I18nKey::CreateAlbum)}</h3>
                        <label class="text-xs text-body block mb-1">{tr(I18nKey::AlbumNamePlaceholder)}</label>
                        <input type="text"
                            prop:value=move || new_name.get()
                            on:input=move |ev| {
                                let input = event_target::<web_sys::HtmlInputElement>(&ev);
                                new_name.set(input.value());
                            }
                            class="w-full h-10 px-3 bg-surf border border-hl rounded-sm \
                                   text-sm text-ink outline-none focus:border-gold \
                                   focus:ring-1 focus:ring-gold/30 mb-3"
                            placeholder={tr(I18nKey::AlbumNamePlaceholder)}
                        />
                        <div class="flex gap-2 justify-end mt-2">
                            <button
                                on:click=move |_| show_modal.set(false)
                                class="px-4 py-2 rounded-sm text-xs font-medium \
                                       bg-surf2 text-body hover:text-ink hover:bg-surf3 transition-all"
                            >
                                {tr(I18nKey::ModalClose)}
                            </button>
                            <button
                                on:click=move |_| create()
                                class="px-4 py-2 rounded-sm text-xs font-medium \
                                       bg-gold text-navy hover:bg-gold/80 transition-all"
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
