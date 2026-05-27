/// localStorage helpers for favorite media

const STORAGE_KEY: &str = "photon_favs";

pub fn is_faved(id: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(STORAGE_KEY).ok())
        .flatten()
        .map(|v| v.contains(id))
        .unwrap_or(false)
}

pub fn toggle_fav(id: &str) {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
    if let Some(s) = storage {
        let current = s.get_item(STORAGE_KEY).ok().flatten().unwrap_or_default();
        let next = if current.contains(id) {
            current
                .replace(id, "")
                .replace(",,", ",")
                .trim_matches(',')
                .to_string()
        } else {
            let mut r = current;
            if !r.is_empty() {
                r.push(',');
            }
            r.push_str(id);
            r
        };
        let _ = s.set_item(STORAGE_KEY, &next);
    }
}
