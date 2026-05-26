# Photon — Build Plan

> Tumblr-inspired art gallery for KhangHeo. Full-stack Rust: Leptos CSR FE + Axum/WASM BE on Cloudflare Workers.

## Tech Stack

| Layer | Technology | Version | Notes |
|-------|-----------|---------|-------|
| Frontend | Leptos (CSR) | 0.8 | reactive_stores, leptos_router |
| Bundler | Trunk | 0.21 | Tailwind v3 plugin |
| CSS | Tailwind CSS | **3.x** | Trunk doesn't support v4 |
| Backend | workers-rs + axum | 0.8 | 100% Rust → WASM Worker |
| Database | Cloudflare D1 | — | SQLite serverless |
| Storage | Cloudflare R2 | — | Object storage (photos/videos) |
| Hosting | Workers + Static Assets | — | Single Worker serves both FE + BE |
| Deploy | Wrangler CLI | 4.x | `npx wrangler deploy` |

## Architecture

**One Worker to rule them all.** A single Cloudflare Worker serves:
1. Static assets (Leptos WASM bundle via `[assets]`)
2. API routes (Axum router in Rust)

No CORS, no separate deployments, shared env bindings.

```
khangheo.com
    └── Cloudflare Worker (photon-be)
        ├── [assets] → index.html + WASM → Leptos SPA
        └── Axum API
            ├── POST   /api/media      — upload
            ├── GET    /api/media      — list
            ├── GET    /api/media/{id} — serve
            ├── DELETE /api/media/{id} — delete
            ├── POST   /api/albums     — create
            ├── GET    /api/albums     — list
            └── GET    /api/search     — search
```

## Project Structure

```
photon/
├── api/                     # Backend — workers-rs + axum
│   ├── Cargo.toml
│   ├── wrangler.toml        # CF config with R2 + D1 bindings
│   └── src/
│       ├── lib.rs           # Router, AppState, #[event(fetch)]
│       ├── db.rs            # D1 schema init
│       ├── error.rs         # AppError enum
│       ├── media/           # Media CRUD
│       │   ├── mod.rs
│       │   ├── route.rs
│       │   ├── service.rs
│       │   └── model.rs
│       ├── albums/          # Album CRUD
│       └── search/          # Search
├── app/                     # Frontend — Leptos CSR
│   ├── Cargo.toml
│   ├── trunk.toml           # Trunk config (Tailwind v3)
│   ├── tailwind.config.js   # TW v3 config
│   ├── index.html           # SPA entry
│   └── src/
│       ├── lib.rs           # wasm_bindgen start
│       ├── app.rs           # Router + providers
│       ├── components/      # Reusable components
│       │   ├── header.rs
│       │   ├── photo_card.rs
│       │   ├── upload_zone.rs
│       │   └── toast.rs
│       ├── pages/           # Route pages
│       │   ├── gallery.rs
│       │   └── albums.rs
│       ├── stores/          # reactive_stores
│       │   ├── app_store.rs
│       │   └── media_store.rs
│       └── utils/           # API client, helpers
├── .docs/                   # Design docs (gitignored)
│   ├── design.html          # HTML prototype
│   ├── spec.html            # System spec
│   └── flow.html            # Upload flow
├── .pi/skills/              # Agent skills
│   ├── photon-be.skill.md
│   ├── photon-fe.skill.md
│   ├── photon-git.skill.md
│   └── photon-arch.skill.md
├── Justfile                 # Dev/ build commands
├── PLAN.md                  # This file
├── .gitignore
└── Cargo.toml               # Workspace root
```

## Code Principles

1. **No comments** — write self-documenting code with expressive names
2. **Tailwind v3 only** — Trunk doesn't support v4, no `@import "tailwindcss"`
3. **Rust edition 2024** — `cargo fmt` + `clippy -D warnings` enforced
4. **BE-first** — build API before FE for each feature
5. **Semantic Git** — conventional commits, one feature per commit, one-line messages
6. **Async for I/O** — R2, D1, HTTP calls are async; pure computation is sync

## Integration Roadmap

### Phase 1: BE Foundation
- [ ] Scaffold workers-rs + axum project
- [ ] D1 schema: `media`, `albums`, `tags`, `media_tags`
- [ ] R2 bucket binding + storage helper
- [ ] `AppError` enum with `IntoResponse`
- [ ] `POST /api/media` — multipart upload, validate MIME+size, save to R2 + D1
- [ ] `GET /api/media` — paginated list from D1
- [ ] `GET /api/media/{id}` — stream file from R2
- [ ] `DELETE /api/media/{id}` — remove from R2 + D1
- [ ] `POST /api/albums` + `GET /api/albums`
- [ ] `GET /api/search?q=` — search by name

### Phase 2: FE Foundation
- [ ] Scaffold Leptos CSR + Trunk + Tailwind v3
- [ ] App shell: Router, Header (nav, search, upload btn)
- [ ] reactive_stores: `AppStore` (theme, view mode), `MediaStore` (items, filter)
- [ ] API client module with `reqwest`
- [ ] Tailwind v3 theme colors matching `.docs/design.html`

### Phase 3: Upload Flow
- [ ] BE: multipart upload handler (verify, persist)
- [ ] FE: Upload button + drag-drop zone (`<input type="file">`)
- [ ] FE: Progress bar animation
- [ ] FE: Toast notification on success/error

### Phase 4: Gallery View
- [ ] FE: Masonry grid with CSS columns (`columns-4 max-[1080px]:columns-3 ...`)
- [ ] FE: List view alternative
- [ ] FE: Filter buttons (all, photo, video, fav, today, week, month)
- [ ] FE: Search with live dropdown results
- [ ] FE: Stagger entrance animations
- [ ] BE: Pagination (limit/offset)

### Phase 5: Album Management
- [ ] BE: Album create/list routes
- [ ] FE: Album list page with grid cards
- [ ] FE: Create album modal
- [ ] FE: Assign media to album

### Phase 6: Photo Viewer
- [ ] FE: Modal overlay with left/right navigation
- [ ] FE: Thumbnail strip at bottom
- [ ] FE: Favorite toggle + filter by fav
- [ ] FE: Delete with confirmation
- [ ] FE: Keyboard shortcuts (Escape, ArrowLeft, ArrowRight)

### Phase 7: Polish & Deploy
- [ ] FE: Page transitions (fadeIn), modal scale animation
- [ ] FE: Responsive across mobile/tablet/desktop
- [ ] FE: Accessibility (focus-visible rings, aria-labels)
- [ ] BE: Input validation + rate limiting
- [ ] Deploy: `npx wrangler deploy` → khangheo.com

## Skills

| Skill | File | Purpose |
|-------|------|---------|
| BE | `.pi/skills/photon-be.skill.md` | Rust + Axum + workers-rs + R2/D1 |
| FE | `.pi/skills/photon-fe.skill.md` | Leptos CSR + Tailwind v3 + Trunk |
| Git | `.pi/skills/photon-git.skill.md` | Semantic commits, workflow |
| Arch | `.pi/skills/photon-arch.skill.md` | Full-stack architecture, roadmap |

## Git Convention

```
<type>(<scope>): <description>

Types: feat, fix, chore, docs, refactor, style
Scopes: be, fe, root

Examples:
  feat(be): add POST /api/media upload with R2 storage
  feat(fe): add gallery grid with masonry layout
  chore(root): add Justfile with dev, build, deploy recipes
```

## References

- `.docs/spec.html` — System spec with schema, stack, free tier limits
- `.docs/flow.html` — Upload flow, REST API endpoints, multipart payload
- `.docs/design.html` — HTML prototype with full Tailwind classes
- `/home/jake/Rust/seekr/api/` — Reference BE: workers-rs + axum + D1
- `/home/jake/Rust/seekr/app/` — Reference FE: Leptos CSR + stores + Tailwind v3
- `/home/jake/Rust/seekr/lektor/` — Reference FE: Leptos components, pages, utils
