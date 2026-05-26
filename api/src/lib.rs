use std::sync::Arc;

use axum::{Router, routing::{get, post, delete}};
use tower_service::Service;
use worker::*;

mod albums;
mod db;
mod error;
mod media;
mod search;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub storage: Storage,
}

#[derive(Clone)]
pub struct Db(pub Arc<worker::D1Database>);

unsafe impl Send for Db {}
unsafe impl Sync for Db {}

#[derive(Clone)]
pub struct Storage(pub Arc<worker::Bucket>);

unsafe impl Send for Storage {}
unsafe impl Sync for Storage {}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<http::Response<axum::body::Body>> {
    let database = env.d1("DB")?;
    let bucket = env.bucket("MEDIA_BUCKET")?;
    db::init(&database).await?;

    let state = AppState {
        db: Db(Arc::new(database)),
        storage: Storage(Arc::new(bucket)),
    };

    let mut app = Router::new()
        .route("/api/media", post(media::route::upload))
        .route("/api/media", get(media::route::list))
        .route("/api/media/{id}", get(media::route::serve))
        .route("/api/media/{id}", delete(media::route::delete))
        .route("/api/albums", post(albums::route::create))
        .route("/api/albums", get(albums::route::list))
        .route("/api/albums/{id}", get(albums::route::get))
        .route("/api/search", get(search::route::search))
        .with_state(state);

    Ok(app.call(req).await?)
}
