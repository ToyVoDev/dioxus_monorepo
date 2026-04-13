# Jellyfin API Migration — Plan 1: Foundation (Schema + Auth)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the existing Dioxus server-function API with a clean Axum REST server — new 10-table schema, AppState, argon2 auth, and `/Users` + `/Auth` routes wired into the web crate.

**Architecture:** `dioxus_music_api` becomes a pure Axum library (no Dioxus dependency). All server functions are deleted. A new clean Diesel schema is created from scratch. Auth uses a custom `AuthUser` extractor that parses Jellyfin's `Authorization: MediaBrowser Token=...` header. View components are stubbed to keep the web crate compiling during migration.

**Tech Stack:** Axum 0.8, Diesel 2 + diesel-async (bb8), PostgreSQL, argon2 0.5, sha2, hex, rand 0.8

**Spec:** `docs/superpowers/specs/2026-04-13-jellyfin-api-migration-design.md`

---

## File Map

**Delete (old server-function era):**
- `packages/api/src/schema.rs`
- `packages/api/src/models.rs`
- `packages/api/src/streaming.rs`
- `packages/api/src/lib.rs` (replaced entirely)
- `packages/api/migrations/00000000000001_create_tracks/`
- `packages/api/migrations/00000000000002_create_playlists/`
- `packages/api/migrations/00000000000003_add_updated_at/`

**Create (new):**
- `packages/api/migrations/2026-04-13-000001_initial_schema/up.sql`
- `packages/api/migrations/2026-04-13-000001_initial_schema/down.sql`
- `packages/api/diesel.toml`
- `packages/api/src/lib.rs`
- `packages/api/src/state.rs`
- `packages/api/src/error.rs`
- `packages/api/src/types.rs` (shared response DTOs — no feature gate, usable by WASM)
- `packages/api/src/db/mod.rs`
- `packages/api/src/db/schema.rs`
- `packages/api/src/db/models.rs`
- `packages/api/src/auth/mod.rs`
- `packages/api/src/auth/password.rs`
- `packages/api/src/auth/token.rs`
- `packages/api/src/auth/middleware.rs`
- `packages/api/src/routes/mod.rs`
- `packages/api/src/routes/users.rs`

**Modify:**
- `packages/api/Cargo.toml`
- `Cargo.toml` (workspace — add argon2, sha2, hex)
- `packages/web/src/main.rs`
- `packages/web/src/views/*.rs` (stub view bodies)

---

## Task 1: Add workspace dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add new crates to workspace dependencies**

In `Cargo.toml`, add to `[workspace.dependencies]`:
```toml
argon2 = "0.5"
hex = "0.4"
sha2 = "0.10"
```

- [ ] **Step 2: Verify workspace resolves**

```bash
cargo check --workspace 2>&1 | head -20
```
Expected: no errors (new crates not yet used anywhere, so just version resolution).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore(deps): add argon2, hex, sha2 to workspace dependencies"
```

---

## Task 2: Delete old migration files and create new migration

**Files:**
- Delete: `packages/api/migrations/00000000000001_create_tracks/`
- Delete: `packages/api/migrations/00000000000002_create_playlists/`
- Delete: `packages/api/migrations/00000000000003_add_updated_at/`
- Create: `packages/api/migrations/2026-04-13-000001_initial_schema/up.sql`
- Create: `packages/api/migrations/2026-04-13-000001_initial_schema/down.sql`
- Create: `packages/api/diesel.toml`

- [ ] **Step 1: Delete old migrations**

```bash
rm -rf packages/api/migrations/00000000000001_create_tracks
rm -rf packages/api/migrations/00000000000002_create_playlists
rm -rf packages/api/migrations/00000000000003_add_updated_at
```

- [ ] **Step 2: Create the up migration**

Create `packages/api/migrations/2026-04-13-000001_initial_schema/up.sql`:

```sql
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name          TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_admin      BOOLEAN NOT NULL DEFAULT false,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT users_name_unique UNIQUE (name)
);

CREATE TABLE access_tokens (
    token        TEXT PRIMARY KEY,
    user_id      UUID NOT NULL REFERENCES users ON DELETE CASCADE,
    device_id    TEXT NOT NULL,
    device_name  TEXT NOT NULL,
    client_name  TEXT NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX access_tokens_user_id_idx ON access_tokens (user_id);

CREATE TABLE artists (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    sort_name  TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE albums (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title      TEXT NOT NULL,
    sort_title TEXT NOT NULL,
    artist_id  UUID NOT NULL REFERENCES artists,
    year       INT4,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX albums_artist_id_idx ON albums (artist_id);

CREATE TABLE tracks (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title          TEXT NOT NULL,
    sort_title     TEXT NOT NULL,
    artist_id      UUID NOT NULL REFERENCES artists,
    album_id       UUID REFERENCES albums,
    genre          TEXT NOT NULL DEFAULT '',
    duration_ticks INT8 NOT NULL DEFAULT 0,
    track_number   INT4,
    disc_number    INT4 NOT NULL DEFAULT 1,
    file_path      TEXT NOT NULL,
    container      TEXT NOT NULL,
    bit_rate       INT4,
    sample_rate    INT4,
    channels       INT4,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT tracks_file_path_unique UNIQUE (file_path)
);
CREATE INDEX tracks_artist_id_idx ON tracks (artist_id);
CREATE INDEX tracks_album_id_idx ON tracks (album_id);

CREATE TABLE genres (
    id   UUID PRIMARY KEY,
    name TEXT NOT NULL,
    CONSTRAINT genres_name_unique UNIQUE (name)
);

CREATE TABLE images (
    item_id    UUID NOT NULL,
    image_type TEXT NOT NULL,
    file_path  TEXT NOT NULL,
    tag        TEXT NOT NULL,
    width      INT4,
    height     INT4,
    PRIMARY KEY (item_id, image_type)
);

CREATE TABLE playlists (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    overview   TEXT,
    is_smart   BOOLEAN NOT NULL DEFAULT false,
    user_id    UUID REFERENCES users,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE playlist_items (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    playlist_id UUID NOT NULL REFERENCES playlists ON DELETE CASCADE,
    item_id     UUID NOT NULL REFERENCES tracks ON DELETE CASCADE,
    position    INT4 NOT NULL,
    CONSTRAINT playlist_items_unique UNIQUE (playlist_id, item_id)
);
CREATE INDEX playlist_items_playlist_position_idx ON playlist_items (playlist_id, position);

CREATE TABLE user_data (
    user_id                 UUID NOT NULL REFERENCES users ON DELETE CASCADE,
    item_id                 UUID NOT NULL,
    item_type               TEXT NOT NULL,
    is_favorite             BOOLEAN NOT NULL DEFAULT false,
    likes                   BOOLEAN,
    play_count              INT4 NOT NULL DEFAULT 0,
    last_played_date        TIMESTAMPTZ,
    played                  BOOLEAN NOT NULL DEFAULT false,
    playback_position_ticks INT8 NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, item_id)
);
```

- [ ] **Step 3: Create the down migration**

Create `packages/api/migrations/2026-04-13-000001_initial_schema/down.sql`:

```sql
DROP TABLE IF EXISTS user_data;
DROP TABLE IF EXISTS playlist_items;
DROP TABLE IF EXISTS playlists;
DROP TABLE IF EXISTS images;
DROP TABLE IF EXISTS genres;
DROP TABLE IF EXISTS tracks;
DROP TABLE IF EXISTS albums;
DROP TABLE IF EXISTS artists;
DROP TABLE IF EXISTS access_tokens;
DROP TABLE IF EXISTS users;
```

- [ ] **Step 4: Create diesel.toml**

Create `packages/api/diesel.toml`:

```toml
[print_schema]
file = "src/db/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId", "Clone"]

[migrations]
dir = "migrations"
```

- [ ] **Step 5: Drop and recreate database, apply migration**

```bash
cd packages/api
diesel database reset
```

Expected output ends with:
```
Running migration 2026-04-13-000001_initial_schema
```

- [ ] **Step 6: Generate schema.rs from database**

```bash
cd packages/api
diesel print-schema > src/db/schema.rs
```

Verify `src/db/schema.rs` now exists and contains `table!` macros for all 10 tables.

- [ ] **Step 7: Commit**

```bash
git add packages/api/migrations/ packages/api/diesel.toml packages/api/src/db/schema.rs
git commit -m "feat(db): add Jellyfin normalized schema migration"
```

---

## Task 3: Update api Cargo.toml

**Files:**
- Modify: `packages/api/Cargo.toml`

- [ ] **Step 1: Replace Cargo.toml**

Replace the entire contents of `packages/api/Cargo.toml`:

```toml
[package]
name = "dioxus_music_api"
version = "0.1.0"
edition = "2024"

[dependencies]
# Always compiled (shared types used by client too)
chrono = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true, features = ["serde"] }

# Server-only
argon2 = { version = "0.5", optional = true }
axum = { workspace = true, optional = true }
diesel = { workspace = true, optional = true, features = ["postgres", "uuid", "serde_json", "chrono"] }
diesel-async = { workspace = true, optional = true, features = ["postgres", "bb8"] }
diesel_migrations = { workspace = true, optional = true, features = ["postgres"] }
dirs = { workspace = true, optional = true }
dotenvy = { workspace = true, optional = true }
hex = { version = "0.4", optional = true }
lofty = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
sha2 = { version = "0.10", optional = true }
tokio = { workspace = true, features = ["fs"], optional = true }
tokio-util = { workspace = true, features = ["io"], optional = true }
tracing = { workspace = true, optional = true }
walkdir = { workspace = true, optional = true }

[dev-dependencies]
tokio = { workspace = true, features = ["macros", "rt-multi-thread"] }

[features]
server = [
    "dep:argon2",
    "dep:axum",
    "dep:diesel",
    "dep:diesel-async",
    "dep:diesel_migrations",
    "dep:dirs",
    "dep:dotenvy",
    "dep:hex",
    "dep:lofty",
    "dep:rand",
    "dep:sha2",
    "dep:tokio",
    "dep:tokio-util",
    "dep:tracing",
    "dep:walkdir",
]
```

- [ ] **Step 2: Verify it compiles (server feature)**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean (no code yet except schema.rs which exists).

- [ ] **Step 3: Commit**

```bash
git add packages/api/Cargo.toml
git commit -m "chore(api): remove dioxus dependency, restructure features for Axum-only server"
```

---

## Task 4: Create module skeleton

**Files:**
- Delete: `packages/api/src/schema.rs`, `packages/api/src/models.rs`, `packages/api/src/streaming.rs`, `packages/api/src/lib.rs`
- Create: `packages/api/src/lib.rs`, `packages/api/src/state.rs`, `packages/api/src/error.rs`, `packages/api/src/types.rs`, `packages/api/src/db/mod.rs`, `packages/api/src/db/models.rs`, `packages/api/src/auth/mod.rs`, `packages/api/src/auth/password.rs`, `packages/api/src/auth/token.rs`, `packages/api/src/auth/middleware.rs`, `packages/api/src/routes/mod.rs`, `packages/api/src/routes/users.rs`

- [ ] **Step 1: Delete old source files**

```bash
rm packages/api/src/schema.rs packages/api/src/models.rs packages/api/src/streaming.rs packages/api/src/lib.rs
mkdir -p packages/api/src/db packages/api/src/auth packages/api/src/routes
```

- [ ] **Step 2: Create lib.rs with stub exports**

Create `packages/api/src/lib.rs`:

```rust
pub mod types;

#[cfg(feature = "server")]
pub mod auth;
#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod error;
#[cfg(feature = "server")]
pub mod routes;
#[cfg(feature = "server")]
pub mod state;

#[cfg(feature = "server")]
pub use state::AppState;
#[cfg(feature = "server")]
pub use routes::create_router;
#[cfg(feature = "server")]
pub use db::{create_pool, run_migrations};

/// Create default admin user if no users exist.
#[cfg(feature = "server")]
pub async fn bootstrap(state: &AppState) {
    // implemented in Task 9
}
```

- [ ] **Step 3: Create stub modules (empty, just to compile)**

Create `packages/api/src/types.rs`:
```rust
// Shared response DTOs — compiled for both server and WASM.
// Populated in Task 8.
```

Create `packages/api/src/state.rs`:
```rust
// Populated in Task 5.
```

Create `packages/api/src/error.rs`:
```rust
// Populated in Task 6.
```

Create `packages/api/src/db/mod.rs`:
```rust
// Populated in Task 5.
pub mod models;
pub mod schema;
```

Create `packages/api/src/db/models.rs`:
```rust
// Populated in Task 7.
```

Create `packages/api/src/auth/mod.rs`:
```rust
pub mod middleware;
pub mod password;
pub mod token;
```

Create `packages/api/src/auth/password.rs`:
```rust
// Populated in Task 8.
```

Create `packages/api/src/auth/token.rs`:
```rust
// Populated in Task 9.
```

Create `packages/api/src/auth/middleware.rs`:
```rust
// Populated in Task 10.
```

Create `packages/api/src/routes/mod.rs`:
```rust
pub mod users;

#[cfg(feature = "server")]
use axum::Router;
#[cfg(feature = "server")]
use crate::state::AppState;

#[cfg(feature = "server")]
pub fn create_router(_state: AppState) -> Router<AppState> {
    Router::new()
        .merge(users::router())
}
```

Create `packages/api/src/routes/users.rs`:
```rust
// Populated in Task 11.
#[cfg(feature = "server")]
use axum::Router;
#[cfg(feature = "server")]
use crate::state::AppState;

#[cfg(feature = "server")]
pub fn router() -> Router<AppState> {
    Router::new()
}
```

- [ ] **Step 4: Verify skeleton compiles**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add packages/api/src/
git commit -m "refactor(api): establish new module skeleton for Jellyfin REST server"
```

---

## Task 5: AppState and database utilities

**Files:**
- Modify: `packages/api/src/state.rs`
- Modify: `packages/api/src/db/mod.rs`

- [ ] **Step 1: Write db/mod.rs**

Replace `packages/api/src/db/mod.rs`:

```rust
pub mod models;
pub mod schema;

use diesel::PgConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;

pub type DbPool = bb8::Pool<AsyncPgConnection>;

pub async fn create_pool(database_url: &str) -> DbPool {
    let config = diesel_async::pooled_connection::AsyncDieselConnectionManager::<
        AsyncPgConnection,
    >::new(database_url);
    bb8::Pool::builder()
        .build(config)
        .await
        .expect("Failed to create database connection pool")
}

pub fn run_migrations(database_url: &str) {
    use diesel::Connection;
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    let mut conn = PgConnection::establish(database_url)
        .expect("Failed to connect to database for migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}
```

- [ ] **Step 2: Write state.rs**

Replace `packages/api/src/state.rs`:

```rust
use std::path::PathBuf;
use uuid::Uuid;
use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub image_cache_dir: PathBuf,
    pub server_id: Uuid,
    pub music_dir: PathBuf,
}
```

- [ ] **Step 3: Verify compilation**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/state.rs packages/api/src/db/mod.rs
git commit -m "feat(api): add AppState and database pool utilities"
```

---

## Task 6: ApiError type

**Files:**
- Modify: `packages/api/src/error.rs`

- [ ] **Step 1: Write error.rs**

Replace `packages/api/src/error.rs`:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Forbidden,
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        (status, Json(json!({ "Message": message }))).into_response()
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => ApiError::NotFound("Item not found".to_string()),
            _ => ApiError::Internal(e.to_string()),
        }
    }
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/error.rs
git commit -m "feat(api): add ApiError type with Axum IntoResponse impl"
```

---

## Task 7: Diesel models for all tables

**Files:**
- Modify: `packages/api/src/db/models.rs`

- [ ] **Step 1: Write db/models.rs**

Replace `packages/api/src/db/models.rs`:

```rust
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Users ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub is_admin: bool,
}

// ── Access Tokens ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::access_tokens)]
#[diesel(primary_key(token))]
pub struct AccessToken {
    pub token: String,
    pub user_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub client_name: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::access_tokens)]
pub struct NewAccessToken {
    pub token: String,
    pub user_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub client_name: String,
}

// ── Artists ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::artists)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::artists)]
pub struct NewArtist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
}

// ── Albums ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::albums)]
pub struct Album {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub year: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::albums)]
pub struct NewAlbum {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub year: Option<i32>,
}

// ── Tracks ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::tracks)]
pub struct Track {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub genre: String,
    pub duration_ticks: i64,
    pub track_number: Option<i32>,
    pub disc_number: i32,
    pub file_path: String,
    pub container: String,
    pub bit_rate: Option<i32>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::tracks)]
pub struct NewTrack {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub genre: String,
    pub duration_ticks: i64,
    pub track_number: Option<i32>,
    pub disc_number: i32,
    pub file_path: String,
    pub container: String,
    pub bit_rate: Option<i32>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
}

// ── Genres ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::genres)]
pub struct Genre {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::genres)]
pub struct NewGenre {
    pub id: Uuid,
    pub name: String,
}

// ── Images ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::images)]
pub struct Image {
    pub item_id: Uuid,
    pub image_type: String,
    pub file_path: String,
    pub tag: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::images)]
pub struct NewImage {
    pub item_id: Uuid,
    pub image_type: String,
    pub file_path: String,
    pub tag: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

// ── Playlists ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::playlists)]
pub struct Playlist {
    pub id: Uuid,
    pub name: String,
    pub overview: Option<String>,
    pub is_smart: bool,
    pub user_id: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::playlists)]
pub struct NewPlaylist {
    pub id: Uuid,
    pub name: String,
    pub overview: Option<String>,
    pub is_smart: bool,
    pub user_id: Option<Uuid>,
}

// ── Playlist Items ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::playlist_items)]
pub struct PlaylistItem {
    pub id: Uuid,
    pub playlist_id: Uuid,
    pub item_id: Uuid,
    pub position: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::playlist_items)]
pub struct NewPlaylistItem {
    pub id: Uuid,
    pub playlist_id: Uuid,
    pub item_id: Uuid,
    pub position: i32,
}

// ── User Data ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::user_data)]
pub struct UserData {
    pub user_id: Uuid,
    pub item_id: Uuid,
    pub item_type: String,
    pub is_favorite: bool,
    pub likes: Option<bool>,
    pub play_count: i32,
    pub last_played_date: Option<DateTime<Utc>>,
    pub played: bool,
    pub playback_position_ticks: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::user_data)]
pub struct NewUserData {
    pub user_id: Uuid,
    pub item_id: Uuid,
    pub item_type: String,
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/db/models.rs
git commit -m "feat(api): add Diesel models for all 10 schema tables"
```

---

## Task 8: Password hashing utilities

**Files:**
- Modify: `packages/api/src/auth/password.rs`

- [ ] **Step 1: Write the test first**

Replace `packages/api/src/auth/password.rs`:

```rust
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_correct_password() {
        let hash = hash_password("hunter2").expect("hash should succeed");
        assert!(verify_password("hunter2", &hash));
    }

    #[test]
    fn verify_rejects_wrong_password() {
        let hash = hash_password("hunter2").expect("hash should succeed");
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn hash_is_different_each_time() {
        let h1 = hash_password("same").expect("hash should succeed");
        let h2 = hash_password("same").expect("hash should succeed");
        assert_ne!(h1, h2, "argon2 hashes must use unique salts");
    }
}
```

- [ ] **Step 2: Run the tests (they should fail — argon2 not enabled yet for cfg(test))**

```bash
cargo test -p dioxus_music_api --features server auth::password
```
Expected: 3 tests pass (argon2 is behind `server` feature and tests are `#[cfg(test)]` within the module, which IS compiled under `--features server`).

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/auth/password.rs
git commit -m "feat(api): add argon2 password hashing utilities with tests"
```

---

## Task 9: Token generation and DB operations

**Files:**
- Modify: `packages/api/src/auth/token.rs`

- [ ] **Step 1: Write token.rs with tests**

Replace `packages/api/src/auth/token.rs`:

```rust
use rand::Rng;

/// Generate a random 32-byte hex token.
pub fn generate() -> String {
    let bytes: [u8; 32] = rand::thread_rng().gen();
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_64_hex_chars() {
        let t = generate();
        assert_eq!(t.len(), 64);
        assert!(t.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn tokens_are_unique() {
        let t1 = generate();
        let t2 = generate();
        assert_ne!(t1, t2);
    }
}
```

- [ ] **Step 2: Run the tests**

```bash
cargo test -p dioxus_music_api --features server auth::token
```
Expected: 2 tests pass.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/auth/token.rs
git commit -m "feat(api): add random token generation with tests"
```

---

## Task 10: AuthUser extractor (auth middleware)

**Files:**
- Modify: `packages/api/src/auth/middleware.rs`

- [ ] **Step 1: Write middleware.rs**

Replace `packages/api/src/auth/middleware.rs`:

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        models::{AccessToken, User},
        schema::{access_tokens, users},
    },
    state::AppState,
};

/// Extracted from the `Authorization: MediaBrowser Token="..."` header.
/// All protected routes receive this as a parameter.
pub struct AuthUser {
    pub user: User,
    pub token: String,
}

/// Admin-only variant — returns 403 if the user is not an admin.
pub struct AdminUser(pub AuthUser);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = extract_token(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;

        let mut conn = state
            .pool
            .get()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Look up token + join user in one query.
        let result = access_tokens::table
            .inner_join(users::table)
            .filter(access_tokens::token.eq(&token))
            .select((AccessToken::as_select(), User::as_select()))
            .first::<(AccessToken, User)>(&mut conn)
            .await
            .optional()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let (_, user) = result.ok_or(StatusCode::UNAUTHORIZED)?;

        // Update last_seen_at (best-effort, don't fail the request if this errors).
        let _ = diesel::update(
            access_tokens::table.filter(access_tokens::token.eq(&token)),
        )
        .set(access_tokens::last_seen_at.eq(chrono::Utc::now()))
        .execute(&mut conn)
        .await;

        Ok(AuthUser { user, token })
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = AuthUser::from_request_parts(parts, state).await?;
        if !auth.user.is_admin {
            return Err(StatusCode::FORBIDDEN);
        }
        Ok(AdminUser(auth))
    }
}

/// Parse `Token="<value>"` from a MediaBrowser Authorization header.
/// Header format: `MediaBrowser Client="...", Device="...", DeviceId="...", Version="...", Token="..."`
pub fn extract_token(header: &str) -> Option<String> {
    header.split(',').map(str::trim).find_map(|part| {
        let (key, val) = part.split_once('=')?;
        if key.trim().eq_ignore_ascii_case("token") {
            Some(val.trim().trim_matches('"').to_string())
        } else {
            None
        }
    })
}

/// Parse all key=value pairs from a MediaBrowser Authorization header.
pub fn parse_auth_header(header: &str) -> std::collections::HashMap<String, String> {
    let header = header
        .trim_start_matches("MediaBrowser ")
        .trim_start_matches("mediabrowser ");
    header
        .split(',')
        .filter_map(|part| {
            let (k, v) = part.trim().split_once('=')?;
            Some((
                k.trim().to_string(),
                v.trim().trim_matches('"').to_string(),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_token_from_full_header() {
        let header = r#"MediaBrowser Client="Finamp", Device="iPhone", DeviceId="abc", Version="1.0", Token="mytoken123""#;
        assert_eq!(extract_token(header), Some("mytoken123".to_string()));
    }

    #[test]
    fn returns_none_when_token_absent() {
        let header = r#"MediaBrowser Client="App", Device="PC", DeviceId="xyz", Version="1.0""#;
        assert_eq!(extract_token(header), None);
    }

    #[test]
    fn parses_all_fields() {
        let header = r#"MediaBrowser Client="MyApp", Device="Desktop", DeviceId="dev1", Version="2.0", Token="tok""#;
        let map = parse_auth_header(header);
        assert_eq!(map.get("Client").map(String::as_str), Some("MyApp"));
        assert_eq!(map.get("DeviceId").map(String::as_str), Some("dev1"));
        assert_eq!(map.get("Token").map(String::as_str), Some("tok"));
    }
}
```

- [ ] **Step 2: Run the unit tests**

```bash
cargo test -p dioxus_music_api --features server auth::middleware
```
Expected: 3 tests pass (unit tests — no DB required).

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/auth/middleware.rs
git commit -m "feat(api): add AuthUser and AdminUser Axum extractors with header parsing tests"
```

---

## Task 11: Shared response types (BaseItemDto and friends)

**Files:**
- Modify: `packages/api/src/types.rs`

- [ ] **Step 1: Write types.rs**

Replace `packages/api/src/types.rs`:

```rust
//! Shared response DTOs — serialized by the server, deserialized by the client.
//! No feature gate: compiled for both server and WASM.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The universal Jellyfin item object. `item_type` discriminates between
/// Audio, MusicAlbum, and MusicArtist.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDto {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: String,
    pub server_id: Uuid,

    // Track-specific
    pub album: Option<String>,
    pub album_id: Option<Uuid>,
    pub album_primary_image_tag: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Option<Vec<NameGuidPair>>,
    pub artists: Option<Vec<String>>,
    pub artist_items: Option<Vec<NameGuidPair>>,
    pub genre_items: Option<Vec<NameGuidPair>>,
    pub genres: Option<Vec<String>>,
    pub run_time_ticks: Option<i64>,
    pub track_number: Option<i32>,
    pub index_number: Option<i32>,      // same as track_number, Jellyfin alias
    pub parent_index_number: Option<i32>, // disc number
    pub container: Option<String>,
    pub media_type: Option<String>,

    // Album-specific
    pub production_year: Option<i32>,

    // Shared
    pub image_tags: Option<std::collections::HashMap<String, String>>,
    pub user_data: Option<UserItemDataDto>,
    pub date_created: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameGuidPair {
    pub name: String,
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserItemDataDto {
    pub is_favorite: bool,
    pub likes: Option<bool>,
    pub play_count: i32,
    pub last_played_date: Option<DateTime<Utc>>,
    pub played: bool,
    pub playback_position_ticks: i64,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemsResult {
    pub items: Vec<BaseItemDto>,
    pub total_record_count: i64,
    pub start_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserDto {
    pub id: Uuid,
    pub name: String,
    pub server_id: Uuid,
    pub has_password: bool,
    pub has_configured_password: bool,
    pub enable_auto_login: bool,
    pub last_login_date: Option<DateTime<Utc>>,
    pub last_activity_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResult {
    pub user: UserDto,
    pub access_token: String,
    pub server_id: Uuid,
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check -p dioxus_music_api
cargo check -p dioxus_music_api --features server
```
Expected: both clean (types.rs has no feature gate).

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/types.rs
git commit -m "feat(api): add shared Jellyfin response DTOs (BaseItemDto, UserDto, etc.)"
```

---

## Task 12: User routes

**Files:**
- Modify: `packages/api/src/routes/users.rs`
- Modify: `packages/api/src/routes/mod.rs`
- Modify: `packages/api/src/lib.rs`

- [ ] **Step 1: Write users.rs**

Replace `packages/api/src/routes/users.rs`:

```rust
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::{
        middleware::{AdminUser, AuthUser, parse_auth_header},
        password, token,
    },
    db::{
        models::{NewAccessToken, NewUser, User},
        schema::{access_tokens, users},
    },
    error::ApiError,
    state::AppState,
    types::{AuthenticationResult, UserDto},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Users/AuthenticateByName", post(authenticate_by_name))
        .route("/Users", get(list_users))
        .route("/Users/{user_id}", get(get_user))
        .route("/Users/{user_id}/Password", post(change_password))
        .route("/Users/{user_id}", delete(delete_user))
        .route("/Sessions/Logout", delete(logout))
        .route("/Auth/Keys", get(list_api_keys))
        .route("/Auth/Keys", post(create_api_key))
        .route("/Auth/Keys/{key}", delete(delete_api_key))
}

// ── DTOs ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateByNameRequest {
    pub username: String,
    pub pw: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangePasswordRequest {
    pub current_pw: String,
    pub new_pw: String,
}

fn user_to_dto(user: &User, server_id: Uuid) -> UserDto {
    UserDto {
        id: user.id,
        name: user.name.clone(),
        server_id,
        has_password: true,
        has_configured_password: true,
        enable_auto_login: false,
        last_login_date: None,
        last_activity_date: None,
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────

/// POST /Users/AuthenticateByName
async fn authenticate_by_name(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<AuthenticateByNameRequest>,
) -> Result<Json<AuthenticationResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let user: Option<User> = users::table
        .filter(users::name.eq(&body.username))
        .first::<User>(&mut conn)
        .await
        .optional()?;

    let user = user.ok_or(ApiError::Unauthorized)?;

    if !password::verify_password(&body.pw, &user.password_hash) {
        return Err(ApiError::Unauthorized);
    }

    // Parse client/device info from Authorization header (present without Token on pre-auth).
    let auth_info = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(parse_auth_header)
        .unwrap_or_default();

    let new_token = NewAccessToken {
        token: token::generate(),
        user_id: user.id,
        device_id: auth_info.get("DeviceId").cloned().unwrap_or_default(),
        device_name: auth_info.get("Device").cloned().unwrap_or_default(),
        client_name: auth_info.get("Client").cloned().unwrap_or_default(),
    };

    let inserted_token: crate::db::models::AccessToken = diesel::insert_into(access_tokens::table)
        .values(&new_token)
        .get_result(&mut conn)
        .await?;

    Ok(Json(AuthenticationResult {
        user: user_to_dto(&user, state.server_id),
        access_token: inserted_token.token,
        server_id: state.server_id,
    }))
}

/// GET /Users — admin only
async fn list_users(
    AdminUser(_): AdminUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserDto>>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let all: Vec<User> = users::table.load(&mut conn).await?;
    let dtos: Vec<UserDto> = all.iter().map(|u| user_to_dto(u, state.server_id)).collect();
    Ok(Json(dtos))
}

/// GET /Users/{userId}
async fn get_user(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserDto>, ApiError> {
    // Non-admin users can only read their own profile.
    if auth.user.id != user_id && !auth.user.is_admin {
        return Err(ApiError::Forbidden);
    }
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let user: User = users::table
        .filter(users::id.eq(user_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;
    Ok(Json(user_to_dto(&user, state.server_id)))
}

/// POST /Users/{userId}/Password
async fn change_password(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    if auth.user.id != user_id && !auth.user.is_admin {
        return Err(ApiError::Forbidden);
    }
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let user: User = users::table
        .filter(users::id.eq(user_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    if !password::verify_password(&body.current_pw, &user.password_hash) {
        return Err(ApiError::Unauthorized);
    }

    let new_hash =
        password::hash_password(&body.new_pw).map_err(|e| ApiError::Internal(e.to_string()))?;

    diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(users::password_hash.eq(new_hash))
        .execute(&mut conn)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /Users/{userId} — admin only
async fn delete_user(
    AdminUser(_): AdminUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(users::table.filter(users::id.eq(user_id)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /Sessions/Logout
async fn logout(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(access_tokens::table.filter(access_tokens::token.eq(&auth.token)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /Auth/Keys — admin only (stub: tokens serve as API keys for now)
async fn list_api_keys(
    AdminUser(auth): AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let tokens: Vec<crate::db::models::AccessToken> = access_tokens::table
        .filter(access_tokens::user_id.eq(auth.user.id))
        .load(&mut conn)
        .await?;
    let items: Vec<serde_json::Value> = tokens
        .iter()
        .map(|t| {
            serde_json::json!({
                "AccessToken": t.token,
                "DeviceId": t.device_id,
                "AppName": t.client_name,
                "DateCreated": t.created_at,
            })
        })
        .collect();
    let count = items.len();
    Ok(Json(serde_json::json!({ "Items": items, "TotalRecordCount": count })))
}

/// POST /Auth/Keys — admin only (creates a new static token for the admin user)
async fn create_api_key(
    AdminUser(auth): AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let new_token = NewAccessToken {
        token: token::generate(),
        user_id: auth.user.id,
        device_id: "api-key".to_string(),
        device_name: "API Key".to_string(),
        client_name: "Static".to_string(),
    };
    let inserted: crate::db::models::AccessToken = diesel::insert_into(access_tokens::table)
        .values(&new_token)
        .get_result(&mut conn)
        .await?;
    Ok(Json(serde_json::json!({ "AccessToken": inserted.token })))
}

/// DELETE /Auth/Keys/{key} — admin only
async fn delete_api_key(
    AdminUser(_): AdminUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(access_tokens::table.filter(access_tokens::token.eq(key)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
```

- [ ] **Step 2: Update routes/mod.rs to mount users router**

Replace `packages/api/src/routes/mod.rs`:

```rust
pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(users::router())
        .with_state(state)
}
```

- [ ] **Step 3: Update lib.rs to export create_router**

Replace `packages/api/src/lib.rs`:

```rust
pub mod types;

#[cfg(feature = "server")]
pub mod auth;
#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod error;
#[cfg(feature = "server")]
pub mod routes;
#[cfg(feature = "server")]
pub mod state;

#[cfg(feature = "server")]
pub use db::{create_pool, run_migrations};
#[cfg(feature = "server")]
pub use routes::create_router;
#[cfg(feature = "server")]
pub use state::AppState;

/// Create default admin user if the users table is empty (first-run bootstrap).
#[cfg(feature = "server")]
pub async fn bootstrap(state: &AppState) {
    use auth::password;
    use db::{
        models::NewUser,
        schema::users,
    };
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use uuid::Uuid;

    let Ok(mut conn) = state.pool.get().await else { return };

    let count: i64 = users::table
        .count()
        .get_result(&mut conn)
        .await
        .unwrap_or(1); // default to 1 so we don't create a user on DB error

    if count > 0 {
        return;
    }

    let admin_user = std::env::var("DIOXUS_MUSIC_ADMIN_USER")
        .unwrap_or_else(|_| "admin".to_string());
    let admin_pass = std::env::var("DIOXUS_MUSIC_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "changeme".to_string());

    let hash = match password::hash_password(&admin_pass) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Failed to hash admin password: {e}");
            return;
        }
    };

    let new_user = NewUser {
        id: Uuid::new_v4(),
        name: admin_user.clone(),
        password_hash: hash,
        is_admin: true,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .await
    {
        Ok(_) => tracing::info!("Created default admin user '{admin_user}'"),
        Err(e) => tracing::error!("Failed to create admin user: {e}"),
    }
}
```

- [ ] **Step 4: Verify compilation**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add packages/api/src/routes/ packages/api/src/lib.rs
git commit -m "feat(api): add user auth routes (AuthenticateByName, CRUD, API keys)"
```

---

## Task 13: Update web crate — stub views, mount new router

**Files:**
- Modify: `packages/web/src/main.rs`
- Modify: `packages/web/src/views/*.rs` (stub all view bodies)
- Modify: `packages/web/Cargo.toml`

- [ ] **Step 1: Add dirs to web Cargo.toml**

In `packages/web/Cargo.toml`, add to `[dependencies]`:

```toml
dirs = { workspace = true, optional = true }
uuid = { workspace = true, features = ["v4"] }
```

Add `"dep:dirs"` to the `server` feature list.

- [ ] **Step 2: Stub out all view components**

The views in `packages/web/src/views/` call server functions that no longer exist. Replace each view's body with a stub so the crate compiles. For each file in `packages/web/src/views/`, replace the component body with `rsx! { "Coming soon" }`.

Example — replace the body of each `#[component]` function (keep the function signature, replace everything inside the `{}` block):

```rust
// packages/web/src/views/library.rs  (and all other view files)
use dioxus::prelude::*;

#[component]
pub fn Library() -> Element {
    rsx! { "Coming soon" }
}
```

Repeat for: `AlbumDetail`, `Artists`, `Playlists`, `PlaylistView`, `Downloads`, `NowPlaying`, `PlaylistSidebarSection`.

- [ ] **Step 3: Replace web/src/main.rs**

Replace the entire `packages/web/src/main.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, ServerConfig, Sidebar};
use uuid::Uuid;
use views::{
    AlbumDetail, Artists, Downloads, Library, NowPlaying, PlaylistSidebarSection, PlaylistView,
    Playlists,
};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Library {},
        #[route("/album/:name")]
        AlbumDetail { name: String },
        #[route("/artists")]
        Artists {},
        #[route("/playlists")]
        Playlists {},
        #[route("/playlist/:id")]
        PlaylistView { id: Uuid },
        #[route("/downloads")]
        Downloads {},
        #[route("/now-playing")]
        NowPlaying {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        dioxus_logger::initialize_default();
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        let music_dir = std::env::var("MUSIC_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::audio_dir().unwrap_or_else(|| {
                    dirs::home_dir().expect("home dir must exist").join("Music")
                })
            });

        let image_cache_dir = std::env::var("IMAGE_CACHE_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::data_local_dir()
                    .unwrap_or_else(|| {
                        dirs::home_dir().expect("home dir must exist").join(".local/share")
                    })
                    .join("dioxus_music/images")
            });

        // Run Diesel migrations (blocking, must complete before serving).
        {
            let url = database_url.clone();
            tokio::task::spawn_blocking(move || dioxus_music_api::run_migrations(&url))
                .await
                .expect("Migration thread panicked");
        }

        let pool = dioxus_music_api::create_pool(&database_url).await;

        let server_id: Uuid = std::env::var("SERVER_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(Uuid::new_v4);

        let state = dioxus_music_api::AppState {
            pool,
            image_cache_dir,
            server_id,
            music_dir,
        };

        // Create default admin user if no users exist.
        dioxus_music_api::bootstrap(&state).await;

        // Mount the Jellyfin REST router alongside the Dioxus app.
        let api_router = dioxus_music_api::create_router(state);

        let router = dioxus::server::router(App).merge(api_router);

        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn AppLayout() -> Element {
    use_context_provider(|| ServerConfig { base_url: String::new() });
    use_player_state_provider();
    let nav = navigator();
    let current_route = use_route::<Route>();
    let is_now_playing = matches!(current_route, Route::NowPlaying {});

    rsx! {
        AppShell {
            player_bar_hidden: is_now_playing,
            on_player_expand: move |_| { nav.push(Route::NowPlaying {}); },
            sidebar: rsx! {
                Sidebar {
                    Link { class: "sidebar__nav-item", to: Route::Artists {}, "Artists" }
                    Link { class: "sidebar__nav-item", to: Route::Library {}, "Albums" }
                    Link { class: "sidebar__nav-item", to: Route::Playlists {}, "Playlists" }
                    Link { class: "sidebar__nav-item", to: Route::Downloads {}, "Downloads" }
                    PlaylistSidebarSection {}
                }
            },
            Outlet::<Route> {}
        }
    }
}
```

- [ ] **Step 4: Build the server to verify everything compiles**

```bash
cargo build -p dioxus_music_web --features server
```
Expected: compiles cleanly. The app will serve but all routes show "Coming soon".

- [ ] **Step 5: Smoke-test the server**

```bash
dx serve --package dioxus_music_web
```

In another terminal:
```bash
# Should return 401 (route exists, auth required)
curl -s -o /dev/null -w "%{http_code}" -X POST http://localhost:8080/Users/AuthenticateByName \
  -H "Content-Type: application/json" \
  -d '{"Username":"admin","Pw":"wrongpass"}'
# Expected: 401

# Should return 200 with AuthenticationResult
curl -s -X POST http://localhost:8080/Users/AuthenticateByName \
  -H "Content-Type: application/json" \
  -H 'Authorization: MediaBrowser Client="Test", Device="curl", DeviceId="test1", Version="1.0"' \
  -d '{"Username":"admin","Pw":"changeme"}' | jq .
# Expected: { "User": { "Id": "...", "Name": "admin", ... }, "AccessToken": "...", "ServerId": "..." }
```

- [ ] **Step 6: Commit**

```bash
git add packages/web/ packages/api/src/
git commit -m "feat: wire Jellyfin REST router into web crate, stub views for migration"
```

---

## Plan 1 Complete

At this point:
- New 10-table schema is live
- `POST /Users/AuthenticateByName` works and returns a token
- All `/Users/*`, `/Sessions/Logout`, `/Auth/Keys/*` routes are functional
- The web app serves (all views show "Coming soon" stubs)
- The server bootstraps an admin user on first run

**Next:** `2026-04-13-jellyfin-plan-2-scanner-library-api.md` — scanner migration and all library browse/stream routes.
