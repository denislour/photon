use leptos::{mount::mount_to_body, prelude::view};
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod stores;
mod utils;

use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| view! { <App /> });
}
