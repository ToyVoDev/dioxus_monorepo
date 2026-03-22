# Offline Sync & Automotive UI Design

**Date:** 2026-03-11
**Status:** Approved

## Problem

Current dioxus_music is a web-only streaming app requiring an always-on server connection. The goal is seamless offline playback across desktop and mobile (replacing Jellyfin + Finamp's manual offline toggle) plus Android Auto and CarPlay support.

## Architecture Overview

### Client-Server Model

- **Server** (existing): Postgres + Diesel, music scanner, audio streaming via `/stream/{id}`. Source of truth for library and playlists.
- **Desktop client**: SQLite (Diesel) for local metadata + filesystem storage for downloaded audio. Connects to remote server.
- **Android client**: Room database for metadata + Android file storage for audio. Exposes Android Auto media session.
- **iOS client**: Core Data for metadata + iOS file storage for audio. Exposes CarPlay media session.
- **Web client**: Always-connected, no local persistence. Streams everything.

All clients are equal — they stream from the server and cache locally. No client acts as a server.

### Crate Structure

```
dioxus_music/packages/
├── api/          # (existing) Server functions, models, scanner, streaming
├── ui/           # (existing) Shared Dioxus components
├── web/          # (existing) Web entry point, always-connected
├── desktop/      # SQLite cache layer + Dioxus UI, connects to remote server
├── mobile/       # Shared mobile Dioxus UI + platform bridge traits
├── android/      # Kotlin: Android Auto MediaBrowserService + Room DB + Rust FFI
└── ios/          # Swift: CarPlay integration + Core Data + Rust FFI
```

`mobile` crate holds shared Rust/Dioxus mobile UI. `android/` and `ios/` are platform-native projects embedding the Dioxus mobile app and adding automotive APIs.

## Connectivity Detection

- On app start and periodically (~30s), ping `GET /api/health`
- Maintain a `ConnectionStatus` signal: `Online | Offline`
- No manual toggle — routing is automatic

## Sync & Cache Layer

### Local Storage

**Desktop (SQLite via Diesel):**

- Mirror server schema: `tracks`, `playlists`, `playlist_tracks`
- Additional fields:
  - `tracks.local_path: Option<String>` — path to cached audio file
  - `tracks.cached_at: Option<i64>` — download timestamp
  - `tracks.sync_status: enum { NotCached, Cached, PendingSync }`
  - `sync_config` table: `playlist_id, enabled` — playlists marked for offline
  - `cache_meta` table: key-value for last sync timestamp, cache size limit, etc.

**Android (Room) / iOS (Core Data):**

- Same logical schema, platform-native ORM
- Exposed to Rust via UniFFI trait bridge

### Sync Flow

1. On connect (or app start while online): fetch library metadata from server, upsert into local DB. Incremental via `GET /api/library?since={timestamp}`.
2. For playlists marked "offline sync": download all uncached tracks. Background queue, priority to currently-playing playlist.
3. Opportunistic caching: after streaming any track, save audio to local storage and mark cached.
4. Cache eviction: opportunistically cached tracks evicted LRU when storage exceeds configurable limit (default 2GB). Synced playlist tracks never auto-evicted.

### Playback Routing

1. Track has `local_path` and file exists → play from local file
2. Else if online → stream from server, opportunistically cache response
3. Else → track unavailable, skip to next playable track

### Download Management

- "Mark playlist for offline" toggle in UI
- Background download queue with progress tracking
- Downloads are track-level (shared across playlists — a track synced for playlist A is available for playlist B)

## Platform Bridge

### Trait Abstraction

```rust
pub trait PlatformStorage: Send + Sync {
    async fn upsert_tracks(&self, tracks: Vec<TrackSummary>) -> Result<()>;
    async fn get_all_tracks(&self) -> Result<Vec<TrackSummary>>;
    async fn get_cached_tracks(&self) -> Result<Vec<TrackSummary>>;
    async fn mark_cached(&self, track_id: Uuid, local_path: String) -> Result<()>;
    async fn remove_cache(&self, track_id: Uuid) -> Result<()>;
    async fn get_synced_playlist_ids(&self) -> Result<Vec<Uuid>>;
    async fn set_playlist_sync(&self, playlist_id: Uuid, enabled: bool) -> Result<()>;
    // playlist operations mirror server API
}

pub trait PlatformAudio: Send + Sync {
    fn play_file(&self, path: &str) -> Result<()>;
    fn play_url(&self, url: &str) -> Result<()>;
    fn pause(&self);
    fn resume(&self);
    fn seek(&self, position_secs: f64);
    fn on_playback_event(&self, callback: Box<dyn Fn(PlaybackEvent)>);
}
```

**Desktop:** `PlatformStorage` = Diesel SQLite in Rust. `PlatformAudio` = HTML5 `<audio>` via `document::eval` (Dioxus desktop webview).

**Android:** `PlatformStorage` = Kotlin Room via UniFFI. `PlatformAudio` = ExoPlayer via UniFFI.

**iOS:** `PlatformStorage` = Swift Core Data via UniFFI. `PlatformAudio` = AVQueuePlayer via UniFFI.

### PlayerState Refactor

Current `PlayerState` controls an HTML5 `<audio>` element via JS eval. Must become platform-agnostic:

- `PlayerState` calls `PlatformAudio` trait methods
- Each platform provides a concrete implementation
- Web keeps current `document::eval` approach
- On mobile, `PlayerState` reads/writes through the platform's media session (see Automotive section)

## Android Auto

Android Auto requires a native `MediaBrowserServiceCompat`. No Dioxus UI — Android Auto renders its own UI from provided data.

### Media Browse Tree

```
Root
├── Library (all tracks, paginated)
├── Playlists
│   ├── Playlist A → tracks
│   ├── Playlist B → tracks
│   └── ...
├── Recently Played
└── Downloaded (offline-available tracks)
```

### Components (Kotlin)

- `MusicMediaBrowserService`: extends `MediaBrowserServiceCompat`, builds browse tree from Room DB, handles search
- `MusicMediaSession`: manages `MediaSessionCompat`, transport controls, now-playing metadata + album art
- Shares Room database with Dioxus mobile app
- Playback: ExoPlayer, shared between main app and Auto service

### Constraints

- Max browse depth: 4 levels
- Max list items: ~100 per page (paginate)

### Shared Playback

`MediaSession` is the single source of truth on Android. Both Dioxus UI and Android Auto read/write through it. `PlayerState` on Android becomes a view over `MediaSession` state.

## CarPlay

### Components (Swift)

- `CarPlaySceneDelegate`: implements `CPTemplateApplicationSceneDelegate`, builds tab bar
- Tab templates: Library (list), Playlists (list → track list), Downloaded, Now Playing
- `MPRemoteCommandCenter` for transport controls
- `MPNowPlayingInfoCenter` for lock screen / now-playing metadata
- Reads from Core Data, shares `AVQueuePlayer` with main app

### Shared Playback

`MPNowPlayingInfoCenter` + shared playback service is the authority on iOS. Dioxus `PlayerState` on iOS reads/writes through it, same pattern as Android.

## Server API Changes

### New Endpoints

```
GET  /api/health                       → 200 OK, connectivity check
GET  /api/library?since={timestamp}    → incremental sync, tracks modified after timestamp
GET  /api/playlists?since={timestamp}  → incremental playlist sync
GET  /api/track/{id}/cover             → album art (extracted by lofty)
```

### Schema Changes

- Add `updated_at: TIMESTAMPTZ` to `tracks` and `playlists` tables (set on insert/update)
- Album art: extract via lofty, serve at `/api/track/{id}/cover`

### Streaming Improvements

- Add `Content-Length` header to `/stream/{id}` response (enables download progress)
- Future: `Range` request support for resumable downloads

## Implementation Phases

### Phase 1 — Foundation

- Server: `/health`, incremental sync (`?since=`), `updated_at` columns, `Content-Length` on streams
- Album art extraction + `/api/track/{id}/cover`
- `PlatformAudio` / `PlatformStorage` trait definitions
- Desktop: SQLite local DB via Diesel, sync engine, playback routing

### Phase 2 — Desktop Offline Experience

- Connectivity detection + `ConnectionStatus` signal
- Download manager: background queue, progress tracking
- Playlist sync toggle UI
- Opportunistic caching on stream playback
- Cache management: LRU eviction, storage limit settings
- Offline-aware views (grey out unavailable tracks when offline)

### Phase 3 — Mobile App (Android First)

- Dioxus mobile UI — port views from web, adapt for touch
- Kotlin: Room DB implementing `PlatformStorage` via UniFFI
- ExoPlayer implementing `PlatformAudio`
- Sync + cache (same logic as desktop, different storage backend)
- Connectivity detection via Android network APIs

### Phase 4 — Android Auto

- `MusicMediaBrowserService` + browse tree from Room DB
- `MediaSession` as shared playback authority
- Refactor `PlayerState` on Android to read/write through MediaSession
- Album art in now-playing metadata

### Phase 5 — iOS + CarPlay

- Swift: Core Data + AVQueuePlayer
- CarPlay scene delegate + templates
- `MPNowPlayingInfoCenter` + `MPRemoteCommandCenter`
- Same sync/cache model as Android

### Future (Not In Scope)

- PWA offline support for web
- Range request support for resumable downloads
- Search in Android Auto / CarPlay
- Multi-user / auth
