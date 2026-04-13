# Jellyfin API Migration Design

**Date:** 2026-04-13  
**Project:** `dioxus_music`  
**Scope:** Replace custom ad-hoc API with a full Jellyfin-compatible REST server implemented in Rust/Axum

---

## Summary

The `dioxus_music` backend is being migrated from a set of Dioxus server functions (`#[server]` macros) and one custom Axum route to a fully Jellyfin-compatible REST API. The server implements Jellyfin's HTTP routes, URL conventions, and response shapes so that any Jellyfin-aware client could eventually connect to it. The Dioxus frontend switches from server function calls to direct HTTP requests against the Jellyfin-shaped endpoints.

Custom features that have no Jellyfin equivalent (smart playlists, library rescan, health check) live under a `/custom/` prefix, isolated from Jellyfin-spec routes.

---

## Approach

**Option chosen:** Implement Jellyfin's API shapes in the existing Rust/Axum backend. No separate Jellyfin server instance. Full control over the implementation, incrementally expandable toward third-party client compatibility.

Key decisions:
- **Data model:** Full normalization — `artists`, `albums`, `tracks`, `users`, `access_tokens`, `user_data`, `images`, `genres`, `playlists`, `playlist_items`
- **Auth:** Full Jellyfin-style user auth (`AuthenticateByName` → token → `Authorization` header on all routes)
- **Images:** Embedded cover art extracted via `lofty` + folder-level `cover.{jpg,jpeg,png,webp}` fallback; served from on-disk image cache
- **Smart playlists:** Stored as regular Jellyfin playlists with rules JSON in the `overview` field and an internal `is_smart` flag; rule management via `/custom/` routes
- **Transcoding:** Out of scope — server always streams original files; codec/bitrate params accepted but ignored
- **Database migration:** Clean slate — existing data is dropped; scanner re-populates on first startup

---

## Architecture

### Structural Change: Server Functions → REST Routes

Dioxus server functions compile to `POST /_dioxus/...` endpoints with a binary format — incompatible with Jellyfin's REST conventions. All server-side logic moves to pure Axum handlers. The frontend uses `gloo-net` (WASM) or `reqwest` (SSR) to call the Jellyfin-shaped endpoints.

### Crate Layout

```
dioxus_music/packages/
├── api/
│   ├── routes/         # One module per Jellyfin route group
│   │   ├── users.rs    # Auth, user management, API keys
│   │   ├── items.rs    # Universal /Items query endpoint
│   │   ├── artists.rs  # /Artists, similar, instant mix
│   │   ├── albums.rs   # /Albums similar + instant mix
│   │   ├── genres.rs   # /Genres, /MusicGenres, instant mix
│   │   ├── audio.rs    # Streaming, lyrics, playback info
│   │   ├── playlists.rs# Playlist CRUD + item management
│   │   ├── user_data.rs# Favorites, ratings, played state
│   │   ├── sessions.rs # Playback session reporting
│   │   ├── images.rs   # Image serving
│   │   ├── search.rs   # /Search/Hints
│   │   └── custom.rs   # /custom/* non-Jellyfin routes
│   ├── db/             # Diesel schema, models, query helpers
│   ├── auth/           # Token middleware, argon2 password hashing
│   ├── scanner/        # File scanner (normalized schema + image extraction)
│   └── images/         # Image extraction (lofty), folder fallback, cache management
├── ui/                 # Shared components + HTTP client layer (api_client module)
├── web/                # Fullstack web — mounts api router; Dioxus shell
├── desktop/            # Desktop entry point
└── mobile/             # Mobile entry point
```

---

## Database Schema

Clean-slate Diesel migration. No data preserved from the existing schema.

```sql
-- Users and authentication
CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,               -- argon2id
    is_admin    BOOLEAN NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE access_tokens (
    token       TEXT PRIMARY KEY,              -- random 32-byte hex, stored plaintext
    user_id     UUID NOT NULL REFERENCES users ON DELETE CASCADE,
    device_id   TEXT NOT NULL,
    device_name TEXT NOT NULL,
    client_name TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Library
CREATE TABLE artists (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    sort_name   TEXT NOT NULL,                 -- "The National" → "National, The"
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE albums (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title       TEXT NOT NULL,
    sort_title  TEXT NOT NULL,
    artist_id   UUID NOT NULL REFERENCES artists,
    year        INT4,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE tracks (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title        TEXT NOT NULL,
    sort_title   TEXT NOT NULL,
    artist_id    UUID NOT NULL REFERENCES artists,
    album_id     UUID REFERENCES albums,
    genre        TEXT NOT NULL DEFAULT '',
    duration_ticks INT8 NOT NULL DEFAULT 0,   -- 100-nanosecond ticks (Jellyfin convention)
    track_number INT4,
    disc_number  INT4 NOT NULL DEFAULT 1,
    file_path    TEXT NOT NULL UNIQUE,
    container    TEXT NOT NULL,               -- mp3 | flac | ogg | opus
    bit_rate     INT4,
    sample_rate  INT4,
    channels     INT4,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE genres (
    id   UUID PRIMARY KEY,                    -- deterministic UUIDv5 from genre name
    name TEXT NOT NULL UNIQUE
);

-- Images
CREATE TABLE images (
    item_id    UUID NOT NULL,                 -- polymorphic: artist | album | track
    image_type TEXT NOT NULL,                 -- Primary | Backdrop
    file_path  TEXT NOT NULL,                 -- absolute path in image cache dir
    tag        TEXT NOT NULL,                 -- SHA-256 of file content
    width      INT4,
    height     INT4,
    PRIMARY KEY (item_id, image_type)
);

-- Playlists
CREATE TABLE playlists (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    overview   TEXT,                          -- smart playlist rules JSON stored here
    is_smart   BOOLEAN NOT NULL DEFAULT false,
    user_id    UUID REFERENCES users,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE playlist_items (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- Jellyfin "entry id"
    playlist_id UUID NOT NULL REFERENCES playlists ON DELETE CASCADE,
    item_id     UUID NOT NULL REFERENCES tracks ON DELETE CASCADE,
    position    INT4 NOT NULL,
    UNIQUE (playlist_id, item_id)
);
CREATE INDEX ON playlist_items (playlist_id, position);

-- User data (polymorphic: tracks, albums, artists, playlists)
CREATE TABLE user_data (
    user_id                UUID NOT NULL REFERENCES users ON DELETE CASCADE,
    item_id                UUID NOT NULL,
    item_type              TEXT NOT NULL,     -- Audio | MusicAlbum | MusicArtist | Playlist
    is_favorite            BOOLEAN NOT NULL DEFAULT false,
    likes                  BOOLEAN,          -- null=unrated | true=like | false=dislike
    play_count             INT4 NOT NULL DEFAULT 0,
    last_played_date       TIMESTAMPTZ,
    played                 BOOLEAN NOT NULL DEFAULT false,
    playback_position_ticks INT8 NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, item_id)
);
```

**Duration convention:** `duration_ticks = duration_seconds * 10_000_000`. Jellyfin clients expect ticks; the scanner converts from `lofty`'s millisecond duration output.

---

## Authentication

### Flow

1. Client sends `POST /Users/AuthenticateByName` with `{ "Username": "...", "Pw": "..." }` and an `Authorization` header that includes `Client`, `Device`, `DeviceId`, `Version` but no `Token` yet.
2. Server verifies password (argon2id), creates a row in `access_tokens`, returns:
   ```json
   {
     "User": { "Id": "...", "Name": "...", "ServerId": "..." },
     "AccessToken": "<token>",
     "ServerId": "<server-uuid>"
   }
   ```
3. All subsequent requests include `Token=<token>` in the `Authorization` header. Axum middleware resolves the token, updates `last_seen_at`, injects the `User` into request extensions. Returns `401` if missing or invalid.
4. `DELETE /Sessions/Logout` revokes the current token.

### First-Run Bootstrap

On startup, if the `users` table is empty, the server creates a default admin user with credentials from env vars (`DIOXUS_MUSIC_ADMIN_USER` / `DIOXUS_MUSIC_ADMIN_PASSWORD`). This avoids needing a setup wizard.

### `ServerId`

`AuthenticationResult` includes a `ServerId` field. This is a stable UUID representing the server instance — generated once on first run and persisted via a `server_config` env var (`DIOXUS_MUSIC_SERVER_ID`) or written to a local state file. Clients use it to distinguish servers in multi-server setups.

### `userId` Parameter

Most Jellyfin routes accept an optional `userId` query param. If omitted, defaults to the authenticated user's ID. Admin users may pass a different `userId` to act on behalf of others.

### API Keys

`GET/POST /Auth/Keys` and `DELETE /Auth/Keys/{key}` — admin-only static API key management for scripting use cases.

---

## API Routes

### `routes/users.rs`
```
POST   /Users/AuthenticateByName
GET    /Users
GET    /Users/{userId}
POST   /Users/{userId}/Password
DELETE /Users/{userId}
DELETE /Sessions/Logout
GET    /Auth/Keys
POST   /Auth/Keys
DELETE /Auth/Keys/{key}
```

### `routes/items.rs`
```
GET    /Items                    # filters: includeItemTypes, parentId, genres, genreIds,
                                 #   isFavorite, searchTerm, sortBy, sortOrder,
                                 #   startIndex, limit, userId, fields
GET    /Items/{itemId}           # single item — Audio | MusicAlbum | MusicArtist
GET    /Items/Filters            # available filter values for current library
```
All three item types return `BaseItemDto`. The `type` field discriminates.

### `routes/artists.rs`
```
GET    /Artists
GET    /Artists/AlbumArtists
GET    /Artists/{name}
GET    /Artists/{itemId}/Similar
GET    /Artists/{itemId}/InstantMix
```

### `routes/albums.rs`
```
GET    /Albums/{itemId}/Similar
GET    /Albums/{itemId}/InstantMix
```

### `routes/genres.rs`
```
GET    /Genres
GET    /MusicGenres              # alias, same handler
GET    /MusicGenres/{genreName}/InstantMix
```

### `routes/audio.rs`
```
GET    /Audio/{itemId}/stream
GET    /Audio/{itemId}/stream.{container}
GET    /Audio/{itemId}/universal          # streams original file; transcoding future work
GET    /Audio/{itemId}/Lyrics             # 404 stub if no lyrics present
GET    /Items/{itemId}/PlaybackInfo
POST   /Items/{itemId}/PlaybackInfo
```
Transcoding is deferred. The server always streams the original file regardless of `container`/`audioCodec` params. `static=true` is implicitly always on.

### `routes/playlists.rs`
```
POST   /Playlists
GET    /Playlists/{playlistId}
POST   /Playlists/{playlistId}
DELETE /Playlists/{playlistId}
GET    /Playlists/{playlistId}/Items      # smart: evaluates genre rules; manual: joins playlist_items
POST   /Playlists/{playlistId}/Items      # 400 for smart playlists
DELETE /Playlists/{playlistId}/Items      # 400 for smart playlists
POST   /Playlists/{playlistId}/Items/{itemId}/Move/{newIndex}
GET    /Playlists/{itemId}/InstantMix
```

### `routes/user_data.rs`
```
POST   /UserFavoriteItems/{itemId}
DELETE /UserFavoriteItems/{itemId}
POST   /UserItems/{itemId}/Rating         # ?likes=true|false (null clears rating)
POST   /UserPlayedItems/{itemId}
DELETE /UserPlayedItems/{itemId}
```

### `routes/sessions.rs`
```
POST   /Sessions/Playing                  # store position in user_data
POST   /Sessions/Playing/Progress         # update position ticks
POST   /Sessions/Playing/Stopped          # finalize play_count + position
POST   /Sessions/Playing/Ping             # update last_seen_at on token
```

### `routes/images.rs`
```
GET    /Items/{itemId}/Images
GET    /Items/{itemId}/Images/{imageType}
GET    /Artists/{name}/Images/{imageType}/{index}
GET    /MusicGenres/{name}/Images/{imageType}/{index}
```
Image responses include `ETag` (content hash tag) and `Cache-Control: public, max-age=31536000, immutable`. `maxWidth`/`maxHeight` params are accepted but ignored (resize is future work). Genre images return 404 (no genre art in local library).

### `routes/search.rs`
```
GET    /Search/Hints                      # searchTerm, includeItemTypes, limit, userId
```
Searches across tracks (title, artist, album), artists (name), albums (title), and playlists (name).

### `routes/custom.rs` — non-Jellyfin extensions
```
POST   /custom/playlists/smart            # create smart playlist with typed rules body
PUT    /custom/playlists/{id}/rules       # update smart playlist rules
POST   /custom/library/rescan             # trigger full rescan
GET    /custom/library/version            # max updated_at timestamp (change detection)
GET    /custom/health                     # health check
```

---

## Scanner

### Metadata Extracted Per File

| Field | Source |
|---|---|
| Track artist | ID3 `TPE1` / Vorbis `ARTIST` |
| Album artist | ID3 `TPE2` / Vorbis `ALBUMARTIST` (fallback: track artist) |
| Album title | ID3 `TALB` / Vorbis `ALBUM` |
| Year | ID3 `TDRC` / Vorbis `DATE` |
| Track number | ID3 `TRCK` / Vorbis `TRACKNUMBER` |
| Disc number | ID3 `TPOS` / Vorbis `DISCNUMBER` |
| Bit rate | `lofty` `AudioProperties` |
| Sample rate | `lofty` `AudioProperties` |
| Channels | `lofty` `AudioProperties` |
| Embedded cover art | `lofty` picture tags |

### Artist and Album Deduplication

- **Artists:** matched by lowercase-trimmed name. If a row exists, reuse its UUID. `sort_name` strips leading articles ("The ", "A ", "An ") and appends them: `"The National"` → `"National, The"`.
- **Albums:** matched by `(normalized_title, artist_id)`. Same title under different artists gets separate rows. `year` is written from the first track scanned for that album.

### Image Extraction (Per Album)

After all tracks in a directory are scanned, if the album has no image row:
1. Extract embedded cover from the first track in the album that contains one (lofty picture tags)
2. If none found, look for `cover.{jpg,jpeg,png,webp}` in the album directory (case-insensitive glob)
3. Write to `$IMAGE_CACHE_DIR/{album_id}/Primary.{ext}`, compute SHA-256 tag, insert `images` row
4. Artist image: an artist's Primary image points to the same file as their first album's Primary image

**Image cache directory:** Configured via `IMAGE_CACHE_DIR` env var, defaults to `~/.local/share/dioxus_music/images/`.

### Genres Table

After each scan pass, `genres` is repopulated from `SELECT DISTINCT genre FROM tracks WHERE genre != ''`. Genre UUIDs are deterministic UUIDv5 derived from the genre name string so `user_data` references survive rescans.

### Scan Modes

- **`quick_scan`** (startup): walks music dir, skips files already present by `file_path`, handles deletions, resolves artist/album rows for new files only. Does not re-extract images for existing albums.
- **`full_scan`** (`POST /custom/library/rescan`): truncates `tracks`, `albums`, `artists`, `genres`, `images`; clears image cache dir; re-scans everything from scratch.

---

## Smart Playlists

Smart playlists are stored as regular Jellyfin playlists. The `is_smart` flag and `overview` JSON are server internals — any Jellyfin client sees an ordinary playlist.

**Rules format** (in `overview` column):
```json
{ "include_genres": ["Jazz", "Blues"], "exclude_genres": ["Holiday"] }
```

**`GET /Playlists/{id}/Items` evaluation:**
- `is_smart = false`: join `playlist_items → tracks`, return ordered by `position`
- `is_smart = true`: parse rules from `overview`, query `WHERE genre = ANY(include_genres) AND genre != ALL(exclude_genres)`, sort by artist/album/track_number

**Add/remove items:** Returns `400 Bad Request` for smart playlists.

**Custom rule management:**
- `POST /custom/playlists/smart` — typed `SmartPlaylistRules` body
- `PUT /custom/playlists/{id}/rules` — update rules in-place

The Dioxus UI's `PlaylistFormModal` continues to call the `/custom/` routes. Jellyfin clients see a regular playlist.

---

## Image Handling

**Track image fallback:** Tracks do not get their own `images` row. When a client requests `GET /Items/{trackId}/Images/Primary`, the handler resolves the track's `album_id` and serves the album's Primary image instead. If the album has no image, 404 is returned.

**Serving:** `GET /Items/{itemId}/Images/{imageType}` looks up the `images` row (with track fallback above), reads the file from disk, and returns the binary with:
- `Content-Type` inferred from file extension
- `ETag: "{tag}"` (SHA-256 content hash)
- `Cache-Control: public, max-age=31536000, immutable`

If the client sends a matching `If-None-Match` header, the server returns `304 Not Modified`.

**Resize:** `maxWidth`/`maxHeight` query params are accepted but ignored. Full resolution is always served. Resize support (via `image` crate) is deferred.

---

## Out of Scope (Future Work)

- Transcoding / format conversion
- HLS adaptive bitrate streaming
- Image resize
- Lyrics fetching from external providers
- Online metadata enrichment (MusicBrainz, Last.fm)
- Instant mix algorithm beyond simple genre matching
- Collaborative playlists
- Multi-user library isolation
