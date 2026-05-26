use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Store)]
struct AppState {
    view_mode: String,
}

#[derive(Clone, Copy)]
pub struct AppStore {
    state: Store<AppState>,
}

impl AppStore {
    pub fn new() -> Self {
        Self { state: Store::new(AppState { view_mode: "grid".into() }) }
    }

    pub fn view_mode(&self) -> impl Get<Value = String> + Copy + use<> {
        self.state.view_mode()
    }

    pub fn set_view_mode(&self, mode: &str) {
        self.state.view_mode().set(mode.to_string());
    }
}
