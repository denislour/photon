# Rust + Axum Backend Skill (Generic Template)

## Stack

- **Runtime**: Cloudflare Workers via `worker = "0.8"` crate
- **Framework**: Axum (routing, JSON, multipart, CORS)
- **Database**: D1 / SQLite (via `worker::D1Database`)
- **Storage**: R2 / S3-compatible (via `worker::Bucket`)
- **Deploy**: Wrangler + `worker-build` CLI
- **Build target**: `wasm32-unknown-unknown`

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

### Strict Rules — MUST follow, NO exceptions

#### 1. No comments in code

- Write self-documenting code: clear function/variable names, extract helpers, small functions.
- Zero `//` or `///` comments allowed in source files.

#### 2. Use Rust captured identifiers in `format!()`

- `format!("{name} is {age}")` — NEVER `format!("{} is {}", name, age)`.
- If the value is an expression, bind to a variable first: `let n = expr; format!("{n}")`.

#### 3. Route return types by HTTP method

- GET: `Result<Json<Value>, AppError>`
- POST: `Result<(StatusCode, Json<Value>), AppError>` (201 Created)
- PUT/PATCH: `Result<Json<Value>, AppError>`
- DELETE: `Result<StatusCode, AppError>` (204 No Content)
- Binary: `Result<Response<Body>, AppError>`

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
- IDs are UUIDv4 strings via `uuid` crate with `js` feature

#### Optional / NULL fields in INSERT

```rust
let mut values: Vec<JsValue> = vec![
    media.id.clone().into(),
    media.original_name.clone().into(),
    (media.file_size as f64).into(),
];
values.push(match &media.album_id {
    Some(aid) => aid.clone().into(),
    None => wasm_bindgen::JsValue::null(),
});

db.prepare("INSERT INTO t (id, name, size, album_id) VALUES (?1, ?2, ?3, ?4)")
    .bind(&values)?
    .run().await?;
```

#### Search with LIKE

```rust
let pattern = format!("%{query}%");
let result = db
    .prepare("SELECT * FROM media WHERE original_name LIKE ?1 ORDER BY created_at DESC LIMIT ?2")
    .bind(&[pattern.into(), (limit as f64).into()])?
    .run().await?;
Ok(result.results::<Media>()?)
```

### Object Storage (R2 / S3)

Wrap with helper functions:

```rust
pub async fn upload_r2(storage: &Bucket, key: &str, data: &[u8]) -> Result<(), AppError> {
    storage.put(key, data.to_vec()).execute().await?;
    Ok(())
}

pub async fn get_r2(storage: &Bucket, key: &str) -> Result<Option<Vec<u8>>, AppError> {
    let obj = storage.get(key).execute().await?;
    match obj {
        Some(o) => {
            let bytes = o.body().ok_or(AppError::NotFound)?.bytes().await?;
            Ok(Some(bytes))
        }
        None => Ok(None),
    }
}

pub async fn delete_r2(storage: &Bucket, key: &str) -> Result<(), AppError> {
    storage.delete(key).await?;
    Ok(())
}
```

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
- `console_log!` for internal errors before returning:
  ```rust
  AppError::Internal(e) => {
      worker::console_log!("[BE ERROR] {}", e);
      (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("internal: {e}")})))
  }
  ```

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
use tower_http::cors::CorsLayer;

let cors = CorsLayer::permissive();

let app = Router::new()
    .route("/api/media", post(media::route::upload))
    .route("/api/media", get(media::route::list))
    .route("/api/media/{id}", get(media::route::serve))
    .route("/api/media/{id}", delete(media::route::delete))
    .layer(cors)
    .with_state(state);
```

### Multipart Upload Handler

```rust
#[worker::send]
pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name = String::from("unknown");
    let mut file_mime = String::from("application/octet-stream");

    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError::Validation(e.to_string()))?
    {
        match field.name() {
            Some("file") => {
                file_name = field.file_name().unwrap_or("unknown").to_string();
                file_mime = field.content_type().unwrap_or_default().to_string();
                file_data = Some(field.bytes().await
                    .map_err(|e| AppError::Validation(e.to_string()))?.to_vec());
            }
            _ => {}
        }
    }
    let data = file_data.ok_or(AppError::MissingFile)?;
    service::validate_mime(&file_mime)?;
    service::validate_size(data.len(), &file_mime)?;

    let key = format!("{}.{}", service::generate_key(&file_mime), service::extension(&file_mime));
    service::upload_r2(&state.storage.0, &key, &data).await?;

    let media = Media {
        id: Uuid::new_v4().to_string(),
        original_name: file_name,
        mime_type: file_mime,
        file_size: data.len() as i64,
        width: None, height: None, duration: None,
        bucket_path: key,
        album_id: None,
        created_at: String::new(),
    };
    service::insert(&state.db.0, &media).await?;

    Ok((StatusCode::CREATED, Json(json!({"id": media.id, "url": format!("/api/media/{id}")}))))
}
```

### Binary Response Handler (serve media)

```rust
#[worker::send]
pub async fn serve(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response<Body>, AppError> {
    let media = service::get(&state.db.0, &id).await?.ok_or(AppError::NotFound)?;
    let data = service::get_r2(&state.storage.0, &media.bucket_path)
        .await?.ok_or(AppError::NotFound)?;

    Response::builder()
        .header("Content-Type", media.mime_type)
        .header("Cache-Control", "public, max-age=31536000, immutable")
        .body(Body::from(data))
        .map_err(|e| AppError::Internal(e.to_string()))
}
```

### Validation Helpers

```rust
pub fn validate_mime(mime: &str) -> Result<(), AppError> {
    match mime {
        "image/jpeg" | "image/png" | "image/webp" |
        "video/mp4" | "video/quicktime" => Ok(()),
        _ => Err(AppError::Validation(format!("unsupported mime: {mime}")))
    }
}

pub fn validate_size(size: usize, mime: &str) -> Result<(), AppError> {
    if mime.starts_with("video") && size > 100_000_000 {
        return Err(AppError::Validation("video exceeds 100mb".into()));
    }
    if !mime.starts_with("video") && size > 50_000_000 {
        return Err(AppError::Validation("image exceeds 50mb".into()));
    }
    Ok(())
}
```

### Key Generation (UUID + Date Path)

```rust
pub fn generate_key(mime: &str) -> String {
    let prefix = if mime.starts_with("video") { "videos" } else { "photos" };
    let uuid = Uuid::new_v4();
    let date_path = Utc::now().format("%Y/%m").to_string();
    format!("{prefix}/{date_path}/{uuid}")
}

pub fn extension(mime: &str) -> &str {
    match mime {
        "image/jpeg" => "jpg", "image/png" => "png",
        "image/webp" => "webp", "video/mp4" => "mp4",
        "video/quicktime" => "mov", _ => "bin",
    }
}
```

### Worker Entry Point (`lib.rs`)

```rust
#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context)
    -> Result<http::Response<axum::body::Body>>
{
    let database = env.d1("DB")?;
    let bucket = env.bucket("MEDIA_BUCKET")?;
    db::init(&database).await?;

    let state = AppState {
        db: Db(Arc::new(database)),
        storage: Storage(Arc::new(bucket)),
    };

    let app = Router::new()
        .route("/api/media", post(media::route::upload))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app.call(req).await?)
}
```

### Worker Entry Point (`lib.rs`)

```rust
#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context)
    -> Result<http::Response<axum::body::Body>>
{
    worker::console_log!("[BE] init start");
    let database = env.d1("DB")?;
    let bucket = env.bucket("MEDIA_BUCKET")?;
    db::init(&database).await?;
    worker::console_log!("[BE] init complete");

    let state = AppState {
        db: Db(Arc::new(database)),
        storage: Storage(Arc::new(bucket)),
    };

    let app = Router::new()
        // routes + CORS
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(app.call(req).await?)
}
```

## wrangler.toml Configuration

```toml
name = "project-name"
main = "build/index.js"
compatibility_date = "2026-05-25"
compatibility_flags = ["nodejs_compat"]

[build]
command = "worker-build --release"

[assets]
directory = "../app/dist/"
not_found_handling = "single-page-application"

[[r2_buckets]]
binding = "MEDIA_BUCKET"
bucket_name = "my-media-bucket"

[[d1_databases]]
binding = "DB"
database_name = "my-db"
database_id = "local-dev"
```

## Database Schema (db.rs)

```rust
pub async fn init(db: &D1Database) -> Result<(), worker::Error> {
    db.prepare(
        "CREATE TABLE IF NOT EXISTS media (
            id TEXT PRIMARY KEY, original_name TEXT NOT NULL,
            mime_type TEXT NOT NULL, file_size INTEGER NOT NULL,
            width INTEGER, height INTEGER, duration REAL,
            bucket_path TEXT NOT NULL, album_id TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).run().await?;

    db.prepare("CREATE INDEX IF NOT EXISTS idx_media_created ON media(created_at DESC)").run().await?;
    db.prepare("CREATE INDEX IF NOT EXISTS idx_media_album ON media(album_id)").run().await?;

    db.prepare(
        "CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY, name TEXT NOT NULL,
            description TEXT, cover_path TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).run().await?;

    // Many-to-many with CASCADE
    db.prepare(
        "CREATE TABLE IF NOT EXISTS media_tags (
            media_id TEXT NOT NULL, tag_id TEXT NOT NULL,
            PRIMARY KEY (media_id, tag_id),
            FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )"
    ).run().await?;

    Ok(())
}
```

## Justfile Commands (Dev Workflow)

```makefile
dev:
    trap 'kill 0' EXIT; \
    (cd api && wrangler dev --port 8000) & \
    until curl -s -o /dev/null http://localhost:8000/api/media 2>/dev/null; \
      do sleep 1; done && \
    cd app && trunk serve --port 3000

dev-api:
    cd api && wrangler dev --port 8000

dev-app:
    cd app && trunk serve --port 3000

build:
    cd api && cargo build --target wasm32-unknown-unknown --release
    cd app && trunk build --release

deploy:
    cd api && wrangler deploy

d1-query query='SELECT name FROM sqlite_master WHERE type="table"':
    cd api && wrangler d1 execute my-db --local --command "{{query}}"
```

## Assign / Remove Album

```rust
pub async fn assign_album(db: &D1Database, media_id: &str, album_id: &str) -> Result<(), AppError> {
    db.prepare("UPDATE media SET album_id = ?1 WHERE id = ?2")
        .bind(&[album_id.into(), media_id.into()])?
        .run().await?;
    Ok(())
}

pub async fn remove_from_album(db: &D1Database, media_id: &str) -> Result<(), AppError> {
    db.prepare("UPDATE media SET album_id = NULL WHERE id = ?1")
        .bind(&[media_id.into()])?
        .run().await?;
    Ok(())
}
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
