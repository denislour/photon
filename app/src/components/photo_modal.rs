use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;

use crate::utils::api::MediaItem;

#[allow(non_snake_case)]
#[component]
pub fn PhotoModal(
    items: Vec<MediaItem>,
    index: RwSignal<Option<usize>>,
) -> impl IntoView {
    let toast = use_context::<RwSignal<String>>();

    let close = move || index.set(None);

    view! {
        {move || {
            let idx = index.get();
            match idx {
                None => None,
                Some(idx) => {
                    let total = items.len();
                    let item = items[idx].clone();

                    let nav = move |dir: i32| {
                        let next = idx as i32 + dir;
                        if next >= 0 && (next as usize) < total {
                            index.set(Some(next as usize));
                        }
                    };

                    Some(view! {
                        <div class="fixed inset-0 bg-black/60 backdrop-blur-sm z-40 flex items-center justify-center"
                            on:click=move |ev| {
                                let target = ev.target();
                                let current = ev.current_target();
                                if target == current {
                                    close();
                                }
                            }>
                            <div class="bg-surf3 border border-hl2 rounded-md max-w-3xl w-[94%] \
                                        max-h-[88vh] flex flex-col relative shadow-2xl">
                                <button
                                    on:click=move |_| close()
                                    class="absolute top-2.5 right-2.5 w-7 h-7 rounded-full \
                                        bg-surf border border-hl text-body flex items-center \
                                        justify-center z-10 hover:text-ink hover:bg-surf2 transition-all"
                                >
                                    <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                                        <path d="M3 3l8 8M11 3l-8 8"/>
                                    </svg>
                                </button>

                                {if idx > 0 {
                                    view! {
                                        <button on:click=move |_| nav(-1)
                                            class="absolute top-1/2 -translate-y-1/2 left-2.5 w-8 h-8 \
                                                rounded-full bg-navy/70 border border-hl text-body \
                                                flex items-center justify-center z-10 hover:text-ink \
                                                hover:bg-navy/90 transition-all">
                                            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                                                <path d="M8.5 3L4 7l4.5 4"/>
                                            </svg>
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                {if idx + 1 < total {
                                    view! {
                                        <button on:click=move |_| nav(1)
                                            class="absolute top-1/2 -translate-y-1/2 right-2.5 w-8 h-8 \
                                                rounded-full bg-navy/70 border border-hl text-body \
                                                flex items-center justify-center z-10 hover:text-ink \
                                                hover:bg-navy/90 transition-all">
                                            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                                                <path d="M5.5 3L10 7l-4.5 4"/>
                                            </svg>
                                        </button>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                <div class="flex-1 overflow-y-auto px-12 py-5 flex flex-col items-center gap-3">
                                    <div class="w-full min-h-[200px] rounded-sm flex items-center justify-center \
                                                bg-navy/30 text-body text-sm">
                                        {item.original_name.clone()}
                                    </div>
                                    <div class="flex flex-wrap items-center justify-center gap-3 w-full py-1">
                                        <span class="text-body font-medium text-sm">
                                            {format!("{}/{}", idx + 1, total)}
                                        </span>
                                        <span class="text-ink font-medium text-sm">{item.original_name.clone()}</span>
                                        <span class="text-mute text-xs">{item.created_at.clone()}</span>
                                    </div>
                                    <div class="flex gap-4 pb-1">
                                        <button
                                            on:click=move |_| {
                                                if let Some(t) = toast {
                                                    t.set("Da luu".into());
                                                    let t2 = t;
                                                    spawn_local(async move {
                                                        TimeoutFuture::new(3000).await;
                                                        t2.set(String::new());
                                                    });
                                                }
                                            }
                                            class="bg-none border-none text-gold opacity-50 cursor-pointer \
                                                text-sm flex items-center gap-1 hover:opacity-100 transition-opacity"
                                        >
                                            {"Luu"}
                                        </button>
                                        <button
                                            on:click=move |_| {
                                                if let Some(t) = toast {
                                                    t.set("Da xoa".into());
                                                    let t2 = t;
                                                    spawn_local(async move {
                                                        TimeoutFuture::new(3000).await;
                                                        t2.set(String::new());
                                                    });
                                                }
                                                close();
                                            }
                                            class="bg-none border-none text-danger opacity-50 cursor-pointer \
                                                text-sm flex items-center gap-1 hover:opacity-100 transition-opacity"
                                        >
                                            {"Xoa"}
                                        </button>
                                    </div>
                                </div>

                                <div class="flex gap-1.5 px-4 pb-3.5 pt-2.5 overflow-x-auto \
                                            justify-center border-t border-hl">
                                    {items.iter().enumerate().map(|(i, ph)| {
                                        let is_active = i == idx;
                                        view! {
                                            <div
                                                on:click=move |_| index.set(Some(i))
                                                class="w-9 h-9 rounded border cursor-pointer shrink-0 \
                                                    overflow-hidden transition-colors duration-200 \
                                                    flex items-center justify-center bg-navy/30 \
                                                    text-[8px] text-body"
                                                class:border-gold=is_active
                                                class:border-transparent=!is_active
                                            >
                                                {if ph.mime_type.starts_with("video") { "[V]" } else { "[I]" }}
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
