use leptos::prelude::*;

use crate::utils::api::MediaItem;

#[derive(Clone, Copy)]
pub struct MediaStore {
    items: RwSignal<Vec<MediaItem>>,
    filter: RwSignal<String>,
    search_query: RwSignal<String>,
}

impl MediaStore {
    pub fn new() -> Self {
        Self {
            items: RwSignal::new(vec![]),
            filter: RwSignal::new("all".into()),
            search_query: RwSignal::new(String::new()),
        }
    }

    pub fn items(&self) -> RwSignal<Vec<MediaItem>> {
        self.items
    }

    pub fn set_items(&self, new_items: Vec<MediaItem>) {
        self.items.set(new_items);
    }

    pub fn filter(&self) -> RwSignal<String> {
        self.filter
    }

    pub fn set_filter(&self, f: &str) {
        self.filter.set(f.to_string());
    }

    pub fn search_query(&self) -> RwSignal<String> {
        self.search_query
    }

    pub fn set_search_query(&self, q: &str) {
        self.search_query.set(q.to_string());
    }

    pub fn filtered_items(&self) -> Memo<Vec<MediaItem>> {
        let items = self.items;
        let filter = self.filter;
        let query = self.search_query;
        Memo::new(move |_| {
            let all = items.get();
            let f = filter.get();
            let q = query.get();

            all.into_iter()
                .filter(|item| {
                    let passes_filter = match f.as_str() {
                        "photo" => item.mime_type.starts_with("image"),
                        "video" => item.mime_type.starts_with("video"),
                        _ => true,
                    };
                    let passes_search = if q.trim().is_empty() {
                        true
                    } else {
                        item.original_name.to_lowercase().contains(&q.to_lowercase())
                    };
                    passes_filter && passes_search
                })
                .collect()
        })
    }
}
