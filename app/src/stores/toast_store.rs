use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Clone, Copy)]
pub struct ToastStore {
    message: RwSignal<String>,
}

impl ToastStore {
    pub fn new() -> Self {
        Self {
            message: RwSignal::new(String::new()),
        }
    }

    pub fn message(&self) -> RwSignal<String> {
        self.message
    }

    pub fn show(&self, msg: &str, ms: u32) {
        self.message.set(msg.to_string());
        let msg = self.message;
        spawn_local(async move {
            TimeoutFuture::new(ms).await;
            msg.set(String::new());
        });
    }
}
