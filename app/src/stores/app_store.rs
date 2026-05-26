use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AppStore {
    view_mode: RwSignal<String>,
}

impl AppStore {
    pub fn new() -> Self {
        Self {
            view_mode: RwSignal::new("grid".into()),
        }
    }

    pub fn view_mode(&self) -> RwSignal<String> {
        self.view_mode
    }

    pub fn set_view_mode(&self, mode: &str) {
        self.view_mode.set(mode.to_string());
    }
}
