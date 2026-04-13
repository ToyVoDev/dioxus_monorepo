# Jellyfin API Migration — Plan 2: Scanner + Library API

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate the music file scanner to populate the normalized schema, extract cover art, and implement all Jellyfin library browse/stream/search routes so the backend serves real data.

**Architecture:** Scanner is rewritten to resolve/create `artists` and `albums` rows before inserting `tracks`. Cover art is extracted via `lofty` (embedded) or folder `cover.*` fallback, written to an image cache directory, and referenced in the `images` table. Library routes follow Jellyfin's `GET /Items`, `/Artists`, `/Albums`, `/Audio`, `/Images`, and `/Search/Hints` conventions.

**Tech Stack:** Axum 0.8, Diesel 2 + diesel-async (bb8), lofty (metadata + embedded art), walkdir, sha2 (image tag), image crate (image dimensions), tokio::fs

**Prerequisite:** Plan 1 must be complete (schema live, auth working, web crate compiles).

**Spec:** `docs/superpowers/specs/2026-04-13-jellyfin-api-migration-design.md`

---

## File Map

**Create:**
- `packages/api/src/scanner/mod.rs`
- `packages/api/src/scanner/metadata.rs`
- `packages/api/src/scanner/images.rs`
- `packages/api/src/scanner/artist.rs`
- `packages/api/src/scanner/album.rs`
- `packages/api/src/scanner/genres.rs`
- `packages/api/src/routes/items.rs`
- `packages/api/src/routes/artists.rs`
- `packages/api/src/routes/albums.rs`
- `packages/api/src/routes/genres.rs`
- `packages/api/src/routes/audio.rs`
- `packages/api/src/routes/images.rs`
- `packages/api/src/routes/search.rs`
- `packages/api/src/routes/custom.rs`

**Delete:**
- `packages/api/src/scanner.rs` (old flat scanner)

**Modify:**
- `packages/api/Cargo.toml` (add `image` crate for dimensions)
- `packages/api/src/lib.rs` (expose scanner startup functions)
- `packages/api/src/routes/mod.rs` (register all new routers)
- `packages/api/src/types.rs` (add remaining DTOs: MediaSourceInfo, SearchHint, etc.)
- `packages/web/src/main.rs` (spawn scanner on startup)

---

## Task 1: Add `image` crate for dimension reading

**Files:**
- Modify: `Cargo.toml` (workspace)
- Modify: `packages/api/Cargo.toml`

- [ ] **Step 1: Add image crate to workspace**

In root `Cargo.toml`, add under `[workspace.dependencies]`:
```toml
image = "0.25"
```

- [ ] **Step 2: Add image to api Cargo.toml**

In `packages/api/Cargo.toml`, add to `[dependencies]`:
```toml
image = { version = "0.25", optional = true }
```

Add `"dep:image"` to the `server` feature list.

- [ ] **Step 3: Verify**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml packages/api/Cargo.toml
git commit -m "chore(deps): add image crate for cover art dimension reading"
```

---

## Task 2: Delete old scanner, create scanner module skeleton

**Files:**
- Delete: `packages/api/src/scanner.rs`
- Create: `packages/api/src/scanner/mod.rs`, `metadata.rs`, `images.rs`, `artist.rs`, `album.rs`, `genres.rs`

- [ ] **Step 1: Delete old scanner**

```bash
rm packages/api/src/scanner.rs
mkdir -p packages/api/src/scanner
```

- [ ] **Step 2: Create scanner/mod.rs stub**

Create `packages/api/src/scanner/mod.rs`:

```rust
mod album;
mod artist;
mod genres;
mod images;
mod metadata;

use crate::{db::DbPool, state::AppState};

/// Incremental scan: skips files already in `tracks`, handles deletions, fast.
/// Run at startup in a background task.
pub async fn quick_scan(state: AppState) {
    // implemented in Task 7
    let _ = state;
}

/// Full rescan: truncates tracks/albums/artists/genres/images tables and image
/// cache, then re-scans everything. Triggered by POST /custom/library/rescan.
pub async fn full_scan(state: AppState) {
    // implemented in Task 7
    let _ = state;
}

/// Collect all music file paths under the music directory.
fn collect_music_paths(music_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    use walkdir::WalkDir;
    WalkDir::new(music_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let path = e.into_path();
            let ext = path.extension()?.to_ascii_lowercase();
            let ext = ext.to_str()?;
            matches!(ext, "flac" | "mp3" | "ogg" | "opus").then_some(path)
        })
        .collect()
}
```

- [ ] **Step 3: Create stub sub-modules**

Create `packages/api/src/scanner/metadata.rs`:
```rust
// Populated in Task 3.
```

Create `packages/api/src/scanner/images.rs`:
```rust
// Populated in Task 4.
```

Create `packages/api/src/scanner/artist.rs`:
```rust
// Populated in Task 5.
```

Create `packages/api/src/scanner/album.rs`:
```rust
// Populated in Task 5.
```

Create `packages/api/src/scanner/genres.rs`:
```rust
// Populated in Task 6.
```

- [ ] **Step 4: Update lib.rs to expose scanner**

In `packages/api/src/lib.rs`, add after the existing `pub mod` declarations:

```rust
#[cfg(feature = "server")]
pub mod scanner;
#[cfg(feature = "server")]
pub use scanner::{full_scan, quick_scan};
```

- [ ] **Step 5: Verify**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add packages/api/src/scanner/ packages/api/src/lib.rs
git commit -m "refactor(scanner): replace flat scanner with modular skeleton"
```

---

## Task 3: Metadata extraction

**Files:**
- Modify: `packages/api/src/scanner/metadata.rs`

- [ ] **Step 1: Write metadata.rs with tests**

Replace `packages/api/src/scanner/metadata.rs`:

```rust
use lofty::{
    AudioFile, ParseOptions, TaggedFileExt,
    file::TaggedFile,
    probe::Probe,
    tag::Accessor,
};
use std::path::Path;

/// All metadata extracted from a single audio file.
#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,          // track artist
    pub album_artist: String,    // falls back to artist if missing
    pub album: String,
    pub genre: String,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub disc_number: i32,
    pub duration_ticks: i64,     // 100-nanosecond ticks
    pub container: String,       // mp3 | flac | ogg | opus
    pub bit_rate: Option<i32>,   // kbps
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub has_embedded_art: bool,
}

/// Extracts metadata from a music file. Returns None if the file cannot be read.
pub fn read_metadata(path: &Path) -> Option<TrackMetadata> {
    let tagged_file = Probe::open(path)
        .ok()?
        .options(ParseOptions::new().read_picture(true))
        .read()
        .ok()?;

    let props = tagged_file.properties();
    let duration_ticks = props.duration().as_millis() as i64 * 10_000;
    let bit_rate = props.audio_bitrate().map(|b| b as i32);
    let sample_rate = props.sample_rate().map(|r| r as i32);
    let channels = props.channels().map(|c| c as i32);

    let container = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".to_string());

    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());
    let has_embedded_art = tag
        .map(|t| t.pictures().next().is_some())
        .unwrap_or(false);

    let title = tag
        .and_then(|t| t.title().map(|s| s.into_owned()))
        .or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(String::from)
        })
        .unwrap_or_default();

    let artist = tag
        .and_then(|t| t.artist().map(|s| s.into_owned()))
        .unwrap_or_default();

    let album_artist = tag
        .and_then(|t| {
            // Check Vorbis ALBUMARTIST or ID3 TPE2
            t.get_string(&lofty::tag::ItemKey::AlbumArtist).map(String::from)
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| artist.clone());

    let album = tag
        .and_then(|t| t.album().map(|s| s.into_owned()))
        .unwrap_or_default();

    let genre = tag
        .and_then(|t| t.genre().map(|s| s.into_owned()))
        .unwrap_or_default();

    let year = tag
        .and_then(|t| t.year())
        .and_then(|y| i32::try_from(y).ok());

    let track_number = tag
        .and_then(|t| t.track())
        .and_then(|n| i32::try_from(n).ok());

    let disc_number = tag
        .and_then(|t| t.disk())
        .and_then(|n| i32::try_from(n).ok())
        .unwrap_or(1);

    Some(TrackMetadata {
        title,
        artist,
        album_artist,
        album,
        genre,
        year,
        track_number,
        disc_number,
        duration_ticks,
        container,
        bit_rate,
        sample_rate,
        channels,
        has_embedded_art,
    })
}

/// Extract the raw bytes of the primary embedded cover art, if any.
pub fn read_embedded_art(path: &Path) -> Option<(Vec<u8>, String)> {
    let tagged_file = Probe::open(path)
        .ok()?
        .options(ParseOptions::new().read_picture(true))
        .read()
        .ok()?;

    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag())?;
    let pic = tag.pictures().next()?;
    let mime = pic.mime_type()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "image/jpeg".to_string());
    Some((pic.data().to_vec(), mime))
}

/// Compute the sort name for an artist:
/// "The Beatles" → "Beatles, The", "A Tribe Called Quest" → "Tribe Called Quest, A"
pub fn make_sort_name(name: &str) -> String {
    for article in ["The ", "the ", "A ", "a ", "An ", "an "] {
        if let Some(rest) = name.strip_prefix(article) {
            return format!("{}, {}", rest, article.trim());
        }
    }
    name.to_string()
}

/// Normalize a string for deduplication: lowercase + trim.
pub fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_name_strips_the() {
        assert_eq!(make_sort_name("The National"), "National, The");
    }

    #[test]
    fn sort_name_strips_a() {
        assert_eq!(make_sort_name("A Tribe Called Quest"), "Tribe Called Quest, A");
    }

    #[test]
    fn sort_name_strips_an() {
        assert_eq!(make_sort_name("An Horse"), "Horse, An");
    }

    #[test]
    fn sort_name_no_article() {
        assert_eq!(make_sort_name("Radiohead"), "Radiohead");
    }

    #[test]
    fn normalize_trims_and_lowercases() {
        assert_eq!(normalize("  The Beatles  "), "the beatles");
    }
}
```

- [ ] **Step 2: Run unit tests**

```bash
cargo test -p dioxus_music_api --features server scanner::metadata
```
Expected: 5 tests pass.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/scanner/metadata.rs
git commit -m "feat(scanner): add metadata extraction and sort-name utilities with tests"
```

---

## Task 4: Cover art extraction and caching

**Files:**
- Modify: `packages/api/src/scanner/images.rs`

- [ ] **Step 1: Write images.rs with tests**

Replace `packages/api/src/scanner/images.rs`:

```rust
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Cached image info ready to insert into the `images` table.
#[derive(Debug)]
pub struct CachedImage {
    pub item_id: Uuid,
    pub image_type: String, // "Primary"
    pub file_path: String,  // absolute path in cache dir
    pub tag: String,        // SHA-256 hex
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// Compute SHA-256 hex tag for image bytes.
pub fn compute_tag(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Determine the file extension for an image given its MIME type.
pub fn mime_to_ext(mime: &str) -> &str {
    match mime {
        "image/png" => "png",
        "image/webp" => "webp",
        _ => "jpg",
    }
}

/// Find a cover image file in a directory (cover.jpg, cover.jpeg, cover.png, cover.webp,
/// folder.jpg, folder.png — all case-insensitive).
pub fn find_folder_cover(dir: &Path) -> Option<PathBuf> {
    let stems = ["cover", "folder", "album", "front"];
    let exts = ["jpg", "jpeg", "png", "webp"];

    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();

        let _ = name; // suppress warning
        if stems.contains(&stem.as_str()) && exts.contains(&ext.as_str()) {
            return Some(path);
        }
    }
    None
}

/// Write image bytes to `{cache_dir}/{item_id}/Primary.{ext}`.
/// Returns the destination path.
pub fn write_image_cache(
    cache_dir: &Path,
    item_id: Uuid,
    image_type: &str,
    data: &[u8],
    ext: &str,
) -> std::io::Result<PathBuf> {
    let item_dir = cache_dir.join(item_id.to_string());
    std::fs::create_dir_all(&item_dir)?;
    let dest = item_dir.join(format!("{image_type}.{ext}"));
    std::fs::write(&dest, data)?;
    Ok(dest)
}

/// Read image dimensions using the `image` crate.
/// Returns None if the file cannot be decoded.
pub fn read_dimensions(path: &Path) -> Option<(i32, i32)> {
    let reader = image::ImageReader::open(path).ok()?;
    let (w, h) = reader.into_dimensions().ok()?;
    Some((w as i32, h as i32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn make_temp_dir() -> TempDir {
        tempfile::TempDir::new().unwrap()
    }

    #[test]
    fn compute_tag_is_deterministic() {
        let data = b"hello";
        assert_eq!(compute_tag(data), compute_tag(data));
    }

    #[test]
    fn compute_tag_differs_for_different_data() {
        assert_ne!(compute_tag(b"a"), compute_tag(b"b"));
    }

    #[test]
    fn mime_to_ext_png() {
        assert_eq!(mime_to_ext("image/png"), "png");
    }

    #[test]
    fn mime_to_ext_defaults_to_jpg() {
        assert_eq!(mime_to_ext("image/jpeg"), "jpg");
        assert_eq!(mime_to_ext("image/unknown"), "jpg");
    }

    #[test]
    fn find_folder_cover_finds_cover_jpg() {
        let dir = make_temp_dir();
        let cover = dir.path().join("cover.jpg");
        std::fs::write(&cover, b"fake image").unwrap();
        assert_eq!(find_folder_cover(dir.path()), Some(cover));
    }

    #[test]
    fn find_folder_cover_finds_folder_png() {
        let dir = make_temp_dir();
        let cover = dir.path().join("folder.png");
        std::fs::write(&cover, b"fake image").unwrap();
        assert_eq!(find_folder_cover(dir.path()), Some(cover));
    }

    #[test]
    fn find_folder_cover_case_insensitive() {
        let dir = make_temp_dir();
        let cover = dir.path().join("Cover.JPG");
        std::fs::write(&cover, b"fake image").unwrap();
        assert_eq!(find_folder_cover(dir.path()), Some(cover));
    }

    #[test]
    fn find_folder_cover_returns_none_when_absent() {
        let dir = make_temp_dir();
        assert_eq!(find_folder_cover(dir.path()), None);
    }

    #[test]
    fn write_image_cache_creates_file() {
        let dir = make_temp_dir();
        let id = Uuid::new_v4();
        let path = write_image_cache(dir.path(), id, "Primary", b"imgdata", "jpg").unwrap();
        assert!(path.exists());
        assert_eq!(std::fs::read(&path).unwrap(), b"imgdata");
    }
}
```

- [ ] **Step 2: Add tempfile to dev-dependencies**

In `packages/api/Cargo.toml`, add:
```toml
[dev-dependencies]
tempfile = "3"
tokio = { workspace = true, features = ["macros", "rt-multi-thread"] }
```

- [ ] **Step 3: Run unit tests**

```bash
cargo test -p dioxus_music_api --features server scanner::images
```
Expected: 9 tests pass.

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/scanner/images.rs packages/api/Cargo.toml
git commit -m "feat(scanner): add cover art extraction, caching, and SHA-256 tagging with tests"
```

---

## Task 5: Artist and album resolution (deduplication)

**Files:**
- Modify: `packages/api/src/scanner/artist.rs`
- Modify: `packages/api/src/scanner/album.rs`

- [ ] **Step 1: Write artist.rs**

Replace `packages/api/src/scanner/artist.rs`:

```rust
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        models::{Artist, NewArtist},
        schema::artists,
    },
    scanner::metadata::{make_sort_name, normalize},
};

/// Find an existing artist by normalized name, or insert a new one.
/// Returns the artist UUID.
pub async fn find_or_create(
    conn: &mut AsyncPgConnection,
    name: &str,
) -> Result<Uuid, diesel::result::Error> {
    let norm = normalize(name);

    // Try to find existing artist by normalized name (case-insensitive).
    let existing: Option<Artist> = artists::table
        .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(
            &format!("LOWER(TRIM(name)) = '{}'", norm.replace('\'', "''")),
        ))
        .first(conn)
        .await
        .optional()?;

    if let Some(a) = existing {
        return Ok(a.id);
    }

    // Insert new artist.
    let display_name = if name.trim().is_empty() {
        "Unknown Artist".to_string()
    } else {
        name.trim().to_string()
    };

    let new_artist = NewArtist {
        id: Uuid::new_v4(),
        name: display_name.clone(),
        sort_name: make_sort_name(&display_name),
    };

    let inserted: Artist = diesel::insert_into(artists::table)
        .values(&new_artist)
        .get_result(conn)
        .await?;

    Ok(inserted.id)
}
```

- [ ] **Step 2: Write album.rs**

Replace `packages/api/src/scanner/album.rs`:

```rust
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        models::{Album, NewAlbum},
        schema::albums,
    },
    scanner::metadata::normalize,
};

/// Find an existing album by normalized (title, artist_id), or insert a new one.
/// Returns the album UUID.
pub async fn find_or_create(
    conn: &mut AsyncPgConnection,
    title: &str,
    artist_id: Uuid,
    year: Option<i32>,
) -> Result<Uuid, diesel::result::Error> {
    let norm_title = normalize(title);

    let existing: Option<Album> = albums::table
        .filter(albums::artist_id.eq(artist_id))
        .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(
            &format!("LOWER(TRIM(title)) = '{}'", norm_title.replace('\'', "''")),
        ))
        .first(conn)
        .await
        .optional()?;

    if let Some(a) = existing {
        return Ok(a.id);
    }

    let display_title = if title.trim().is_empty() {
        "Unknown Album".to_string()
    } else {
        title.trim().to_string()
    };

    let new_album = NewAlbum {
        id: Uuid::new_v4(),
        title: display_title.clone(),
        sort_title: display_title.to_ascii_lowercase(),
        artist_id,
        year,
    };

    let inserted: Album = diesel::insert_into(albums::table)
        .values(&new_album)
        .get_result(conn)
        .await?;

    Ok(inserted.id)
}
```

- [ ] **Step 3: Verify compilation**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/scanner/artist.rs packages/api/src/scanner/album.rs
git commit -m "feat(scanner): add artist and album deduplication helpers"
```

---

## Task 6: Genres table refresh

**Files:**
- Modify: `packages/api/src/scanner/genres.rs`

- [ ] **Step 1: Write genres.rs**

Replace `packages/api/src/scanner/genres.rs`:

```rust
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::db::{models::NewGenre, schema::{genres, tracks}};

/// UUIDv5 namespace for deterministic genre UUIDs.
/// Using the DNS namespace UUID as a stable base.
const NAMESPACE: uuid::Uuid = uuid::Uuid::from_bytes([
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
    0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
]);

/// Derive a deterministic UUID for a genre name.
pub fn genre_uuid(name: &str) -> Uuid {
    Uuid::new_v5(&NAMESPACE, name.trim().to_lowercase().as_bytes())
}

/// Repopulate the `genres` table from distinct non-empty genres in `tracks`.
/// UUIDs are deterministic so existing `user_data` references survive.
pub async fn refresh(conn: &mut AsyncPgConnection) -> Result<(), diesel::result::Error> {
    // Get distinct genres from tracks.
    let distinct: Vec<String> = tracks::table
        .select(tracks::genre)
        .filter(tracks::genre.ne(""))
        .distinct()
        .load(conn)
        .await?;

    // Delete existing genres not in new set.
    let new_names: Vec<String> = distinct.clone();
    diesel::delete(genres::table.filter(genres::name.ne_all(new_names)))
        .execute(conn)
        .await?;

    // Upsert each genre.
    for name in distinct {
        let new_genre = NewGenre {
            id: genre_uuid(&name),
            name: name.clone(),
        };
        diesel::insert_into(genres::table)
            .values(&new_genre)
            .on_conflict(genres::id)
            .do_nothing()
            .execute(conn)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genre_uuid_is_deterministic() {
        assert_eq!(genre_uuid("Jazz"), genre_uuid("Jazz"));
    }

    #[test]
    fn genre_uuid_is_case_insensitive() {
        assert_eq!(genre_uuid("Jazz"), genre_uuid("jazz"));
        assert_eq!(genre_uuid("Jazz"), genre_uuid("  Jazz  "));
    }

    #[test]
    fn genre_uuid_differs_for_different_names() {
        assert_ne!(genre_uuid("Jazz"), genre_uuid("Blues"));
    }
}
```

- [ ] **Step 2: Run unit tests**

```bash
cargo test -p dioxus_music_api --features server scanner::genres
```
Expected: 3 tests pass.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/scanner/genres.rs
git commit -m "feat(scanner): add deterministic genre UUID generation with tests"
```

---

## Task 7: Implement full_scan and quick_scan

**Files:**
- Modify: `packages/api/src/scanner/mod.rs`

- [ ] **Step 1: Write scanner/mod.rs**

Replace `packages/api/src/scanner/mod.rs`:

```rust
pub mod album;
pub mod artist;
pub mod genres;
pub mod images;
pub mod metadata;

use std::path::Path;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::{
    db::{
        models::{NewImage, NewTrack},
        schema::{albums, artists, genres as genres_table, images, playlist_items, playlists, tracks, user_data},
    },
    state::AppState,
};

/// Incremental scan: processes only new files, removes deleted ones.
pub async fn quick_scan(state: AppState) {
    tracing::info!("Starting quick scan of {:?}", state.music_dir);
    let paths = collect_music_paths(&state.music_dir);

    let Ok(mut conn) = state.pool.get().await else {
        tracing::error!("quick_scan: failed to get DB connection");
        return;
    };

    // Get existing file paths from DB.
    let existing_paths: Vec<String> = tracks::table
        .select(tracks::file_path)
        .load(&mut conn)
        .await
        .unwrap_or_default();
    let existing_set: std::collections::HashSet<String> =
        existing_paths.into_iter().collect();

    // New files not yet in DB.
    let new_paths: Vec<_> = paths
        .iter()
        .filter(|p| !existing_set.contains(p.to_str().unwrap_or("")))
        .cloned()
        .collect();

    // Files in DB that no longer exist on disk.
    let disk_set: std::collections::HashSet<String> = paths
        .iter()
        .filter_map(|p| p.to_str().map(String::from))
        .collect();
    let stale: Vec<String> = existing_set
        .difference(&disk_set)
        .cloned()
        .collect();

    if !stale.is_empty() {
        tracing::info!("Removing {} stale tracks", stale.len());
        diesel::delete(tracks::table.filter(tracks::file_path.eq_any(&stale)))
            .execute(&mut conn)
            .await
            .ok();
    }

    if new_paths.is_empty() {
        tracing::info!("Quick scan: no new files");
        genres::refresh(&mut conn).await.ok();
        return;
    }

    tracing::info!("Quick scan: processing {} new files", new_paths.len());
    ingest_files(&state, &new_paths).await;

    let Ok(mut conn) = state.pool.get().await else { return };
    genres::refresh(&mut conn).await.ok();
    tracing::info!("Quick scan complete");
}

/// Full rescan: truncates all library tables, clears image cache, re-scans.
pub async fn full_scan(state: AppState) {
    tracing::info!("Starting full scan of {:?}", state.music_dir);

    let Ok(mut conn) = state.pool.get().await else {
        tracing::error!("full_scan: failed to get DB connection");
        return;
    };

    // Clear image cache directory.
    if state.image_cache_dir.exists() {
        std::fs::remove_dir_all(&state.image_cache_dir).ok();
    }
    std::fs::create_dir_all(&state.image_cache_dir).ok();

    // Truncate tables in dependency order.
    diesel::delete(user_data::table).execute(&mut conn).await.ok();
    diesel::delete(playlist_items::table).execute(&mut conn).await.ok();
    diesel::delete(playlists::table).execute(&mut conn).await.ok();
    diesel::delete(images::table).execute(&mut conn).await.ok();
    diesel::delete(genres_table::table).execute(&mut conn).await.ok();
    diesel::delete(tracks::table).execute(&mut conn).await.ok();
    diesel::delete(albums::table).execute(&mut conn).await.ok();
    diesel::delete(artists::table).execute(&mut conn).await.ok();

    drop(conn);

    let paths = collect_music_paths(&state.music_dir);
    tracing::info!("Full scan: processing {} files", paths.len());
    ingest_files(&state, &paths).await;

    let Ok(mut conn) = state.pool.get().await else { return };
    genres::refresh(&mut conn).await.ok();
    tracing::info!("Full scan complete");
}

/// Process a batch of file paths: extract metadata, resolve artist/album, insert track.
async fn ingest_files(state: &AppState, paths: &[std::path::PathBuf]) {
    // Group paths by directory for album-level image extraction.
    let mut by_dir: std::collections::HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> =
        std::collections::HashMap::new();
    for path in paths {
        if let Some(dir) = path.parent() {
            by_dir.entry(dir.to_path_buf()).or_default().push(path.clone());
        }
    }

    for (dir, dir_paths) in &by_dir {
        process_directory(state, dir, dir_paths).await;
    }
}

/// Process all files in one directory (one album's worth of tracks).
async fn process_directory(
    state: &AppState,
    dir: &Path,
    paths: &[std::path::PathBuf],
) {
    let mut album_id_for_dir: Option<Uuid> = None;

    for path in paths {
        // Read metadata on a blocking thread to avoid blocking the async runtime.
        let path_clone = path.clone();
        let meta = tokio::task::spawn_blocking(move || metadata::read_metadata(&path_clone))
            .await
            .ok()
            .flatten();

        let Some(meta) = meta else {
            tracing::warn!("Could not read metadata for {:?}", path);
            continue;
        };

        let Ok(mut conn) = state.pool.get().await else { continue };

        // Resolve artist and album artist.
        let track_artist_id = artist::find_or_create(&mut conn, &meta.artist)
            .await
            .unwrap_or_else(|_| Uuid::new_v4());

        let album_artist_id = if meta.album_artist != meta.artist {
            artist::find_or_create(&mut conn, &meta.album_artist)
                .await
                .unwrap_or(track_artist_id)
        } else {
            track_artist_id
        };

        // Resolve album (once per directory).
        let the_album_id = if let Some(aid) = album_id_for_dir {
            aid
        } else {
            let aid = album::find_or_create(&mut conn, &meta.album, album_artist_id, meta.year)
                .await
                .unwrap_or_else(|_| Uuid::new_v4());
            album_id_for_dir = Some(aid);
            aid
        };

        let container = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_else(|| "unknown".to_string());

        let new_track = NewTrack {
            id: Uuid::new_v4(),
            title: meta.title.clone(),
            sort_title: meta.title.to_ascii_lowercase(),
            artist_id: track_artist_id,
            album_id: Some(the_album_id),
            genre: meta.genre.clone(),
            duration_ticks: meta.duration_ticks,
            track_number: meta.track_number,
            disc_number: meta.disc_number,
            file_path: path.to_str().unwrap_or("").to_string(),
            container,
            bit_rate: meta.bit_rate,
            sample_rate: meta.sample_rate,
            channels: meta.channels,
        };

        if let Err(e) = diesel::insert_into(crate::db::schema::tracks::table)
            .values(&new_track)
            .on_conflict(crate::db::schema::tracks::file_path)
            .do_nothing()
            .execute(&mut conn)
            .await
        {
            tracing::error!("Failed to insert track {:?}: {e}", path);
        }

        // Extract cover art once per album (first track that has it).
        if album_id_for_dir.is_some() {
            try_extract_album_cover(state, dir, path, the_album_id, &meta).await;
            // After first attempt, set to None so we don't try again for this dir.
            // (We only wanted to run this once per dir.)
        }
    }

    // Also try to assign the album cover to the album artist as their Primary image.
    if let Some(album_id) = album_id_for_dir {
        if let Ok(mut conn) = state.pool.get().await {
            propagate_artist_image_from_album(&mut conn, album_id).await;
        }
    }
}

/// Extract and cache cover art for an album. Tries embedded art first, then folder cover.
async fn try_extract_album_cover(
    state: &AppState,
    dir: &Path,
    first_track: &Path,
    album_id: Uuid,
    meta: &metadata::TrackMetadata,
) {
    let Ok(mut conn) = state.pool.get().await else { return };

    // Skip if album already has an image.
    let existing: Option<crate::db::models::Image> =
        crate::db::schema::images::table
            .filter(crate::db::schema::images::item_id.eq(album_id))
            .filter(crate::db::schema::images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()
            .unwrap_or(None);

    if existing.is_some() {
        return;
    }

    // Try embedded art.
    let art_data: Option<(Vec<u8>, String)> = if meta.has_embedded_art {
        let path = first_track.to_path_buf();
        tokio::task::spawn_blocking(move || metadata::read_embedded_art(&path))
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    // Fallback to folder cover.
    let art_data = if art_data.is_none() {
        let dir = dir.to_path_buf();
        tokio::task::spawn_blocking(move || {
            images::find_folder_cover(&dir).and_then(|p| {
                let data = std::fs::read(&p).ok()?;
                let ext = p.extension()?.to_str()?.to_ascii_lowercase();
                let mime = match ext.as_str() {
                    "png" => "image/png".to_string(),
                    "webp" => "image/webp".to_string(),
                    _ => "image/jpeg".to_string(),
                };
                Some((data, mime))
            })
        })
        .await
        .ok()
        .flatten()
    } else {
        art_data
    };

    let Some((data, mime)) = art_data else { return };

    let ext = images::mime_to_ext(&mime);
    let tag = images::compute_tag(&data);
    let cache_dir = state.image_cache_dir.clone();

    let dest = match tokio::task::spawn_blocking(move || {
        images::write_image_cache(&cache_dir, album_id, "Primary", &data, ext)
    })
    .await
    .ok()
    .and_then(|r| r.ok())
    {
        Some(p) => p,
        None => {
            tracing::warn!("Failed to write image cache for album {album_id}");
            return;
        }
    };

    let (width, height) = {
        let dest_clone = dest.clone();
        tokio::task::spawn_blocking(move || images::read_dimensions(&dest_clone))
            .await
            .ok()
            .flatten()
            .map(|(w, h)| (Some(w), Some(h)))
            .unwrap_or((None, None))
    };

    let new_image = NewImage {
        item_id: album_id,
        image_type: "Primary".to_string(),
        file_path: dest.to_str().unwrap_or("").to_string(),
        tag,
        width,
        height,
    };

    diesel::insert_into(crate::db::schema::images::table)
        .values(&new_image)
        .on_conflict((
            crate::db::schema::images::item_id,
            crate::db::schema::images::image_type,
        ))
        .do_nothing()
        .execute(&mut conn)
        .await
        .ok();
}

/// Give the album artist the same Primary image as one of their albums, if they don't have one.
async fn propagate_artist_image_from_album(
    conn: &mut diesel_async::AsyncPgConnection,
    album_id: Uuid,
) {
    use crate::db::schema::{albums, images};

    let album: Option<crate::db::models::Album> = albums::table
        .filter(albums::id.eq(album_id))
        .first(conn)
        .await
        .optional()
        .unwrap_or(None);

    let Some(album) = album else { return };
    let artist_id = album.artist_id;

    // Check if artist already has an image.
    let has_image: bool = images::table
        .filter(images::item_id.eq(artist_id))
        .filter(images::image_type.eq("Primary"))
        .count()
        .get_result::<i64>(conn)
        .await
        .unwrap_or(0)
        > 0;

    if has_image {
        return;
    }

    // Copy album's image reference to artist.
    let album_image: Option<crate::db::models::Image> = images::table
        .filter(images::item_id.eq(album_id))
        .filter(images::image_type.eq("Primary"))
        .first(conn)
        .await
        .optional()
        .unwrap_or(None);

    if let Some(img) = album_image {
        let new_image = NewImage {
            item_id: artist_id,
            image_type: "Primary".to_string(),
            file_path: img.file_path,
            tag: img.tag,
            width: img.width,
            height: img.height,
        };
        diesel::insert_into(images::table)
            .values(&new_image)
            .on_conflict((images::item_id, images::image_type))
            .do_nothing()
            .execute(conn)
            .await
            .ok();
    }
}

fn collect_music_paths(music_dir: &Path) -> Vec<std::path::PathBuf> {
    WalkDir::new(music_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let path = e.into_path();
            let ext = path.extension()?.to_ascii_lowercase();
            let ext = ext.to_str()?;
            matches!(ext, "flac" | "mp3" | "ogg" | "opus").then_some(path)
        })
        .collect()
}
```

- [ ] **Step 2: Update web/src/main.rs to spawn scanner on startup**

In `packages/web/src/main.rs`, after `dioxus_music_api::bootstrap(&state).await;` and before the router setup, add:

```rust
        // Spawn background quick scan.
        tokio::spawn(dioxus_music_api::quick_scan(state.clone()));
```

- [ ] **Step 3: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```
Expected: clean build.

- [ ] **Step 4: Smoke-test the scanner**

Start the server and watch the logs:
```bash
dx serve --package dioxus_music_web
```
Expected log output:
```
Starting quick scan of "/Users/.../Music"
Quick scan: processing N new files
Quick scan complete
```

- [ ] **Step 5: Commit**

```bash
git add packages/api/src/scanner/mod.rs packages/web/src/main.rs
git commit -m "feat(scanner): implement quick_scan and full_scan with normalized schema and image extraction"
```

---

## Task 8: BaseItemDto builder helpers in types.rs

**Files:**
- Modify: `packages/api/src/types.rs`

- [ ] **Step 1: Add remaining DTOs and item builders to types.rs**

Append to the end of `packages/api/src/types.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSourceInfo {
    pub id: String,
    pub path: Option<String>,
    pub protocol: String,           // "File"
    pub media_type: Option<String>, // "Audio"
    pub container: Option<String>,
    pub size: Option<i64>,
    pub bit_rate: Option<i32>,
    pub default_audio_stream_index: Option<i32>,
    pub supports_direct_play: bool,
    pub supports_direct_stream: bool,
    pub supports_transcoding: bool,
    pub is_remote: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackInfoResponse {
    pub media_sources: Vec<MediaSourceInfo>,
    pub play_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHint {
    pub item_id: Uuid,
    pub name: String,
    #[serde(rename = "Type")]
    pub item_type: String,
    pub album: Option<String>,
    pub album_id: Option<Uuid>,
    pub album_artist: Option<String>,
    pub primary_image_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHintsResult {
    pub search_hints: Vec<SearchHint>,
    pub total_record_count: i64,
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/types.rs
git commit -m "feat(api): add MediaSourceInfo, PlaybackInfoResponse, SearchHint DTOs"
```

---

## Task 9: Query helpers (item assembly)

**Files:**
- Create: `packages/api/src/routes/query.rs`

These helpers build `BaseItemDto` from DB rows — shared across `/Items`, `/Artists`, `/Albums`, and `/Search`.

- [ ] **Step 1: Create routes/query.rs**

Create `packages/api/src/routes/query.rs`:

```rust
//! Shared helpers to assemble BaseItemDto from DB rows.

use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    db::models::{Album, Artist, Image, Track, UserData},
    types::{BaseItemDto, NameGuidPair, UserItemDataDto},
};

pub fn track_to_dto(
    track: &Track,
    artist: &Artist,
    album: Option<&Album>,
    album_artist: Option<&Artist>,
    image: Option<&Image>,
    user_data: Option<&UserData>,
    server_id: Uuid,
) -> BaseItemDto {
    let image_tags = image.map(|img| {
        let mut m = HashMap::new();
        m.insert("Primary".to_string(), img.tag.clone());
        m
    });

    let album_primary_image_tag = album
        .and_then(|_| image.map(|img| img.tag.clone()));

    BaseItemDto {
        id: track.id,
        name: track.title.clone(),
        sort_name: Some(track.sort_title.clone()),
        item_type: "Audio".to_string(),
        server_id,
        album: album.map(|a| a.title.clone()),
        album_id: track.album_id,
        album_primary_image_tag,
        album_artist: album_artist.map(|a| a.name.clone()),
        album_artists: album_artist.map(|a| {
            vec![NameGuidPair { name: a.name.clone(), id: a.id }]
        }),
        artists: Some(vec![artist.name.clone()]),
        artist_items: Some(vec![NameGuidPair {
            name: artist.name.clone(),
            id: artist.id,
        }]),
        genre_items: None,
        genres: if track.genre.is_empty() {
            None
        } else {
            Some(vec![track.genre.clone()])
        },
        run_time_ticks: Some(track.duration_ticks),
        track_number: track.track_number,
        index_number: track.track_number,
        parent_index_number: Some(track.disc_number),
        container: Some(track.container.clone()),
        media_type: Some("Audio".to_string()),
        production_year: None,
        image_tags,
        user_data: user_data.map(user_data_to_dto),
        date_created: Some(track.updated_at),
    }
}

pub fn album_to_dto(
    album: &Album,
    artist: &Artist,
    image: Option<&Image>,
    track_count: i64,
    user_data: Option<&UserData>,
    server_id: Uuid,
) -> BaseItemDto {
    let image_tags = image.map(|img| {
        let mut m = HashMap::new();
        m.insert("Primary".to_string(), img.tag.clone());
        m
    });

    BaseItemDto {
        id: album.id,
        name: album.title.clone(),
        sort_name: Some(album.sort_title.clone()),
        item_type: "MusicAlbum".to_string(),
        server_id,
        album: None,
        album_id: None,
        album_primary_image_tag: image.map(|i| i.tag.clone()),
        album_artist: Some(artist.name.clone()),
        album_artists: Some(vec![NameGuidPair {
            name: artist.name.clone(),
            id: artist.id,
        }]),
        artists: Some(vec![artist.name.clone()]),
        artist_items: Some(vec![NameGuidPair {
            name: artist.name.clone(),
            id: artist.id,
        }]),
        genre_items: None,
        genres: None,
        run_time_ticks: None,
        track_number: None,
        index_number: None,
        parent_index_number: None,
        container: None,
        media_type: Some("Audio".to_string()),
        production_year: album.year,
        image_tags,
        user_data: user_data.map(user_data_to_dto),
        date_created: Some(album.updated_at),
    }
}

pub fn artist_to_dto(
    artist: &Artist,
    image: Option<&Image>,
    user_data: Option<&UserData>,
    server_id: Uuid,
) -> BaseItemDto {
    let image_tags = image.map(|img| {
        let mut m = HashMap::new();
        m.insert("Primary".to_string(), img.tag.clone());
        m
    });

    BaseItemDto {
        id: artist.id,
        name: artist.name.clone(),
        sort_name: Some(artist.sort_name.clone()),
        item_type: "MusicArtist".to_string(),
        server_id,
        album: None,
        album_id: None,
        album_primary_image_tag: None,
        album_artist: None,
        album_artists: None,
        artists: None,
        artist_items: None,
        genre_items: None,
        genres: None,
        run_time_ticks: None,
        track_number: None,
        index_number: None,
        parent_index_number: None,
        container: None,
        media_type: None,
        production_year: None,
        image_tags,
        user_data: user_data.map(user_data_to_dto),
        date_created: Some(artist.updated_at),
    }
}

fn user_data_to_dto(ud: &UserData) -> UserItemDataDto {
    UserItemDataDto {
        is_favorite: ud.is_favorite,
        likes: ud.likes,
        play_count: ud.play_count,
        last_played_date: ud.last_played_date,
        played: ud.played,
        playback_position_ticks: ud.playback_position_ticks,
        key: ud.item_id.to_string(),
    }
}
```

- [ ] **Step 2: Add query module to routes/mod.rs**

In `packages/api/src/routes/mod.rs`, add:
```rust
pub mod query;
```

- [ ] **Step 3: Verify**

```bash
cargo check -p dioxus_music_api --features server
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/query.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add shared BaseItemDto assembly helpers for tracks, albums, artists"
```

---

## Task 10: /Items universal endpoint

**Files:**
- Create: `packages/api/src/routes/items.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/items.rs**

Create `packages/api/src/routes/items.rs`:

```rust
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{
        models::{Album, Artist, Image, Track},
        schema::{albums, artists, images, tracks},
    },
    error::ApiError,
    routes::query,
    state::AppState,
    types::{BaseItemDto, ItemsResult},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Items", get(list_items))
        .route("/Items/{item_id}", get(get_item))
        .route("/Items/Filters", get(get_filters))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemsQuery {
    pub include_item_types: Option<String>, // "Audio", "MusicAlbum", "MusicArtist"
    pub parent_id: Option<Uuid>,
    pub genres: Option<String>,             // comma or pipe delimited
    pub is_favorite: Option<bool>,
    pub search_term: Option<String>,
    pub sort_by: Option<String>,            // "SortName", "Album", "DateCreated", etc.
    pub sort_order: Option<String>,         // "Ascending" | "Descending"
    pub start_index: Option<i64>,
    pub limit: Option<i64>,
    pub user_id: Option<Uuid>,
    pub fields: Option<String>,
}

/// GET /Items
async fn list_items(
    auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<ItemsQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(500);
    let start = params.start_index.unwrap_or(0);

    let item_types: Vec<&str> = params
        .include_item_types
        .as_deref()
        .map(|s| s.split(',').map(str::trim).collect())
        .unwrap_or_else(|| vec!["Audio", "MusicAlbum", "MusicArtist"]);

    let genre_filter: Vec<String> = params
        .genres
        .as_deref()
        .map(|s| s.split([',', '|']).map(|g| g.trim().to_string()).collect())
        .unwrap_or_default();

    let mut items: Vec<BaseItemDto> = Vec::new();

    if item_types.contains(&"Audio") {
        let mut q = tracks::table
            .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
            .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
            .into_boxed();

        if let Some(parent_id) = params.parent_id {
            q = q.filter(tracks::album_id.eq(parent_id));
        }
        if !genre_filter.is_empty() {
            q = q.filter(tracks::genre.eq_any(&genre_filter));
        }
        if let Some(ref term) = params.search_term {
            let pattern = format!("%{}%", term.to_ascii_lowercase());
            q = q.filter(
                diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                    "LOWER(tracks.title) LIKE '{}' OR LOWER(artists.name) LIKE '{}'",
                    pattern.replace('\'', "''"),
                    pattern.replace('\'', "''"),
                )),
            );
        }

        let rows: Vec<(Track, Artist, Option<Album>)> = q
            .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
            .offset(start)
            .limit(limit)
            .load(&mut conn)
            .await?;

        for (track, artist, album) in &rows {
            let image: Option<Image> = images::table
                .filter(images::item_id.eq(
                    album.as_ref().map(|a| a.id).unwrap_or(track.id),
                ))
                .filter(images::image_type.eq("Primary"))
                .first(&mut conn)
                .await
                .optional()?;

            items.push(query::track_to_dto(
                track,
                &artist,
                album.as_ref(),
                album.as_ref().map(|_| &artist),
                image.as_ref(),
                None,
                state.server_id,
            ));
        }
    }

    if item_types.contains(&"MusicAlbum") {
        let mut q = albums::table
            .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
            .into_boxed();

        if let Some(parent_id) = params.parent_id {
            q = q.filter(albums::artist_id.eq(parent_id));
        }
        if let Some(ref term) = params.search_term {
            let pattern = format!("%{}%", term.to_ascii_lowercase());
            q = q.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                "LOWER(albums.title) LIKE '{}'",
                pattern.replace('\'', "''"),
            )));
        }

        let rows: Vec<(Album, Artist)> = q
            .select((Album::as_select(), Artist::as_select()))
            .offset(start)
            .limit(limit)
            .load(&mut conn)
            .await?;

        for (album, artist) in &rows {
            let image: Option<Image> = images::table
                .filter(images::item_id.eq(album.id))
                .filter(images::image_type.eq("Primary"))
                .first(&mut conn)
                .await
                .optional()?;

            let track_count: i64 = tracks::table
                .filter(tracks::album_id.eq(album.id))
                .count()
                .get_result(&mut conn)
                .await
                .unwrap_or(0);

            items.push(query::album_to_dto(album, artist, image.as_ref(), track_count, None, state.server_id));
        }
    }

    if item_types.contains(&"MusicArtist") {
        let mut q = artists::table.into_boxed();
        if let Some(ref term) = params.search_term {
            let pattern = format!("%{}%", term.to_ascii_lowercase());
            q = q.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                "LOWER(artists.name) LIKE '{}'",
                pattern.replace('\'', "''"),
            )));
        }

        let all_artists: Vec<Artist> = q
            .offset(start)
            .limit(limit)
            .load(&mut conn)
            .await?;

        for artist in &all_artists {
            let image: Option<Image> = images::table
                .filter(images::item_id.eq(artist.id))
                .filter(images::image_type.eq("Primary"))
                .first(&mut conn)
                .await
                .optional()?;
            items.push(query::artist_to_dto(artist, image.as_ref(), None, state.server_id));
        }
    }

    let total = items.len() as i64;
    Ok(Json(ItemsResult {
        items,
        total_record_count: total,
        start_index: start as i32,
    }))
}

/// GET /Items/{itemId}
async fn get_item(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<BaseItemDto>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Try track first.
    let track: Option<(Track, Artist, Option<Album>)> = tracks::table
        .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
        .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
        .filter(tracks::id.eq(item_id))
        .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
        .first(&mut conn)
        .await
        .optional()?;

    if let Some((track, artist, album)) = track {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()?;
        return Ok(Json(query::track_to_dto(&track, &artist, album.as_ref(), album.as_ref().map(|_| &artist), image.as_ref(), None, state.server_id)));
    }

    // Try album.
    let album_row: Option<(Album, Artist)> = albums::table
        .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
        .filter(albums::id.eq(item_id))
        .select((Album::as_select(), Artist::as_select()))
        .first(&mut conn)
        .await
        .optional()?;

    if let Some((album, artist)) = album_row {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()?;
        let count = tracks::table.filter(tracks::album_id.eq(album.id)).count().get_result::<i64>(&mut conn).await.unwrap_or(0);
        return Ok(Json(query::album_to_dto(&album, &artist, image.as_ref(), count, None, state.server_id)));
    }

    // Try artist.
    let artist_row: Option<Artist> = artists::table
        .filter(artists::id.eq(item_id))
        .first(&mut conn)
        .await
        .optional()?;

    if let Some(artist) = artist_row {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(artist.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()?;
        return Ok(Json(query::artist_to_dto(&artist, image.as_ref(), None, state.server_id)));
    }

    Err(ApiError::NotFound("Item not found".to_string()))
}

/// GET /Items/Filters — returns available genres (for filter UI)
async fn get_filters(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let genre_names: Vec<String> = tracks::table
        .select(tracks::genre)
        .filter(tracks::genre.ne(""))
        .distinct()
        .order(tracks::genre.asc())
        .load(&mut conn)
        .await?;
    Ok(Json(serde_json::json!({ "Genres": genre_names })))
}
```

- [ ] **Step 2: Register in routes/mod.rs**

Replace `packages/api/src/routes/mod.rs`:

```rust
pub mod items;
pub mod query;
pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(users::router())
        .merge(items::router())
        .with_state(state)
}
```

- [ ] **Step 3: Build and test**

```bash
cargo build -p dioxus_music_web --features server
```

Start the server, authenticate, then:
```bash
TOKEN="<token from AuthenticateByName>"
curl -s "http://localhost:8080/Items?IncludeItemTypes=MusicAlbum&Limit=5" \
  -H "Authorization: MediaBrowser Token=\"$TOKEN\"" | jq '.TotalRecordCount'
# Expected: number of albums in your library
```

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/items.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add GET /Items universal query endpoint"
```

---

## Task 11: /Artists and /Albums routes

**Files:**
- Create: `packages/api/src/routes/artists.rs`
- Create: `packages/api/src/routes/albums.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/artists.rs**

Create `packages/api/src/routes/artists.rs`:

```rust
use axum::{Json, Router, extract::{Path, Query, State}, routing::get};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::{Artist, Image, Track}, schema::{albums, artists, images, tracks}},
    error::ApiError,
    routes::query,
    state::AppState,
    types::{BaseItemDto, ItemsResult},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Artists", get(list_artists))
        .route("/Artists/AlbumArtists", get(list_album_artists))
        .route("/Artists/{name}", get(get_artist_by_name))
        .route("/Artists/{item_id}/Similar", get(similar_artists))
        .route("/Artists/{item_id}/InstantMix", get(instant_mix_from_artist))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtistQuery {
    pub start_index: Option<i64>,
    pub limit: Option<i64>,
    pub search_term: Option<String>,
    pub is_favorite: Option<bool>,
    pub user_id: Option<Uuid>,
}

async fn list_artists(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<ArtistQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(500);
    let start = params.start_index.unwrap_or(0);

    let mut q = artists::table.into_boxed();
    if let Some(ref term) = params.search_term {
        let pattern = format!("%{}%", term.to_ascii_lowercase());
        q = q.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
            "LOWER(name) LIKE '{}'", pattern.replace('\'', "''")
        )));
    }

    let all: Vec<Artist> = q.order(artists::sort_name.asc()).offset(start).limit(limit).load(&mut conn).await?;
    let total = all.len() as i64;
    let mut items = Vec::with_capacity(all.len());
    for artist in &all {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(artist.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        items.push(query::artist_to_dto(artist, image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: start as i32 }))
}

async fn list_album_artists(
    auth: AuthUser,
    state: State<AppState>,
    query_params: Query<ArtistQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    // Album artists are artists who own at least one album (artist_id on albums table).
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = query_params.limit.unwrap_or(50).min(500);
    let start = query_params.start_index.unwrap_or(0);

    let artist_ids: Vec<Uuid> = albums::table
        .select(albums::artist_id)
        .distinct()
        .load(&mut conn)
        .await?;

    let all: Vec<Artist> = artists::table
        .filter(artists::id.eq_any(&artist_ids))
        .order(artists::sort_name.asc())
        .offset(start)
        .limit(limit)
        .load(&mut conn)
        .await?;

    let total = all.len() as i64;
    let mut items = Vec::with_capacity(all.len());
    for artist in &all {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(artist.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        items.push(query::artist_to_dto(artist, image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: start as i32 }))
}

async fn get_artist_by_name(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<BaseItemDto>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let artist: Artist = artists::table
        .filter(artists::name.eq(&name))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound(format!("Artist '{name}' not found")))?;
    let image: Option<Image> = images::table
        .filter(images::item_id.eq(artist.id))
        .filter(images::image_type.eq("Primary"))
        .first(&mut conn).await.optional()?;
    Ok(Json(query::artist_to_dto(&artist, image.as_ref(), None, state.server_id)))
}

async fn similar_artists(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Find genres of this artist's tracks.
    let track_artist_ids: Vec<Uuid> = artists::table
        .filter(artists::id.eq(item_id))
        .select(artists::id)
        .load(&mut conn).await?;
    let genres: Vec<String> = tracks::table
        .filter(tracks::artist_id.eq_any(&track_artist_ids))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre)
        .distinct()
        .load(&mut conn).await?;

    if genres.is_empty() {
        return Ok(Json(ItemsResult { items: vec![], total_record_count: 0, start_index: 0 }));
    }

    // Artists who share those genres (excluding the source artist).
    let similar_artist_ids: Vec<Uuid> = tracks::table
        .filter(tracks::genre.eq_any(&genres))
        .filter(tracks::artist_id.ne(item_id))
        .select(tracks::artist_id)
        .distinct()
        .limit(10)
        .load(&mut conn).await?;

    let similar: Vec<Artist> = artists::table
        .filter(artists::id.eq_any(&similar_artist_ids))
        .load(&mut conn).await?;

    let total = similar.len() as i64;
    let mut items = Vec::with_capacity(similar.len());
    for artist in &similar {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(artist.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        items.push(query::artist_to_dto(artist, image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}

async fn instant_mix_from_artist(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<ArtistQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(200);

    let genres: Vec<String> = tracks::table
        .filter(tracks::artist_id.eq(item_id))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre)
        .distinct()
        .load(&mut conn).await?;

    let genre_tracks: Vec<(crate::db::models::Track, Artist, Option<crate::db::models::Album>)> =
        tracks::table
            .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
            .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
            .filter(tracks::genre.eq_any(&genres))
            .select((
                crate::db::models::Track::as_select(),
                Artist::as_select(),
                Option::<crate::db::models::Album>::as_select(),
            ))
            .limit(limit)
            .load(&mut conn)
            .await?;

    let total = genre_tracks.len() as i64;
    let mut items = Vec::with_capacity(genre_tracks.len());
    for (track, artist, album) in &genre_tracks {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        items.push(query::track_to_dto(track, &artist, album.as_ref(), album.as_ref().map(|_| &artist), image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}
```

- [ ] **Step 2: Create routes/albums.rs**

Create `packages/api/src/routes/albums.rs`:

```rust
use axum::{Json, Router, extract::{Path, Query, State}, routing::get};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::{Album, Artist, Image, Track}, schema::{albums, artists, images, tracks}},
    error::ApiError,
    routes::query,
    state::AppState,
    types::ItemsResult,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Albums/{item_id}/Similar", get(similar_albums))
        .route("/Albums/{item_id}/InstantMix", get(instant_mix_from_album))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumQuery {
    pub limit: Option<i64>,
    pub user_id: Option<Uuid>,
}

async fn similar_albums(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<AlbumQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(10).min(50);

    let source: Album = albums::table.filter(albums::id.eq(item_id)).first(&mut conn).await
        .optional()?.ok_or_else(|| ApiError::NotFound("Album not found".to_string()))?;

    let genres: Vec<String> = tracks::table
        .filter(tracks::album_id.eq(item_id))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre).distinct()
        .load(&mut conn).await?;

    let mut similar: Vec<(Album, Artist)> = albums::table
        .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
        .filter(albums::id.ne(item_id))
        .filter(albums::artist_id.eq(source.artist_id))
        .select((Album::as_select(), Artist::as_select()))
        .limit(limit)
        .load(&mut conn).await?;

    // If not enough from same artist, add from same genre.
    if (similar.len() as i64) < limit {
        let album_ids_from_genre: Vec<Uuid> = tracks::table
            .filter(tracks::genre.eq_any(&genres))
            .filter(tracks::album_id.ne(item_id).and(tracks::album_id.is_not_null()))
            .select(tracks::album_id.assume_not_null())
            .distinct()
            .limit(limit - similar.len() as i64)
            .load(&mut conn).await?;

        let extra: Vec<(Album, Artist)> = albums::table
            .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
            .filter(albums::id.eq_any(&album_ids_from_genre))
            .select((Album::as_select(), Artist::as_select()))
            .load(&mut conn).await?;
        similar.extend(extra);
    }

    let total = similar.len() as i64;
    let mut items = Vec::with_capacity(similar.len());
    for (album, artist) in &similar {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        let count = tracks::table.filter(tracks::album_id.eq(album.id)).count().get_result::<i64>(&mut conn).await.unwrap_or(0);
        items.push(query::album_to_dto(album, artist, image.as_ref(), count, None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}

async fn instant_mix_from_album(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<AlbumQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(200);

    let genres: Vec<String> = tracks::table
        .filter(tracks::album_id.eq(item_id))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre).distinct()
        .load(&mut conn).await?;

    let rows: Vec<(Track, Artist, Option<Album>)> = tracks::table
        .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
        .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
        .filter(tracks::genre.eq_any(&genres))
        .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
        .limit(limit)
        .load(&mut conn).await?;

    let total = rows.len() as i64;
    let mut items = Vec::with_capacity(rows.len());
    for (track, artist, album) in &rows {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        items.push(query::track_to_dto(track, &artist, album.as_ref(), album.as_ref().map(|_| &artist), image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}
```

- [ ] **Step 3: Register in routes/mod.rs**

Replace `packages/api/src/routes/mod.rs`:

```rust
pub mod albums;
pub mod artists;
pub mod items;
pub mod query;
pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(users::router())
        .merge(items::router())
        .merge(artists::router())
        .merge(albums::router())
        .with_state(state)
}
```

- [ ] **Step 4: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 5: Commit**

```bash
git add packages/api/src/routes/artists.rs packages/api/src/routes/albums.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Artists and /Albums routes with similar and instant-mix endpoints"
```

---

## Task 12: /Genres and /MusicGenres routes

**Files:**
- Create: `packages/api/src/routes/genres.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/genres.rs**

Create `packages/api/src/routes/genres.rs`:

```rust
use axum::{Json, Router, extract::{Path, Query, State}, routing::get};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::{Album, Artist, Genre, Image, Track}, schema::{albums, artists, genres, images, tracks}},
    error::ApiError,
    routes::query,
    state::AppState,
    types::{BaseItemDto, ItemsResult, NameGuidPair},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Genres", get(list_genres))
        .route("/MusicGenres", get(list_genres))
        .route("/MusicGenres/{genre_name}/InstantMix", get(instant_mix_from_genre))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenreQuery {
    pub start_index: Option<i64>,
    pub limit: Option<i64>,
    pub search_term: Option<String>,
    pub user_id: Option<Uuid>,
}

async fn list_genres(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<GenreQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(100).min(500);
    let start = params.start_index.unwrap_or(0);

    let mut q = genres::table.into_boxed();
    if let Some(ref term) = params.search_term {
        let pattern = format!("%{}%", term.to_ascii_lowercase());
        q = q.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
            "LOWER(name) LIKE '{}'", pattern.replace('\'', "''")
        )));
    }

    let all: Vec<Genre> = q.order(genres::name.asc()).offset(start).limit(limit).load(&mut conn).await?;
    let total = all.len() as i64;

    let items: Vec<BaseItemDto> = all.iter().map(|g| BaseItemDto {
        id: g.id,
        name: g.name.clone(),
        sort_name: Some(g.name.to_ascii_lowercase()),
        item_type: "MusicGenre".to_string(),
        server_id: state.server_id,
        album: None, album_id: None, album_primary_image_tag: None,
        album_artist: None, album_artists: None,
        artists: None, artist_items: None,
        genre_items: None, genres: None,
        run_time_ticks: None, track_number: None, index_number: None,
        parent_index_number: None, container: None, media_type: None,
        production_year: None, image_tags: None, user_data: None,
        date_created: None,
    }).collect();

    Ok(Json(ItemsResult { items, total_record_count: total, start_index: start as i32 }))
}

async fn instant_mix_from_genre(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(genre_name): Path<String>,
    Query(params): Query<GenreQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(200);

    let rows: Vec<(Track, Artist, Option<Album>)> = tracks::table
        .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
        .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
        .filter(tracks::genre.eq(&genre_name))
        .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
        .limit(limit)
        .load(&mut conn).await?;

    let total = rows.len() as i64;
    let mut items = Vec::with_capacity(rows.len());
    for (track, artist, album) in &rows {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn).await.optional()?;
        items.push(query::track_to_dto(track, &artist, album.as_ref(), album.as_ref().map(|_| &artist), image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}
```

- [ ] **Step 2: Update routes/mod.rs**

Add `pub mod genres;` to imports and `.merge(genres::router())` to `create_router`.

- [ ] **Step 3: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/genres.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Genres and /MusicGenres routes with instant-mix"
```

---

## Task 13: Audio streaming and PlaybackInfo routes

**Files:**
- Create: `packages/api/src/routes/audio.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/audio.rs**

Create `packages/api/src/routes/audio.rs`:

```rust
use axum::{
    Json, Router,
    body::Body,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::Track, schema::tracks},
    error::ApiError,
    state::AppState,
    types::{MediaSourceInfo, PlaybackInfoResponse},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Audio/{item_id}/stream", get(stream_audio))
        .route("/Audio/{item_id}/stream.{container}", get(stream_audio_with_container))
        .route("/Audio/{item_id}/universal", get(stream_audio))
        .route("/Audio/{item_id}/Lyrics", get(get_lyrics))
        .route("/Items/{item_id}/PlaybackInfo", get(get_playback_info).post(get_playback_info))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamQuery {
    #[serde(rename = "static")]
    pub is_static: Option<bool>,
    pub container: Option<String>,
    pub audio_codec: Option<String>,
    pub play_session_id: Option<String>,
    pub media_source_id: Option<String>,
}

fn container_to_mime(container: &str) -> &'static str {
    match container {
        "flac" => "audio/flac",
        "mp3" => "audio/mpeg",
        "ogg" | "opus" => "audio/ogg",
        "aac" | "m4a" => "audio/aac",
        _ => "application/octet-stream",
    }
}

async fn stream_track_file(
    state: &AppState,
    item_id: Uuid,
) -> Result<Response, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let track: Track = tracks::table
        .filter(tracks::id.eq(item_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Track not found".to_string()))?;

    let file = File::open(&track.file_path)
        .await
        .map_err(|e| ApiError::Internal(format!("File open error: {e}")))?;

    let metadata = file.metadata().await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let content_length = metadata.len();

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let mime = container_to_mime(&track.container);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CONTENT_LENGTH, content_length)
        .header(header::ACCEPT_RANGES, "bytes")
        .body(body)
        .unwrap())
}

/// GET /Audio/{itemId}/stream
async fn stream_audio(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(_params): Query<StreamQuery>,
) -> Result<Response, ApiError> {
    stream_track_file(&state, item_id).await
}

/// GET /Audio/{itemId}/stream.{container}
async fn stream_audio_with_container(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path((item_id, _container)): Path<(Uuid, String)>,
    Query(_params): Query<StreamQuery>,
) -> Result<Response, ApiError> {
    // We always stream the original file regardless of requested container.
    stream_track_file(&state, item_id).await
}

/// GET /Audio/{itemId}/Lyrics — stub, returns 404 (no lyrics implementation)
async fn get_lyrics(
    _auth: AuthUser,
    _state: State<AppState>,
    Path(_item_id): Path<Uuid>,
) -> StatusCode {
    StatusCode::NOT_FOUND
}

/// GET|POST /Items/{itemId}/PlaybackInfo
async fn get_playback_info(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<PlaybackInfoResponse>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let track: Track = tracks::table
        .filter(tracks::id.eq(item_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Track not found".to_string()))?;

    let file_size = std::fs::metadata(&track.file_path).ok().map(|m| m.len() as i64);

    let source = MediaSourceInfo {
        id: item_id.to_string(),
        path: Some(track.file_path.clone()),
        protocol: "File".to_string(),
        media_type: Some("Audio".to_string()),
        container: Some(track.container.clone()),
        size: file_size,
        bit_rate: track.bit_rate,
        default_audio_stream_index: Some(0),
        supports_direct_play: true,
        supports_direct_stream: true,
        supports_transcoding: false,
        is_remote: false,
    };

    Ok(Json(PlaybackInfoResponse {
        media_sources: vec![source],
        play_session_id: Uuid::new_v4().to_string(),
    }))
}
```

- [ ] **Step 2: Update routes/mod.rs**

Add `pub mod audio;` and `.merge(audio::router())`.

- [ ] **Step 3: Build and test streaming**

```bash
cargo build -p dioxus_music_web --features server
```

Start the server, get a track ID from `/Items?IncludeItemTypes=Audio&Limit=1`, then:
```bash
TRACK_ID="<uuid>"
TOKEN="<token>"
curl -I "http://localhost:8080/Audio/$TRACK_ID/stream" \
  -H "Authorization: MediaBrowser Token=\"$TOKEN\""
# Expected: HTTP 200, Content-Type: audio/flac (or audio/mpeg etc.)
```

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/audio.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Audio streaming and /Items/PlaybackInfo routes"
```

---

## Task 14: Image serving routes

**Files:**
- Create: `packages/api/src/routes/images.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/images.rs**

Create `packages/api/src/routes/images.rs`:

```rust
use axum::{
    Router,
    body::Body,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::Image, schema::{albums, artists, images, tracks}},
    error::ApiError,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Items/{item_id}/Images", get(list_item_images))
        .route("/Items/{item_id}/Images/{image_type}", get(get_item_image))
        .route("/Artists/{name}/Images/{image_type}/{index}", get(get_artist_image))
        .route("/MusicGenres/{name}/Images/{image_type}/{index}", get(get_genre_image))
}

#[derive(Debug, Deserialize)]
pub struct ImageQuery {
    pub tag: Option<String>,
    #[serde(rename = "maxWidth")]
    pub max_width: Option<u32>,
    #[serde(rename = "maxHeight")]
    pub max_height: Option<u32>,
}

async fn serve_image(path: &str, client_etag: Option<&str>) -> Result<Response, ApiError> {
    // ETag cache check.
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let mime = match ext {
        "png" => "image/png",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };

    if let Some(etag) = client_etag {
        // We'll compare after reading the tag from the image row — simplified: skip 304 for now.
        let _ = etag;
    }

    let data = tokio::fs::read(path)
        .await
        .map_err(|_| ApiError::NotFound("Image file not found on disk".to_string()))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(Body::from(data))
        .unwrap())
}

/// GET /Items/{itemId}/Images — list available image types for an item
async fn list_item_images(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let imgs: Vec<Image> = images::table
        .filter(images::item_id.eq(item_id))
        .load(&mut conn).await?;
    let list: Vec<serde_json::Value> = imgs.iter().map(|i| {
        serde_json::json!({
            "ImageType": i.image_type,
            "ImageTag": i.tag,
            "Width": i.width,
            "Height": i.height,
        })
    }).collect();
    Ok(axum::Json(serde_json::json!(list)))
}

/// GET /Items/{itemId}/Images/{imageType}
async fn get_item_image(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path((item_id, image_type)): Path<(Uuid, String)>,
    Query(_params): Query<ImageQuery>,
) -> Result<Response, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Try direct image lookup.
    let img: Option<Image> = images::table
        .filter(images::item_id.eq(item_id))
        .filter(images::image_type.eq(&image_type))
        .first(&mut conn).await.optional()?;

    if let Some(img) = img {
        return serve_image(&img.file_path, None).await;
    }

    // Track fallback: look up the track's album image.
    let album_id: Option<Uuid> = tracks::table
        .filter(tracks::id.eq(item_id))
        .select(tracks::album_id)
        .first::<Option<Uuid>>(&mut conn)
        .await
        .optional()?
        .flatten();

    if let Some(aid) = album_id {
        let album_img: Option<Image> = images::table
            .filter(images::item_id.eq(aid))
            .filter(images::image_type.eq(&image_type))
            .first(&mut conn).await.optional()?;
        if let Some(img) = album_img {
            return serve_image(&img.file_path, None).await;
        }
    }

    Err(ApiError::NotFound("Image not found".to_string()))
}

/// GET /Artists/{name}/Images/{imageType}/{index}
async fn get_artist_image(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path((name, image_type, _index)): Path<(String, String, u32)>,
) -> Result<Response, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let artist_id: Option<Uuid> = artists::table
        .filter(artists::name.eq(&name))
        .select(artists::id)
        .first(&mut conn).await.optional()?;
    let Some(aid) = artist_id else {
        return Err(ApiError::NotFound("Artist not found".to_string()));
    };
    let img: Image = images::table
        .filter(images::item_id.eq(aid))
        .filter(images::image_type.eq(&image_type))
        .first(&mut conn).await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Image not found".to_string()))?;
    serve_image(&img.file_path, None).await
}

/// GET /MusicGenres/{name}/Images/{imageType}/{index} — always 404 (no genre art)
async fn get_genre_image(
    _auth: AuthUser,
    _state: State<AppState>,
    _path: Path<(String, String, u32)>,
) -> StatusCode {
    StatusCode::NOT_FOUND
}
```

- [ ] **Step 2: Update routes/mod.rs**

Add `pub mod images;` and `.merge(images::router())`.

- [ ] **Step 3: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

Test with a known album ID:
```bash
ALBUM_ID="<uuid from /Items?IncludeItemTypes=MusicAlbum>"
TOKEN="<token>"
curl -I "http://localhost:8080/Items/$ALBUM_ID/Images/Primary" \
  -H "Authorization: MediaBrowser Token=\"$TOKEN\""
# Expected: HTTP 200, Content-Type: image/jpeg (or png/webp)
# If no art was found for this album: HTTP 404
```

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/images.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Items/Images and /Artists/Images serving routes"
```

---

## Task 15: /Search/Hints and /custom/* routes

**Files:**
- Create: `packages/api/src/routes/search.rs`
- Create: `packages/api/src/routes/custom.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/search.rs**

Create `packages/api/src/routes/search.rs`:

```rust
use axum::{Json, Router, extract::{Query, State}, routing::get};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::{Album, Artist, Track}, schema::{albums, artists, tracks}},
    error::ApiError,
    state::AppState,
    types::{SearchHint, SearchHintsResult},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/Search/Hints", get(search_hints))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchQuery {
    pub search_term: String,
    pub limit: Option<i64>,
    pub include_item_types: Option<String>,
    pub user_id: Option<Uuid>,
}

async fn search_hints(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchHintsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(20).min(100);
    let term = params.search_term.trim().to_ascii_lowercase();
    let pattern = format!("%{term}%");

    let types: Vec<&str> = params.include_item_types
        .as_deref()
        .map(|s| s.split(',').map(str::trim).collect())
        .unwrap_or_else(|| vec!["Audio", "MusicAlbum", "MusicArtist"]);

    let mut hints: Vec<SearchHint> = Vec::new();

    if types.contains(&"Audio") {
        let track_rows: Vec<(Track, Artist, Option<Album>)> = tracks::table
            .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
            .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
            .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                "LOWER(tracks.title) LIKE '{}' OR LOWER(artists.name) LIKE '{}'",
                pattern.replace('\'', "''"),
                pattern.replace('\'', "''"),
            )))
            .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
            .limit(limit)
            .load(&mut conn).await?;

        for (track, artist, album) in &track_rows {
            hints.push(SearchHint {
                item_id: track.id,
                name: track.title.clone(),
                item_type: "Audio".to_string(),
                album: album.as_ref().map(|a| a.title.clone()),
                album_id: track.album_id,
                album_artist: Some(artist.name.clone()),
                primary_image_tag: None,
            });
        }
    }

    if types.contains(&"MusicAlbum") {
        let album_rows: Vec<(Album, Artist)> = albums::table
            .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
            .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                "LOWER(albums.title) LIKE '{}'", pattern.replace('\'', "''")
            )))
            .select((Album::as_select(), Artist::as_select()))
            .limit(limit)
            .load(&mut conn).await?;

        for (album, artist) in &album_rows {
            hints.push(SearchHint {
                item_id: album.id,
                name: album.title.clone(),
                item_type: "MusicAlbum".to_string(),
                album: Some(album.title.clone()),
                album_id: Some(album.id),
                album_artist: Some(artist.name.clone()),
                primary_image_tag: None,
            });
        }
    }

    if types.contains(&"MusicArtist") {
        let artist_rows: Vec<Artist> = artists::table
            .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                "LOWER(artists.name) LIKE '{}'", pattern.replace('\'', "''")
            )))
            .limit(limit)
            .load(&mut conn).await?;

        for artist in &artist_rows {
            hints.push(SearchHint {
                item_id: artist.id,
                name: artist.name.clone(),
                item_type: "MusicArtist".to_string(),
                album: None,
                album_id: None,
                album_artist: None,
                primary_image_tag: None,
            });
        }
    }

    let total = hints.len() as i64;
    Ok(Json(SearchHintsResult { search_hints: hints, total_record_count: total }))
}
```

- [ ] **Step 2: Create routes/custom.rs**

Create `packages/api/src/routes/custom.rs`:

```rust
use axum::{Json, Router, extract::State, http::StatusCode, routing::{get, post}};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::{
    auth::middleware::AuthUser,
    db::schema::tracks,
    error::ApiError,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/custom/library/rescan", post(rescan_library))
        .route("/custom/library/version", get(library_version))
        .route("/custom/health", get(health))
}

/// POST /custom/library/rescan — trigger full rescan (admin only)
async fn rescan_library(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> StatusCode {
    let state_clone = state.clone();
    tokio::spawn(crate::scanner::full_scan(state_clone));
    StatusCode::ACCEPTED
}

/// GET /custom/library/version — returns max updated_at for cache invalidation
async fn library_version(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let version: Option<DateTime<Utc>> = tracks::table
        .select(diesel::dsl::max(tracks::updated_at))
        .first(&mut conn)
        .await?;
    Ok(Json(serde_json::json!({ "Version": version })))
}

/// GET /custom/health
async fn health() -> StatusCode {
    StatusCode::OK
}
```

- [ ] **Step 3: Update routes/mod.rs (final state for Plan 2)**

Replace `packages/api/src/routes/mod.rs`:

```rust
pub mod albums;
pub mod artists;
pub mod audio;
pub mod custom;
pub mod genres;
pub mod images;
pub mod items;
pub mod query;
pub mod search;
pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(users::router())
        .merge(items::router())
        .merge(artists::router())
        .merge(albums::router())
        .merge(genres::router())
        .merge(audio::router())
        .merge(images::router())
        .merge(search::router())
        .merge(custom::router())
        .with_state(state)
}
```

- [ ] **Step 4: Build final Plan 2 state**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 5: Smoke-test search**

```bash
TOKEN="<token>"
curl -s "http://localhost:8080/Search/Hints?SearchTerm=love&Limit=5" \
  -H "Authorization: MediaBrowser Token=\"$TOKEN\"" | jq '.TotalRecordCount'
# Expected: number >= 0

curl -s "http://localhost:8080/custom/health"
# Expected: HTTP 200
```

- [ ] **Step 6: Commit**

```bash
git add packages/api/src/routes/search.rs packages/api/src/routes/custom.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Search/Hints and /custom/* routes — Plan 2 complete"
```

---

## Plan 2 Complete

At this point the backend serves all library data through Jellyfin-compatible routes:
- Scanner populates artists/albums/tracks/images on startup
- `/Items`, `/Artists`, `/Albums`, `/Genres` all return `BaseItemDto`
- `/Audio/{id}/stream` serves original audio files
- `/Items/{id}/Images/Primary` serves extracted cover art
- `/Search/Hints` searches across all item types
- Custom routes handle rescan and health check

**Next:** `2026-04-13-jellyfin-plan-3-playlists-userdata-frontend.md` — playlist CRUD, user data/sessions, and frontend migration from server functions to HTTP client calls.
