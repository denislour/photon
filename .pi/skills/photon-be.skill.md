# Rust + Axum Backend Skill (Generic Template)

## Stack

- **Runtime**: Cloudflare Workers via `workers-rs` (or other WASM runtime)
- **Framework**: Axum (routing, JSON, multipart)
- **Database**: D1 / SQLite (or SQL via prepared statements)
- **Storage**: R2 / S3-compatible object storage
- **Deploy**: Wrangler / wrangler.toml

## Project Layout (Convention)

```
api/src/
├── lib.rs          # Router, AppState, #[event(fetch)] entry point
├── db.rs           # Schema init (CREATE TABLE IF NOT EXISTS)
├── error.rs        # AppError enum → IntoResponse
├── <module_a>/
│   ├── mod.rs      # pub mod route; pub mod service; pub mod model;
│   ├── route.rs    # Handlers (business logic calls service)
│   ├── service.rs  # D1/SQL queries + R2/storage operations
│   └── model.rs    # Request/response structs (Serialize, Deserialize)
├── <module_b>/
│   ├── mod.rs
│   ├── route.rs
│   ├── service.rs
│   └── model.rs
└── <module_c>/
    ├── mod.rs      # (model.rs optional if reusing from another module)
    ├── route.rs
    └── service.rs
```

## Key Patterns

### Code Style

- **No comments in code**: Use clear function/variable names, extract helpers, and keep functions small. Code should be self-documenting.
- Extract long class strings (>80 chars) into `const` variables.
- **Use Rust captured identifiers** in `format!()` instead of positional arguments: `format!("{name} is {age}")` instead of `format!("{} is {}", name, age)`.

### Route Handlers

- All handlers use `#[worker::send]` attribute (workers-rs)
- State extracted via `State(state): State<AppState>`
- Return types follow HTTP method convention:
  - **GET** (list, detail): `Result<Json<Value>, AppError>` — implicit 200 OK
  - **POST** (create, upload): `Result<(StatusCode, Json<Value>), AppError>` — explicit 201 Created
  - **PUT / PATCH** (update): `Result<Json<Value>, AppError>` — implicit 200 OK
  - **DELETE** (delete, remove): `Result<StatusCode, AppError>` — 204 No Content
  - **Binary GET** (file serve): `Result<Response<Body>, AppError>`
- Validation (empty check, format check) happens in route handler
- Business logic (DB queries, storage I/O) happens in service

### AppState

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: Db,           // Arc<D1Database> or similar
    pub storage: Storage, // Arc<Bucket> or similar
}
```

Wrap external resources in newtypes for `Send + Sync` safety:

```rust
#[derive(Clone)]
pub struct Db(pub Arc<D1Database>);
unsafe impl Send for Db {}
unsafe impl Sync for Db {}
```

### D1 / SQL Queries

- Use `db.prepare("SQL").bind(&[...])?.run().await?`
- Bind values use `wasm_bindgen::JsValue`:
  - Strings: `"text".into()`
  - Numbers: `(42 as f64).into()`
  - NULL: `wasm_bindgen::JsValue::null()`
- Results deserialized via `result.results::<T>()?`
- IDs are UUIDv4 strings

### Object Storage (R2 / S3)

- Upload: `bucket.put(path, data.to_vec()).execute().await?`
- Download: `bucket.get(path).execute().await?` → `obj.body().bytes().await?`
- Delete: `storage.delete(key).await?`

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("validation: {0}")]
    Validation(String),
    #[error("not found")]
    NotFound,
    #[error("missing file")]
    MissingFile,
    #[error("internal: {0}")]
    Internal(String),
}
```

- Implements `IntoResponse` → proper HTTP status codes + `{"error": "..."}` JSON body
- Converts from `worker::Error` / runtime errors via `AppError::Internal`

### Module Structure Pattern

```
my_module/
├── mod.rs      →  pub mod route; pub mod service; pub mod model;
├── route.rs    →  #[worker::send] handler functions
├── service.rs  →  async fn create(), list(), get(), delete(), etc.
└── model.rs    →  Request/Response structs with Serialize/Deserialize
```

### Route Registration (lib.rs)

```rust
Router::new()
    .route("/api/<resource>", get(module::route::list))
    .route("/api/<resource>", post(module::route::create))
    .route("/api/<resource>/{id}", get(module::route::get))
    .route("/api/<resource>/{id}", delete(module::route::delete))
    .with_state(state);
```

## Adding a New Route

1. Add service function in `service.rs` (DB query + business logic)
2. Add handler in `route.rs` with `#[worker::send]` (validation + call service)
3. Register route in `lib.rs`
4. Handle errors with `AppError` variants

## Adding a New Module

1. Create `module_name/` dir with `mod.rs`, `route.rs`, `service.rs`, `model.rs`
2. `mod.rs` re-exports: `pub mod route; pub mod service; pub mod model;`
3. Register module in `lib.rs`

## Dev Commands (Template — adapt per project)

```bash
# Full stack dev (API + Frontend)
just dev

# API only
just dev-api

# Build WASM (for workers-rs)
cargo build --target wasm32-unknown-unknown

# DB query (D1)
wrangler d1 execute <db-name> --local --command "SELECT ..."
```
