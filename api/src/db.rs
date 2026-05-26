use worker::D1Database;

pub async fn init(db: &D1Database) -> Result<(), worker::Error> {
    db.prepare(
        "CREATE TABLE IF NOT EXISTS media (
            id            TEXT PRIMARY KEY,
            original_name TEXT NOT NULL,
            mime_type     TEXT NOT NULL,
            file_size     INTEGER NOT NULL,
            width         INTEGER,
            height        INTEGER,
            duration      REAL,
            bucket_path   TEXT NOT NULL,
            album_id      TEXT,
            created_at    TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .run()
    .await?;

    db.prepare(
        "CREATE INDEX IF NOT EXISTS idx_media_created ON media(created_at DESC)",
    )
    .run()
    .await?;

    db.prepare(
        "CREATE INDEX IF NOT EXISTS idx_media_album ON media(album_id)",
    )
    .run()
    .await?;

    db.prepare(
        "CREATE TABLE IF NOT EXISTS albums (
            id          TEXT PRIMARY KEY,
            name        TEXT NOT NULL,
            description TEXT,
            cover_path  TEXT,
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .run()
    .await?;

    db.prepare(
        "CREATE TABLE IF NOT EXISTS tags (
            id   TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
    )
    .run()
    .await?;

    db.prepare(
        "CREATE TABLE IF NOT EXISTS media_tags (
            media_id TEXT NOT NULL,
            tag_id   TEXT NOT NULL,
            PRIMARY KEY (media_id, tag_id),
            FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
    )
    .run()
    .await?;

    Ok(())
}
