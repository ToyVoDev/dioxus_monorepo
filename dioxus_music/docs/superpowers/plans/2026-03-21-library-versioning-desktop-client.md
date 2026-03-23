# Library Versioning + Desktop as Pure Client Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `updated_at` timestamps to the database for change detection, expose a library version endpoint, and convert the desktop crate from an embedded server to a pure client connecting to the web server's API.

**Architecture:** Server gets `updated_at` columns + triggers on tracks/playlists tables, plus `get_library_version()` and `get_health()` server functions. Desktop removes its embedded Axum server, drops server-only dependencies, and connects to the web server via HTTP. A `ServerConfig` context provides the base URL for audio streaming.

**Tech Stack:** Rust nightly, Dioxus 0.7, Diesel (PostgreSQL), chrono

**Spec:** `dioxus_music/docs/superpowers/specs/2026-03-21-library-versioning-desktop-client-design.md`

**No tests.** Verification: `cargo check` and `dx serve`.

---

## File Structure

### New files

| File | Responsibility |
|---|---|
| `packages/api/migrations/00000000000003_add_updated_at/up.sql` | Add updated_at columns + triggers |
| `packages/api/migrations/00000000000003_add_updated_at/down.sql` | Reverse migration |

### Modified files

| File | Changes |
|---|---|
| `Cargo.toml` (workspace) | Ensure chrono has serde feature |
| `packages/api/Cargo.toml` | Add chrono dependency |
| `packages/api/src/schema.rs` | Regenerated with updated_at columns |
| `packages/api/src/models.rs` | Add updated_at to TrackSummary |
| `packages/api/src/lib.rs` | Add get_library_version() and get_health() |
| `packages/ui/src/lib.rs` | Export ServerConfig |
| `packages/ui/src/audio.rs` | Use ServerConfig for stream URL |
| `packages/desktop/Cargo.toml` | Remove server deps |
| `packages/desktop/src/main.rs` | Remove server block, add ServerConfig |
| `packages/web/src/main.rs` | Add ServerConfig context |

---

## Task 1: Add updated_at migration + regenerate schema

**Files:**
- Create: `packages/api/migrations/00000000000003_add_updated_at/up.sql`
- Create: `packages/api/migrations/00000000000003_add_updated_at/down.sql`
- Modify: `packages/api/src/schema.rs`

- [ ] **Step 1: Create migration files**

Create `packages/api/migrations/00000000000003_add_updated_at/up.sql`:
```sql
ALTER TABLE tracks ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();
ALTER TABLE playlists ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER tracks_updated_at BEFORE UPDATE ON tracks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER playlists_updated_at BEFORE UPDATE ON playlists
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

Create `packages/api/migrations/00000000000003_add_updated_at/down.sql`:
```sql
DROP TRIGGER IF EXISTS tracks_updated_at ON tracks;
DROP TRIGGER IF EXISTS playlists_updated_at ON playlists;
DROP FUNCTION IF EXISTS update_updated_at_column();
ALTER TABLE tracks DROP COLUMN IF EXISTS updated_at;
ALTER TABLE playlists DROP COLUMN IF EXISTS updated_at;
```

- [ ] **Step 2: Update schema.rs manually**

Since we can't run `diesel print-schema` without a running DB, manually add `updated_at` to the schema. In `packages/api/src/schema.rs`, add `updated_at -> Timestamptz,` to both the `tracks` and `playlists` table definitions:

```rust
diesel::table! {
    tracks (id) {
        id -> Uuid,
        title -> Text,
        artist -> Text,
        album -> Text,
        genre -> Text,
        duration_secs -> Int4,
        file_path -> Text,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    playlists (id) {
        id -> Uuid,
        name -> Text,
        playlist_type -> Text,
        rules -> Nullable<Jsonb>,
        updated_at -> Timestamptz,
    }
}
```

Keep the rest of schema.rs unchanged (playlist_tracks table, joinable!, allow_tables_to_appear_in_same_query!).

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_api`

Note: This will fail because `TrackSummary` doesn't include `updated_at` yet. That's fixed in Task 2.

- [ ] **Step 4: Commit**

```bash
git add packages/api/migrations/ packages/api/src/schema.rs
git commit -m "feat(dioxus_music): add updated_at migration and schema for tracks/playlists"
```

---

## Task 2: Update models + add server functions

**Files:**
- Modify: `packages/api/Cargo.toml`
- Modify: `packages/api/src/models.rs`
- Modify: `packages/api/src/lib.rs`

- [ ] **Step 1: Add chrono to api crate**

In `packages/api/Cargo.toml`, add to `[dependencies]`:
```toml
chrono = { workspace = true, features = ["serde"] }
```

Also update the workspace root `Cargo.toml` to ensure chrono has serde:
```toml
chrono = { version = "0", features = ["serde"] }
```

- [ ] **Step 2: Add updated_at to TrackSummary**

In `packages/api/src/models.rs`, update `TrackSummary`:
```rust
pub struct TrackSummary {
    pub id: Uuid,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub duration_secs: i32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

Add `use chrono;` or use the full path. The `Serialize`/`Deserialize` derives handle chrono with the `serde` feature.

- [ ] **Step 3: Add get_library_version() and get_health() to lib.rs**

In `packages/api/src/lib.rs`, add:

```rust
#[server]
pub async fn get_library_version() -> Result<Option<chrono::DateTime<chrono::Utc>>, ServerFnError> {
    use diesel::dsl::max;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use axum::Extension;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let result: Option<chrono::DateTime<chrono::Utc>> = schema::tracks::table
        .select(max(schema::tracks::updated_at))
        .first(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    Ok(result)
}

#[server]
pub async fn get_health() -> Result<String, ServerFnError> {
    Ok("ok".to_string())
}
```

- [ ] **Step 4: Verify**

Run: `cargo check -p dioxus_music_api`

Then: `cargo check -p dioxus_music_web` (ensure web still compiles with the model change)

- [ ] **Step 5: Commit**

```bash
git add packages/api/ Cargo.toml
git commit -m "feat(dioxus_music): add updated_at to models, get_library_version() and get_health()"
```

---

## Task 3: Add ServerConfig context + update audio.rs

**Files:**
- Modify: `packages/ui/src/lib.rs`
- Modify: `packages/ui/src/audio.rs`
- Modify: `packages/web/src/main.rs`

- [ ] **Step 1: Define and export ServerConfig**

In `packages/ui/src/lib.rs`, add:
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub base_url: String,
}
```

- [ ] **Step 2: Update audio.rs to use ServerConfig**

In `packages/ui/src/audio.rs`, read the context and build the full URL:

```rust
use crate::ServerConfig;

pub fn render_audio_element() -> Element {
    let mut player = use_player_state();
    let config = use_context::<ServerConfig>();
    let track_info = player.read().current_track.clone();
    let repeat_mode = player.read().repeat_mode;

    let audio_src = track_info
        .as_ref()
        .map(|t| format!("{}/stream/{}", config.base_url, t.id))
        .unwrap_or_default();
    // ... rest unchanged
}
```

- [ ] **Step 3: Add ServerConfig to web crate**

In `packages/web/src/main.rs`, in the `AppLayout` component, add:
```rust
use dioxus_music_ui::ServerConfig;
// ... in AppLayout:
use_context_provider(|| ServerConfig { base_url: String::new() });
```

Empty base_url means URLs stay relative (e.g., `/stream/{id}`), which is correct for the web crate since the browser resolves relative URLs against the server origin.

- [ ] **Step 4: Verify**

Run: `cargo check -p dioxus_music_ui` and `cargo check -p dioxus_music_web`

- [ ] **Step 5: Commit**

```bash
git add packages/ui/ packages/web/src/main.rs
git commit -m "feat(dioxus_music): add ServerConfig context for configurable stream URL"
```

---

## Task 4: Convert desktop to pure client

**Files:**
- Modify: `packages/desktop/Cargo.toml`
- Modify: `packages/desktop/src/main.rs`

- [ ] **Step 1: Strip server dependencies from Cargo.toml**

Replace `packages/desktop/Cargo.toml`:
```toml
[package]
name = "dioxus_music_desktop"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { workspace = true, features = ["router", "fullstack"] }
dioxus_music_ui = { workspace = true }
kinetic_ui = { workspace = true }
uuid = { workspace = true }

[features]
default = []
desktop = ["dioxus/desktop"]
```

Removed: `dioxus_music_api`, `dotenvy`, `tokio`, `axum`. Removed: `server` feature.

- [ ] **Step 2: Rewrite main.rs — remove server block, add ServerConfig**

Remove the entire `#[cfg(feature = "server")] dioxus::serve(...)` block (lines 34-64).

Simplify the `main()` function — only the desktop launch remains:

```rust
fn main() {
    use dioxus::desktop::Config;
    use dioxus::desktop::tao::window::WindowBuilder;

    let mut wb = WindowBuilder::new().with_title(env!("CARGO_PKG_NAME"));

    #[cfg(target_os = "macos")]
    {
        use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;
        wb = wb
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true);
    }

    let config = Config::new().with_window(wb);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}
```

Remove the `#[cfg(not(any(feature = "desktop", feature = "server")))]` fallback main.

In `DesktopLayout`, add `ServerConfig` context:
```rust
use dioxus_music_ui::ServerConfig;

#[component]
fn DesktopLayout() -> Element {
    let server_url = std::env::var("SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    use_player_state_provider();
    use_context_provider(|| ServerConfig { base_url: server_url });

    // ... rest of DesktopLayout unchanged
}
```

Remove unused imports: `dioxus_music_api`, `dotenvy`, `tokio`, `axum`.

**Note:** The views still call `use_server_future(dioxus_music_api::get_library)` etc. These are accessed transitively through `dioxus_music_ui` → `dioxus_music_api`. Without the `server` feature, they compile to HTTP client stubs. Dioxus fullstack routes these calls to the server.

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_desktop`

If there are compilation issues around server function client stubs not knowing the server URL, investigate how Dioxus 0.7 fullstack configures the remote URL for desktop. Check `dioxus-cli-config` or environment variables like `DIOXUS_SERVER_URL`.

- [ ] **Step 4: Commit**

```bash
git add packages/desktop/
git commit -m "feat(dioxus_music): convert desktop to pure client, remove embedded server"
```

---

## Task 5: Verify end-to-end + cleanup

**Files:**
- Various

- [ ] **Step 1: Run the web server**

Start the web server: `cd dioxus_music/packages/web && dx serve`

Verify:
- Library loads at `http://localhost:8080/`
- `curl http://localhost:8080/api/health` returns `"ok"`
- The migration runs (check logs for updated_at columns)

- [ ] **Step 2: Test desktop client**

In a separate terminal, run:
```bash
SERVER_URL=http://localhost:8080 dx serve --package dioxus_music_desktop
```

Verify:
- Desktop app launches
- Library loads (tracks fetched from web server)
- Clicking a track shows it in PlayerBar
- Audio streams from the web server (check that `/stream/{id}` resolves correctly)

- [ ] **Step 3: Run clippy**

Run: `cargo clippy -p dioxus_music_api -p dioxus_music_ui -p dioxus_music_web -p dioxus_music_desktop`

Fix any warnings.

- [ ] **Step 4: Commit**

```bash
git add -A dioxus_music/
git commit -m "feat(dioxus_music): verify library versioning + desktop client, clippy cleanup"
```

---

## Summary

| Task | Description | Key Files |
|---|---|---|
| 1 | Migration + schema | `migrations/`, `schema.rs` |
| 2 | Models + server functions | `models.rs`, `lib.rs`, `Cargo.toml` |
| 3 | ServerConfig context + audio URL | `ui/src/lib.rs`, `audio.rs`, `web/main.rs` |
| 4 | Desktop as pure client | `desktop/Cargo.toml`, `desktop/main.rs` |
| 5 | End-to-end verification | Various |

Tasks 1-2 are sequential (migration before models). Task 3 is independent (can parallel with 1-2). Task 4 depends on Task 3 (needs ServerConfig). Task 5 is final verification.
