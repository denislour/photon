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
│   ├── modal.rs         # Fullscreen / overlay modal component
│   ├── toast.rs         # Auto-dismiss toast notification
│   └── form.rs          # Upload / input zone component
├── pages/
│   ├── mod.rs
│   ├── home.rs          # Main list / gallery page
│   └── secondary.rs     # Secondary page (albums, settings, etc.)
├── stores/
│   ├── mod.rs           # AppCtx struct combining all stores
│   ├── app_store.rs     # Global app state (view mode, preferences)
│   └── resource_store.rs # Entity-specific state (items, filters, search)
└── utils/
    ├── mod.rs
    ├── api.rs           # API client functions
    ├── storage.rs       # localStorage / sessionStorage helpers
    └── icons.rs         # SVG icon constants
```

## Key Patterns

### Code Style

- **No comments in code**: Use clear function/variable names, extract helpers, and keep components small. Code should be self-documenting.
- Extract long Tailwind class strings (>80 chars) into `const` variables at top of file.
- Extract complex event handlers as named closures before the `view!` block.
- Extract repeated HTML patterns into sub-components.
- Use `.collect_view()` instead of `.collect::<Vec<_>>()`.
- **Use Rust captured identifiers** in `format!()` instead of positional arguments: `format!("{name} is {age}")` instead of `format!("{} is {}", name, age)`.

### Component Structure

```rust
#[allow(non_snake_case)]
#[component]
pub fn MyComponent() -> impl IntoView {
    // 1. Extract context
    let ctx = expect_context::<AppCtx>();
    // 2. Local signals / state
    let local_state = RwSignal::new(initial);
    // 3. Effects / async spawns
    // 4. Event handlers (on:click, on:input, etc.)
    // 5. view! macro
}
```

### State Management

- `AppCtx` is provided at router level, bundles all store structs
- Each store is a `Copy` struct holding `RwSignal<T>` fields
- Derived state uses `Memo` (e.g. filtered list from items + filter + search)
- Toast: `RwSignal<String>` provided as context at app root

```rust
#[derive(Clone, Copy)]
pub struct AppCtx {
    pub app: AppStore,
    pub resource: ResourceStore,
}
```

### Toast Pattern

```rust
if let Some(t) = toast {
    t.set("message".into());
    let t2 = t;
    spawn_local(async move {
        gloo_timers::future::TimeoutFuture::new(3000).await;
        t2.set(String::new());
    });
}
```

Use `gloo_timers::future::TimeoutFuture` for toast auto-dismiss, **not** `set_timeout` from Leptos.

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
- Dynamic messages: `format!("{} {}", tr(I18nKey::SomeKey), dynamic_part)`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I18nKey {
    AppTitle,
    // ... all keys
}

pub fn tr(key: I18nKey) -> &'static str {
    match key {
        I18nKey::AppTitle => "...",
    }
}
```

### Tailwind CSS

- Define custom theme colors in `tailwind.config.js` (e.g. `navy`, `ink`, `gold`, `surf`, `hl`, `body`, `mute`, `danger`)
- Group colors by role (background, text, accent, border, muted)
- Use Tailwind v3 syntax: `@tailwind base/components/utilities`, no `@import "tailwindcss"`, no `@theme`
- Class variants: `class:border-gold=is_active` for conditional styling

### Icons

- All SVGs centralized in `icons.rs` as `&'static str`
- Rendered via `<span inner_html=icons::NAME />`

### Router

```rust
<Routes fallback=|| view! { "404" }>
    <Route path=path!("") view=HomePage />
    <Route path=path!("secondary") view=SecondaryPage />
</Routes>
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
2. Export in `pages/mod.rs`
3. Add route in `app.rs`
4. Add navigation link in `header.rs`
5. Add i18n keys in `i18n.rs` if needed

## Adding a New API Call

1. Add request/response structs in `utils/api.rs`
2. Add async function returning `Result<T, String>`
3. Call with `spawn_local` in component
4. Handle loading/error states

## Adding a New Utility

1. Create `utils/my_util.rs` with pub functions
2. Export in `utils/mod.rs`

## Router + Provider Setup (app.rs)

```rust
#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    let ctx = AppCtx {
        app: AppStore::new(),
        resource: ResourceStore::new(),
    };
    provide_context(ctx);
    let toast: RwSignal<String> = RwSignal::new(String::new());
    provide_context(toast);

    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=|| view! { "404" }>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("secondary") view=SecondaryPage />
                </Routes>
            </main>
            <Toast message=toast />
        </Router>
    }
}
```
