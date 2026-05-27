use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::utils::api;

#[derive(Clone, Copy)]
pub struct AlbumStore {
    items: RwSignal<Vec<api::Album>>,
    show_modal: RwSignal<bool>,
    new_name: RwSignal<String>,
}

impl AlbumStore {
    pub fn new() -> Self {
        Self {
            items: RwSignal::new(vec![]),
            show_modal: RwSignal::new(false),
            new_name: RwSignal::new(String::new()),
        }
    }

    pub fn items(&self) -> RwSignal<Vec<api::Album>> {
        self.items
    }

    pub fn show_modal(&self) -> RwSignal<bool> {
        self.show_modal
    }

    pub fn new_name(&self) -> RwSignal<String> {
        self.new_name
    }

    pub fn load(&self) {
        let items = self.items;
        spawn_local(async move {
            if let Ok(list) = api::fetch_albums().await {
                items.set(list);
            }
        });
    }

    pub fn set_show_modal(&self, v: bool) {
        self.show_modal.set(v);
    }

    pub fn set_new_name(&self, name: &str) {
        self.new_name.set(name.to_string());
    }
}
