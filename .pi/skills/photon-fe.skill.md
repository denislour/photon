# Leptos + Tailwind Frontend Skill (Generic Template)

## Stack

- **Framework**: Leptos (CSR / SSR mode)
- **Router**: leptos_router
- **Styling**: Tailwind CSS via bundler plugin
- **Bundler**: Trunk / Leptos CLI
- **State**: RwSignal + Memo pattern (no reactive_stores crate)

## Project Layout (Convention)

```
app/src/
├── lib.rs              # wasm_bindgen start, mount_to_body
├── app.rs              # Router, context providers, App wrapper
├── app.css             # Tailwind directives (@tailwind base/components/utilities)
├── i18n.rs             # I18nKey enum + tr() function for i18n
├── components/
│   ├── mod.rs
│   ├── header.rs        # Navigation bar / app header
│   ├── modal.rs         # Fullscreen / overlay modal (lightbox/photo)
│   ├── toast.rs         # ToastStore-based auto-dismiss notification
│   └── form.rs          # Upload / drag-drop zone component
├── pages/
│   ├── mod.rs
│   ├── home.rs          # Main list / gallery page
│   └── detail.rs        # Secondary page (detail, settings, albums)
├── stores/
│   ├── mod.rs           # AppCtx struct combining all stores
│   ├── app_store.rs     # Global app state (view mode, preferences)
│   ├── media_store.rs   # Entity state (items, filter, search, selected)
│   ├── album_store.rs   # Album list state
│   ├── upload_store.rs  # Upload progress / queue state
│   └── toast_store.rs   # Toast message state + .show() method
└── utils/
    ├── mod.rs
    ├── api.rs           # API client functions
    ├── storage.rs       # localStorage / sessionStorage helpers
    └── icons.rs         # SVG icon constants
```

## Key Patterns

### Strict Rules — MUST follow, NO exceptions

#### 1. No comments in code

- Write self-documenting code: clear function/variable names, extract helpers, small components.
- Zero `//` or `///` comments allowed in source files.

#### 2. All class strings as constants

- Every `class="..."` must be a `const` variable at top of file — **regardless of length**.
- Name in `UPPER_SNAKE_CASE`. Use `class=CONST_NAME` syntax.
- Even single-word classes like `class="relative"` must be extracted.

#### 3. All state must be in stores

- Every `RwSignal` must live inside a store struct.
- Zero local `RwSignal::new(...)` in components or pages.
- Store pattern: **State** (fields) → **Getter** (methods returning signal) → **Setter** (mutation methods).
- Load/store data via `expect_context::<StoreType>()`.

#### 4. Toast via ToastStore

- Use `toast.show(msg, ms)` — NEVER inline `spawn_local` + `TimeoutFuture`.
- ToastStore is provided globally, access via `expect_context::<ToastStore>()`.

#### 5. All user-facing strings via i18n

- Use `tr(I18nKey::*)` — NO hardcoded strings anywhere.
- Add new keys to `I18nKey` enum + `tr()` match.

#### 6. Use Rust captured identifiers in `format!()`

- `format!("{name} is {age}")` — NEVER `format!("{} is {}", name, age)`.
- If the value is an expression, bind to a variable first.

#### 7. Use `.collect_view()`

- Use `.collect_view()` — NEVER `collect::<Vec<_>>()`.

#### 8. Extract event handlers before `view!`

- Define complex closures as named variables BEFORE the `view!` block.
- Pass as `on:click=handler`, NOT inline in the HTML.

#### 9. Extract repeated HTML into sub-components

- Any pattern used ≥2 times must be extracted into a `#[component]` function.
- Also extract helpers returning `impl IntoView` (e.g. conditional thumbnails).

### Component Structure

```rust
#[allow(non_snake_case)]
#[component]
pub fn MyComponent() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();
    let toast = expect_context::<ToastStore>();
    let local_state = RwSignal::new(initial);
    let handle_click = move |_| {
        toast.show("hello", 2000);
    };
    view! {
        <div class=CONTAINER on:click=handle_click>
            {children}
        </div>
    }
}
```

### State Management

- `AppCtx` is provided at router level, bundles all store structs
- Each store is a `Copy` struct holding `RwSignal<T>` fields
- Derived state uses `Memo` (e.g. filtered list from items + filter + search)
- Every store gets its own file. Named re-export in `mod.rs`: `mod foo; pub use foo::Foo;`

```rust
#[derive(Clone, Copy)]
pub struct AppCtx {
    pub app: AppStore,
    pub media: MediaStore,
    pub album: AlbumStore,
    pub upload: UploadStore,
    pub toast: ToastStore,
}
```

### ToastStore Pattern

```rust
#[derive(Clone, Copy)]
pub struct ToastStore {
    message: RwSignal<String>,
    visible: RwSignal<bool>,
}

impl ToastStore {
    pub fn new() -> Self { /* ... */ }

    pub fn show(&self, msg: &str, duration_ms: u32) {
        self.message.set(msg.to_string());
        self.visible.set(true);
        let msg = self.message;
        let vis = self.visible;
        spawn_local(async move {
            TimeoutFuture::new(duration_ms).await;
            msg.set(String::new());
            vis.set(false);
        });
    }

    pub fn message(&self) -> ReadSignal<String> { self.message.read_only() }
    pub fn visible(&self) -> ReadSignal<bool> { self.visible.read_only() }
}

// In component:
let toast = expect_context::<ToastStore>();
toast.show("uploaded!", 3000);
```

### API Calls

- **JSON APIs** (list, create, update): `reqwest::Client` with serde JSON
- **Multipart upload**: `web_sys::FormData` + `web_sys::Request` + `JsFuture`
- `spawn_local` for async in components
- Helper `api_url()` handles dev (localhost proxy port) vs prod (same origin) resolution
- All API functions return `Result<T, String>` with consistent error handling:
  ```rust
  if !resp.status().is_success() {
      let text = resp.text().await.map_err(|e| e.to_string())?;
      return Err(text);
  }
  ```

### i18n (Internationalization)

- All user-facing strings must use `tr(I18nKey::*)` — no hardcoded strings
- Dynamic messages: `format!("{} {name}", tr(I18nKey::SomeKey), name=val)` — with captured identifiers

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I18nKey {
    AppTitle,
    UploadComplete,
    // ... all keys
}

pub fn tr(key: I18nKey) -> &'static str {
    match key {
        I18nKey::AppTitle => "...",
        I18nKey::UploadComplete => "uploaded",
    }
}
```

### Tailwind CSS

- Define custom theme colors in `tailwind.config.js` (e.g. `navy`, `ink`, `gold`, `surf`, `hl`, `body`, `mute`, `danger`)
- Group colors by role (background, text, accent, border, muted)
- Use Tailwind v3 syntax: `@tailwind base/components/utilities`, no `@import "tailwindcss"`, no `@theme`
- Class variants: `class:border-gold=is_active` for conditional styling
- `app.css`: only 3 lines (`@tailwind base; @tailwind components; @tailwind utilities;`)
- Example theme colors:
  ```js
  // tailwind.config.js
  theme: { extend: { colors: {
    navy: "#13161f",   // bg primary
    ink: "#e8e6e0",   // text primary
    mute: "#8a8780",  // text secondary
    body: "#a8a49e",  // text body
    gold: "#f2992e",  // accent
    surf: "#1a1e2a",  // surface card
    hl: "rgb(255 255 255 / 0.06)", // hover highlight
  }}}
  ```

### Icons

- All SVGs centralized in `icons.rs` as `&'static str`
- Rendered via `<span inner_html=icons::NAME />`

### Router

```rust
<Routes fallback=|| view! { "404" }>
    <Route path=path!("") view=GalleryPage />
    <Route path=path!("detail") view=DetailPage />
</Routes>
```

### Modal / Lightbox Pattern

```rust
#[derive(Clone, Copy)]
pub struct MediaStore {
    items: RwSignal<Vec<MediaItem>>,
    selected: RwSignal<Option<usize>>,
    // ...
}

impl MediaStore {
    pub fn open_modal(&self, index: usize) {
        self.selected.set(Some(index));
    }
    pub fn close_modal(&self) {
        self.selected.set(None);
    }
}

// In modal component:
let store = expect_context::<MediaStore>();
let is_open = move || store.selected().get().is_some();
let close = move |_| store.close_modal();
view! {
    <Show when=is_open>
        <div class=OVERLAY on:click=close>
            <img src={move || format!("/api/media/{}", idx)} />
            <button on:click=close class=CLOSE_BTN>
                <span inner_html=icons::CLOSE />
            </button>
        </div>
    </Show>
}
```

### Upload Store Pattern

```rust
#[derive(Clone, Copy)]
pub struct UploadStore {
    uploading: RwSignal<bool>,
    progress: RwSignal<u8>, // 0-100 percent
}

impl UploadStore {
    pub async fn upload(&self, file: &web_sys::File) -> Result<String, String> {
        self.uploading.set(true);
        // Use web_sys::FormData + JsFuture to POST multipart
        let form = web_sys::FormData::new().unwrap();
        form.append_with_blob("file", file).unwrap();
        let url = api_url("/api/media");
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&form.into()));
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
        let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
        // parse response
        self.uploading.set(false);
        Ok(id)
    }
}
```

### Pagination / Infinite Scroll Pattern

```rust
// In page component
let store = expect_context::<MediaStore>();
let page = RwSignal::new(0);
let loading = RwSignal::new(false);
let has_more = RwSignal::new(true);

let load_more = move |_| {
    if loading.get() || !has_more.get() { return; }
    loading.set(true);
    let p = page.get();
    spawn_local(async move {
        match api::list_media(p, 50).await {
            Ok(mut items) => {
                store.items().update(|old| old.append(&mut items));
                page.set(p + 1);
                has_more.set(items.len() == 50);
            }
            Err(e) => toast.show(&e, 3000),
        }
        loading.set(false);
    });
};

// scroll sentinel
view! {
    <div on:click=load_more>
        {move || loading.get().then(|| view! { <p>"loading..."</p> })}
    </div>
}
```

### Album Store Pattern

```rust
#[derive(Clone, Copy)]
pub struct AlbumStore {
    items: RwSignal<Vec<Album>>,
    selected: RwSignal<Option<String>>, // album_id
}

impl AlbumStore {
    pub fn set_items(&self, items: Vec<Album>) { self.items.set(items); }
    pub fn items(&self) -> RwSignal<Vec<Album>> { self.items }
    pub fn selected_id(&self) -> RwSignal<Option<String>> { self.selected }

    pub fn load(&self) {
        spawn_local(async move {
            if let Ok(albums) = api::list_albums().await {
                self.items.set(albums);
            }
        });
    }
}
```

### Store Pattern

```rust
#[derive(Clone, Copy)]
pub struct ResourceStore {
    items: RwSignal<Vec<Item>>,
    filter: RwSignal<String>,
    search_query: RwSignal<String>,
}

impl ResourceStore {
    pub fn new() -> Self { /* ... */ }

    pub fn filtered_items(&self) -> Memo<Vec<Item>> {
        let items = self.items;
        let filter = self.filter;
        let query = self.search_query;
        Memo::new(move |_| {
            all.into_iter()
                .filter(|item| passes_filter && passes_search)
                .collect()
        })
    }
}
```

## Adding a New Page

1. Create `pages/my_page.rs` with `#[component] pub fn MyPage()`
2. Named export in `pages/mod.rs`: `mod my_page; pub use my_page::MyPage;`
3. Add route in `app.rs`
4. Add navigation link in `header.rs`
5. Add i18n keys in `i18n.rs` if needed

## Adding a New Store

1. Create `stores/new_store.rs`:
   - Define `#[derive(Clone, Copy)] pub struct NewStore { field: RwSignal<T> }`
   - `pub fn new()` + getters + setters
2. Named export in `stores/mod.rs`: `mod new_store; pub use new_store::NewStore;`
3. Add field to `AppCtx` + init in `app.rs`

## Adding a New API Call

1. Add request/response structs in `utils/api.rs`
2. Add async function returning `Result<T, String>`
3. Call with `spawn_local` in component
4. Handle loading/error states via toast

## Adding a New Utility

1. Create `utils/my_util.rs` with pub functions
2. Named export in `utils/mod.rs`

## Router + Provider Setup (app.rs)

```rust
#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    let app = AppStore::new();
    let media = MediaStore::new();
    let album = AlbumStore::new();
    let upload = UploadStore::new();
    let toast = ToastStore::new();
    let ctx = AppCtx { app, media, album, upload, toast };

    // Provide individually so components can grab only what they need
    provide_context(ctx);
    provide_context(app);
    provide_context(media);
    provide_context(album);
    provide_context(upload);
    provide_context(toast);

    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=|| view! { "404" }>
                    <Route path=path!("") view=GalleryPage />
                    <Route path=path!("albums") view=AlbumsPage />
                </Routes>
            </main>
            <Toast />
        </Router>
    }
}
```
