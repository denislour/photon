# Photon Architecture Skill

## Overview

Tumblr-inspired art gallery for KhangHeo. Single Cloudflare Worker serves both frontend (Leptos WASM bundle) and API (Axum routes). No CORS, no separate deployments.

```
khangheo.com
    └── Cloudflare Worker (photon-be)
        ├── [assets] → index.html + WASM → Leptos SPA
        └── Axum API
            ├── /api/media       (POST/GET)
            ├── /api/media/{id}  (GET/DELETE)
            ├── /api/albums      (POST/GET)
            ├── /api/albums/{id} (GET)
            └── /api/search?q=   (GET)
```

## Data Flow

```
Browser (Leptos WASM)
  │
  ├─ SPA routing (client-side)
  │    ├─ /       → GalleryPage
  │    └─ /albums → AlbumsPage
  │
  ├─ API calls (reqwest / fetch)
  │    ├─ GET /api/media      → JSON list
  │    ├─ POST /api/media     → multipart upload
  │    ├─ GET /api/media/{id} → binary (image/video)
  │    ├─ POST /api/albums    → create album
  │    └─ GET /api/albums     → list albums
  │
  └─ Worker handles all:
       ├─ matches /api/* → Axum router
       └─ else → serve static assets
```

## D1 Schema

```sql
media (id, original_name, mime_type, file_size, width, height, duration, bucket_path, album_id, created_at)
albums (id, name, description, cover_path, created_at)
tags (id, name)
media_tags (media_id, tag_id) → FK media + tags
```

## R2 Structure

```
MEDIA_BUCKET/
├── {uuid}.jpg     ← raw files stored by UUID
├── {uuid}.mp4
└── ...
```

## Tech Stack Matrix

| Layer    | Tech              | Version                    |
| -------- | ----------------- | -------------------------- |
| Frontend | Leptos CSR        | 0.8                        |
| CSS      | Tailwind CSS      | 3.x                        |
| Bundler  | Trunk             | 0.21                       |
| Backend  | workers-rs + axum | 0.8                        |
| Database | Cloudflare D1     | serverless SQLite          |
| Storage  | Cloudflare R2     | S3-compatible object store |
| Deploy   | Wrangler          | 4.x                        |

## Free Tier Limits (important)

- Worker: 100K req/day, 10ms CPU/req
- D1: 5GB storage, 5M rows read/day
- R2: 10GB storage, 1M Class A ops, 10M Class B ops

## References

- Seekr (reference project): `/home/jake/Rust/seekr/`
  - `api/` — workers-rs + axum + D1 patterns
  - `app/` — Leptos CSR + stores + Tailwind v3
  - `lektor/` — Leptos components, pages, utils
