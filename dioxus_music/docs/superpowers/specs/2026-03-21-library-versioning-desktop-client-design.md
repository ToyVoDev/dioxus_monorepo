# Library Versioning + Desktop as Pure Client — Design Specification

## 1. Overview

Two changes:

1. **Library versioning** — Add `updated_at` timestamps to tracks/playlists tables, expose `get_library_version()` server function returning `max(updated_at)`. Foundation for future sync.
2. **Desktop as pure client** — Remove the embedded Axum server from the desktop crate. Desktop becomes a lightweight client that connects to the web server's API via HTTP. Server URL configured via `SERVER_URL` env var.

---

## 2. Server Changes (api + web crates)

### 2.1 Migration: add `updated_at`

New migration adding `updated_at TIMESTAMPTZ` columns to `tracks` and `playlists` tables, with auto-update triggers. This aligns with Phase 1 of the existing sync design doc (`docs/plans/2026-03-11-offline-sync-and-automotive-design.md`, Section "Server API Changes").

`up.sql`:
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

`down.sql`:
```sql
DROP TRIGGER IF EXISTS tracks_updated_at ON tracks;
DROP TRIGGER IF EXISTS playlists_updated_at ON playlists;
DROP FUNCTION IF EXISTS update_updated_at_column();
ALTER TABLE tracks DROP COLUMN IF EXISTS updated_at;
ALTER TABLE playlists DROP COLUMN IF EXISTS updated_at;
```

### 2.2 Model changes

Add `chrono` dependency to workspace and api crate (with `serde` feature).

Update `TrackSummary` to include `updated_at`:
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

### 2.3 New server functions

**`get_library_version()`** — returns `Option<DateTime<Utc>>`:
```rust
#[server]
pub async fn get_library_version() -> Result<Option<chrono::DateTime<chrono::Utc>>, ServerFnError> {
    // SELECT max(updated_at) FROM tracks
    // Only checks tracks table — playlists version can be added separately if needed
    use diesel::dsl::max;
    use diesel_async::RunQueryDsl;
    let pool = extract_pool().await?;
    let mut conn = pool.get().await?;
    let result: Option<chrono::DateTime<chrono::Utc>> = tracks::table
        .select(max(tracks::updated_at))
        .first(&mut conn)
        .await?;
    Ok(result)
}
```

Returns `None` if the library is empty, otherwise the most recent `updated_at` timestamp from the tracks table. Clients compare this against their last-known version to decide whether to re-fetch.

**`get_health()`** — connectivity check (aligns with the existing sync design doc's `/api/health` endpoint):
```rust
#[server]
pub async fn get_health() -> Result<String, ServerFnError> {
    Ok("ok".to_string())
}
```

### 2.4 Schema regeneration

After running the migration, regenerate `schema.rs` via `diesel print-schema`. The tracks and playlists tables will include `updated_at -> Timestamptz`.

---

## 3. ServerConfig Context

### 3.1 Definition

A shared context struct for configuring the server base URL. Defined in `packages/ui/src/lib.rs` (or a small `config.rs` module):

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub base_url: String,
}
```

Exported from `packages/ui/src/lib.rs`.

### 3.2 Providing the context

Each platform provides `ServerConfig` via `use_context_provider()` at the layout component level (same pattern as `use_player_state_provider()`):

**Web (`AppLayout`):**
```rust
use_context_provider(|| ServerConfig { base_url: String::new() }); // relative URLs
```

**Desktop (`DesktopLayout`):**
```rust
let server_url = std::env::var("SERVER_URL")
    .unwrap_or_else(|_| "http://localhost:8080".to_string());
use_context_provider(|| ServerConfig { base_url: server_url });
```

### 3.3 Consuming the context

`packages/ui/src/audio.rs` reads `ServerConfig` to build the stream URL:

```rust
let config = use_context::<ServerConfig>();
let audio_src = track_info
    .as_ref()
    .map(|t| format!("{}/stream/{}", config.base_url, t.id))
    .unwrap_or_default();
```

For web, `base_url` is empty so the URL becomes `/stream/{id}` (relative, resolved by the browser). For desktop, it becomes `http://server:port/stream/{id}`.

---

## 4. Desktop Changes

### 4.1 Remove embedded server

Remove the entire `#[cfg(feature = "server")] dioxus::serve(...)` block from desktop `main.rs`. This includes DB pool creation, migration running, streaming route registration, and background scan spawning.

### 4.2 Update dependencies

Update `packages/desktop/Cargo.toml`:

```toml
[dependencies]
dioxus = { workspace = true, features = ["router", "fullstack"] }
dioxus_music_ui = { workspace = true }
kinetic_ui = { workspace = true }
uuid = { workspace = true }

[features]
default = []
desktop = ["dioxus/desktop"]
```

Removed: `dioxus_music_api` (accessed transitively through `dioxus_music_ui`), `dotenvy`, `tokio`, `axum`. Removed: `server` feature (desktop never compiles server side).

**Note on dependency chain:** `dioxus_music_ui` depends on `dioxus_music_api`. The `#[server]`/`#[get]`/`#[post]` macros in `dioxus_music_api` compile to HTTP client stubs when the `server` feature is NOT enabled. Desktop gets these client stubs transitively.

### 4.3 Simplified main.rs

```rust
use dioxus::prelude::*;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, ServerConfig, Sidebar};
// ... view imports ...

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

#[component]
fn DesktopLayout() -> Element {
    // Read server URL from environment (captured at startup)
    let server_url = std::env::var("SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    use_player_state_provider();
    use_context_provider(|| ServerConfig { base_url: server_url });

    rsx! {
        // ... drag regions, AppShell, etc.
    }
}
```

No server block. No DB pool. No migrations. No streaming routes.

### 4.4 View changes

**None.** All existing views call `use_server_future(dioxus_music_api::get_library)` etc. With the `server` feature disabled, these compile to HTTP client calls. Dioxus fullstack routes these to the configured server URL.

### 4.5 Dioxus fullstack server URL

Dioxus fullstack client needs to know where to send server function HTTP calls. This is separate from the `ServerConfig` context (which is for the audio stream URL).

**Check how Dioxus 0.7 fullstack handles this for desktop.** Possible approaches:
- `dioxus-cli-config` may provide a `set_server_url()` API
- The `DIOXUS_SERVER_URL` environment variable may be read by the generated client code
- The server URL may be compiled in at build time by `dx serve`

If `dx serve` for desktop automatically proxies server function calls to the dev server, this may just work. For production, an explicit configuration mechanism is needed. **The implementer should investigate and document the actual mechanism.**

---

## 5. New Files

| File | Responsibility |
|---|---|
| `packages/api/migrations/NNNN_add_updated_at/up.sql` | Add updated_at columns + triggers |
| `packages/api/migrations/NNNN_add_updated_at/down.sql` | Reverse migration |

## 6. Modified Files

| File | Changes |
|---|---|
| `Cargo.toml` (workspace root) | Add `chrono` to workspace deps (if not present) |
| `packages/api/Cargo.toml` | Add `chrono` dependency |
| `packages/api/src/schema.rs` | Regenerated (new updated_at columns) |
| `packages/api/src/models.rs` | Add `updated_at` to `TrackSummary` |
| `packages/api/src/lib.rs` | Add `get_library_version()` and `get_health()` server functions |
| `packages/ui/src/audio.rs` | Use `ServerConfig` context for stream URL |
| `packages/ui/src/lib.rs` | Export `ServerConfig` struct |
| `packages/desktop/Cargo.toml` | Remove server deps and server feature |
| `packages/desktop/src/main.rs` | Remove server block, add ServerConfig context |
| `packages/web/src/main.rs` | Add ServerConfig context provider (empty base_url) |

---

## 7. Out of Scope

- Local SQLite database on desktop (future — Phase 1 of sync plan)
- Download manager / offline caching
- Connectivity detection (`ConnectionStatus` signal)
- Settings UI for server URL (future — use env var for now)
- Incremental sync (`?since=timestamp` parameter)
- Album art extraction
- Mobile client changes
- Playlist versioning (only tracks `updated_at` is checked for now)
