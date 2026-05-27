use leptos::prelude::*;

const TOAST_CLASS: &str = "fixed bottom-6 left-1/2 -translate-x-1/2 z-50 \
                   bg-surf3 border border-hl2 text-ink \
                   px-5 py-2.5 rounded-sm text-sm shadow-2xl \
                   flex items-center gap-2 \
                   transition-all duration-300 pointer-events-none";

#[allow(non_snake_case)]
#[component]
pub fn Toast(message: RwSignal<String>) -> impl IntoView {
    let visible = move || !message.get().is_empty();

    view! {
        <div
            class=TOAST_CLASS
            class:opacity-0=move || !visible()
            class:translate-y-24=move || !visible()
            class:opacity-100=visible
            class:translate-y-0=visible
        >
            { move || message.get() }
        </div>
    }
}
