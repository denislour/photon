use leptos::prelude::*;
use reactive_stores::Store;

use crate::utils::api::MediaItem;

#[derive(Store)]
struct MediaState {
    items: Vec<MediaItem>,
    filter: String,
    search_query: String,
}

#[derive(Clone, Copy)]
pub struct MediaStore {
    state: Store<MediaState>,
}

impl MediaStore {
    pub fn new() -> Self {
        Self {
            state: Store::new(MediaState {
                items: vec![],
                filter: "all".into(),
                search_query: String::new(),
            }),
        }
    }

    pub fn items(&self) -> impl Get<Value = Vec<MediaItem>> + Copy + use<> {
        self.state.items()
    }

    pub fn set_items(&self, items: Vec<MediaItem>) {
        self.state.items().set(items);
    }

    pub fn filter(&self) -> impl Get<Value = String> + Copy + use<> {
        self.state.filter()
    }

    pub fn set_filter(&self, filter: &str) {
        self.state.filter().set(filter.to_string());
    }

    pub fn search_query(&self) -> impl Get<Value = String> + Copy + use<> {
        self.state.search_query()
    }

    pub fn set_search_query(&self, q: &str) {
        self.state.search_query().set(q.to_string());
    }
}
