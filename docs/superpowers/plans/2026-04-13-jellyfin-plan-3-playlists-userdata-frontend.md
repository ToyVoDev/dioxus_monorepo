# Jellyfin API Migration — Plan 3: Playlists, User Data, Sessions + Frontend

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the backend with playlist CRUD, user data, and session routes; then migrate all Dioxus frontend views from server-function calls to HTTP client calls against the Jellyfin API.

**Architecture:** Three new route modules (`playlists`, `user_data`, `sessions`) are added to the api router. The `ui` crate gets an `api_client` module that wraps HTTP calls using `reqwest`. Each view is rewritten to use `use_resource` with `api_client` calls instead of server function calls.

**Tech Stack:** Axum 0.8, Diesel 2 + diesel-async, reqwest (WASM + native), Dioxus 0.7

**Prerequisite:** Plans 1 and 2 must be complete (auth + library API working).

**Spec:** `docs/superpowers/specs/2026-04-13-jellyfin-api-migration-design.md`

---

## File Map

**Create:**
- `packages/api/src/routes/playlists.rs`
- `packages/api/src/routes/user_data.rs`
- `packages/api/src/routes/sessions.rs`
- `packages/ui/src/api_client.rs`

**Modify:**
- `packages/api/src/routes/mod.rs` (add 3 new routers)
- `packages/api/src/types.rs` (add playlist DTOs, smart rules)
- `packages/ui/Cargo.toml` (add reqwest)
- `packages/ui/src/lib.rs` (expose api_client)
- `packages/web/src/views/library.rs`
- `packages/web/src/views/album_detail.rs`
- `packages/web/src/views/artists.rs`
- `packages/web/src/views/playlists.rs`
- `packages/web/src/views/playlist_view.rs`
- `packages/web/src/views/now_playing.rs`

---

## Task 1: Playlist and smart-rules DTOs in types.rs

**Files:**
- Modify: `packages/api/src/types.rs`

- [ ] **Step 1: Append playlist types to types.rs**

Add to the end of `packages/api/src/types.rs`:

```rust
/// Smart playlist genre filter rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPlaylistRules {
    pub include_genres: Vec<String>,
    pub exclude_genres: Vec<String>,
}

impl SmartPlaylistRules {
    /// Check if a track's genre matches these rules.
    pub fn matches(&self, genre: &str) -> bool {
        let included = self.include_genres.is_empty()
            || self.include_genres.iter().any(|g| g.eq_ignore_ascii_case(genre));
        let excluded = self.exclude_genres.iter().any(|g| g.eq_ignore_ascii_case(genre));
        included && !excluded
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub ids: Option<Vec<Uuid>>,
    pub user_id: Option<Uuid>,
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdatePlaylistRequest {
    pub name: Option<String>,
    pub overview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSmartPlaylistRequest {
    pub name: String,
    pub rules: SmartPlaylistRules,
    pub user_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_rules_empty_include_matches_all() {
        let rules = SmartPlaylistRules {
            include_genres: vec![],
            exclude_genres: vec![],
        };
        assert!(rules.matches("Jazz"));
        assert!(rules.matches("Blues"));
    }

    #[test]
    fn smart_rules_include_filters() {
        let rules = SmartPlaylistRules {
            include_genres: vec!["Jazz".to_string()],
            exclude_genres: vec![],
        };
        assert!(rules.matches("Jazz"));
        assert!(!rules.matches("Blues"));
    }

    #[test]
    fn smart_rules_exclude_overrides_include() {
        let rules = SmartPlaylistRules {
            include_genres: vec![],
            exclude_genres: vec!["Holiday".to_string()],
        };
        assert!(rules.matches("Jazz"));
        assert!(!rules.matches("Holiday"));
    }

    #[test]
    fn smart_rules_case_insensitive() {
        let rules = SmartPlaylistRules {
            include_genres: vec!["jazz".to_string()],
            exclude_genres: vec![],
        };
        assert!(rules.matches("Jazz"));
        assert!(rules.matches("JAZZ"));
    }
}
```

- [ ] **Step 2: Run tests**

```bash
cargo test -p dioxus_music_api types::tests
```
Expected: 4 tests pass.

- [ ] **Step 3: Commit**

```bash
git add packages/api/src/types.rs
git commit -m "feat(api): add playlist DTOs and SmartPlaylistRules with matching tests"
```

---

## Task 2: Playlist routes

**Files:**
- Create: `packages/api/src/routes/playlists.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/playlists.rs**

Create `packages/api/src/routes/playlists.rs`:

```rust
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{
        models::{Album, Artist, Image, NewPlaylist, NewPlaylistItem, Playlist, Track},
        schema::{albums, artists, images, playlist_items, playlists, tracks},
    },
    error::ApiError,
    routes::query,
    state::AppState,
    types::{
        BaseItemDto, CreatePlaylistRequest, CreateSmartPlaylistRequest, ItemsResult,
        SmartPlaylistRules, UpdatePlaylistRequest,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Playlists", post(create_playlist))
        .route("/Playlists/{playlist_id}", get(get_playlist).post(update_playlist).delete(delete_playlist))
        .route(
            "/Playlists/{playlist_id}/Items",
            get(get_playlist_items)
                .post(add_playlist_items)
                .delete(remove_playlist_items),
        )
        .route(
            "/Playlists/{playlist_id}/Items/{item_id}/Move/{new_index}",
            post(move_playlist_item),
        )
        .route("/Playlists/{item_id}/InstantMix", get(instant_mix_from_playlist))
        // Custom smart playlist routes
        .route("/custom/playlists/smart", post(create_smart_playlist))
        .route("/custom/playlists/{id}/rules", post(update_smart_rules))
}

fn playlist_to_dto(playlist: &Playlist, server_id: Uuid) -> BaseItemDto {
    BaseItemDto {
        id: playlist.id,
        name: playlist.name.clone(),
        sort_name: Some(playlist.name.to_ascii_lowercase()),
        item_type: "Playlist".to_string(),
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
        image_tags: None,
        user_data: None,
        date_created: Some(playlist.updated_at),
    }
}

/// POST /Playlists
async fn create_playlist(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreatePlaylistRequest>,
) -> Result<Json<BaseItemDto>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let new_playlist = NewPlaylist {
        id: Uuid::new_v4(),
        name: body.name.clone(),
        overview: None,
        is_smart: false,
        user_id: Some(auth.user.id),
    };

    let playlist: Playlist = diesel::insert_into(playlists::table)
        .values(&new_playlist)
        .get_result(&mut conn)
        .await?;

    // Add initial items if provided.
    if let Some(ids) = body.ids {
        for (pos, track_id) in ids.iter().enumerate() {
            let item = NewPlaylistItem {
                id: Uuid::new_v4(),
                playlist_id: playlist.id,
                item_id: *track_id,
                position: pos as i32,
            };
            diesel::insert_into(playlist_items::table)
                .values(&item)
                .on_conflict((playlist_items::playlist_id, playlist_items::item_id))
                .do_nothing()
                .execute(&mut conn)
                .await
                .ok();
        }
    }

    Ok(Json(playlist_to_dto(&playlist, state.server_id)))
}

/// GET /Playlists/{playlistId}
async fn get_playlist(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
) -> Result<Json<BaseItemDto>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let playlist: Playlist = playlists::table
        .filter(playlists::id.eq(playlist_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Playlist not found".to_string()))?;
    Ok(Json(playlist_to_dto(&playlist, state.server_id)))
}

/// POST /Playlists/{playlistId} — update name or overview
async fn update_playlist(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Json(body): Json<UpdatePlaylistRequest>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    if let Some(name) = body.name {
        diesel::update(playlists::table.filter(playlists::id.eq(playlist_id)))
            .set((playlists::name.eq(name), playlists::updated_at.eq(Utc::now())))
            .execute(&mut conn)
            .await?;
    }
    if let Some(overview) = body.overview {
        diesel::update(playlists::table.filter(playlists::id.eq(playlist_id)))
            .set(playlists::overview.eq(Some(overview)))
            .execute(&mut conn)
            .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /Playlists/{playlistId}
async fn delete_playlist(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(playlists::table.filter(playlists::id.eq(playlist_id)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaylistItemsQuery {
    pub start_index: Option<i64>,
    pub limit: Option<i64>,
    pub user_id: Option<Uuid>,
}

/// GET /Playlists/{playlistId}/Items
async fn get_playlist_items(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Query(params): Query<PlaylistItemsQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(500).min(1000);
    let start = params.start_index.unwrap_or(0);

    let playlist: Playlist = playlists::table
        .filter(playlists::id.eq(playlist_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Playlist not found".to_string()))?;

    let track_rows: Vec<(Track, Artist, Option<Album>)> = if playlist.is_smart {
        // Smart playlist: evaluate genre rules from overview JSON.
        let rules: SmartPlaylistRules = playlist
            .overview
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(SmartPlaylistRules {
                include_genres: vec![],
                exclude_genres: vec![],
            });

        let mut q = tracks::table
            .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
            .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
            .into_boxed();

        if !rules.include_genres.is_empty() {
            q = q.filter(tracks::genre.eq_any(&rules.include_genres));
        }
        if !rules.exclude_genres.is_empty() {
            q = q.filter(tracks::genre.ne_all(&rules.exclude_genres));
        }

        q.select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
            .order((artists::sort_name.asc(), albums::sort_title.asc().nulls_last(), tracks::disc_number.asc(), tracks::track_number.asc().nulls_last()))
            .offset(start)
            .limit(limit)
            .load(&mut conn)
            .await?
    } else {
        // Manual playlist: join through playlist_items in position order.
        playlist_items::table
            .filter(playlist_items::playlist_id.eq(playlist_id))
            .inner_join(tracks::table.on(playlist_items::item_id.eq(tracks::id)))
            .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
            .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
            .order(playlist_items::position.asc())
            .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
            .offset(start)
            .limit(limit)
            .load(&mut conn)
            .await?
    };

    let total = track_rows.len() as i64;
    let mut items = Vec::with_capacity(track_rows.len());
    for (track, artist, album) in &track_rows {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()?;
        items.push(query::track_to_dto(
            track, &artist, album.as_ref(),
            album.as_ref().map(|_| &artist), image.as_ref(), None, state.server_id,
        ));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: start as i32 }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddItemsQuery {
    pub ids: String,  // comma-delimited UUIDs
    pub user_id: Option<Uuid>,
}

/// POST /Playlists/{playlistId}/Items
async fn add_playlist_items(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Query(params): Query<AddItemsQuery>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let playlist: Playlist = playlists::table.filter(playlists::id.eq(playlist_id)).first(&mut conn)
        .await.optional()?.ok_or_else(|| ApiError::NotFound("Playlist not found".to_string()))?;

    if playlist.is_smart {
        return Err(ApiError::BadRequest(
            "Cannot add items to a smart playlist".to_string(),
        ));
    }

    // Determine next position.
    let max_pos: Option<i32> = playlist_items::table
        .filter(playlist_items::playlist_id.eq(playlist_id))
        .select(diesel::dsl::max(playlist_items::position))
        .first(&mut conn)
        .await?;
    let mut next_pos = max_pos.map(|p| p + 1).unwrap_or(0);

    let ids: Vec<Uuid> = params
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    for track_id in &ids {
        let item = NewPlaylistItem {
            id: Uuid::new_v4(),
            playlist_id,
            item_id: *track_id,
            position: next_pos,
        };
        diesel::insert_into(playlist_items::table)
            .values(&item)
            .on_conflict((playlist_items::playlist_id, playlist_items::item_id))
            .do_nothing()
            .execute(&mut conn)
            .await?;
        next_pos += 1;
    }

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveItemsQuery {
    pub entry_ids: String,  // comma-delimited entry UUIDs (playlist_items.id)
}

/// DELETE /Playlists/{playlistId}/Items
async fn remove_playlist_items(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Query(params): Query<RemoveItemsQuery>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let playlist: Playlist = playlists::table.filter(playlists::id.eq(playlist_id)).first(&mut conn)
        .await.optional()?.ok_or_else(|| ApiError::NotFound("Playlist not found".to_string()))?;

    if playlist.is_smart {
        return Err(ApiError::BadRequest(
            "Cannot remove items from a smart playlist".to_string(),
        ));
    }

    let entry_ids: Vec<Uuid> = params
        .entry_ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    diesel::delete(
        playlist_items::table.filter(
            playlist_items::playlist_id.eq(playlist_id)
                .and(playlist_items::id.eq_any(&entry_ids)),
        ),
    )
    .execute(&mut conn)
    .await?;

    // Recompact positions.
    let remaining: Vec<Uuid> = playlist_items::table
        .filter(playlist_items::playlist_id.eq(playlist_id))
        .order(playlist_items::position.asc())
        .select(playlist_items::id)
        .load(&mut conn)
        .await?;
    for (i, entry_id) in remaining.iter().enumerate() {
        diesel::update(playlist_items::table.filter(playlist_items::id.eq(entry_id)))
            .set(playlist_items::position.eq(i as i32))
            .execute(&mut conn)
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /Playlists/{playlistId}/Items/{itemId}/Move/{newIndex}
async fn move_playlist_item(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path((playlist_id, item_id, new_index)): Path<(Uuid, Uuid, i32)>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Load all items in order.
    let mut entries: Vec<(Uuid, Uuid, i32)> = playlist_items::table
        .filter(playlist_items::playlist_id.eq(playlist_id))
        .order(playlist_items::position.asc())
        .select((playlist_items::id, playlist_items::item_id, playlist_items::position))
        .load(&mut conn)
        .await?;

    // Find current position of the item being moved.
    let Some(current_pos) = entries.iter().position(|(_, tid, _)| *tid == item_id) else {
        return Err(ApiError::NotFound("Item not in playlist".to_string()));
    };

    let entry = entries.remove(current_pos);
    let clamped = (new_index as usize).min(entries.len());
    entries.insert(clamped, entry);

    // Rewrite positions.
    for (i, (entry_id, _, _)) in entries.iter().enumerate() {
        diesel::update(playlist_items::table.filter(playlist_items::id.eq(entry_id)))
            .set(playlist_items::position.eq(i as i32))
            .execute(&mut conn)
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// GET /Playlists/{itemId}/InstantMix
async fn instant_mix_from_playlist(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Query(params): Query<PlaylistItemsQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(200);

    // Get genres represented in this playlist.
    let genres: Vec<String> = playlist_items::table
        .filter(playlist_items::playlist_id.eq(playlist_id))
        .inner_join(tracks::table.on(playlist_items::item_id.eq(tracks::id)))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre)
        .distinct()
        .load(&mut conn)
        .await?;

    let rows: Vec<(Track, Artist, Option<Album>)> = tracks::table
        .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
        .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
        .filter(tracks::genre.eq_any(&genres))
        .select((Track::as_select(), Artist::as_select(), Option::<Album>::as_select()))
        .limit(limit)
        .load(&mut conn)
        .await?;

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

// ── Custom smart playlist routes ──────────────────────────────────────────

/// POST /custom/playlists/smart
async fn create_smart_playlist(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateSmartPlaylistRequest>,
) -> Result<Json<BaseItemDto>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let rules_json = serde_json::to_string(&body.rules)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let new_playlist = NewPlaylist {
        id: Uuid::new_v4(),
        name: body.name,
        overview: Some(rules_json),
        is_smart: true,
        user_id: body.user_id.or(Some(auth.user.id)),
    };

    let playlist: Playlist = diesel::insert_into(playlists::table)
        .values(&new_playlist)
        .get_result(&mut conn)
        .await?;

    Ok(Json(playlist_to_dto(&playlist, state.server_id)))
}

/// POST /custom/playlists/{id}/rules
async fn update_smart_rules(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Json(rules): Json<SmartPlaylistRules>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let rules_json = serde_json::to_string(&rules)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    diesel::update(
        playlists::table
            .filter(playlists::id.eq(playlist_id))
            .filter(playlists::is_smart.eq(true)),
    )
    .set((
        playlists::overview.eq(Some(rules_json)),
        playlists::updated_at.eq(Utc::now()),
    ))
    .execute(&mut conn)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
```

- [ ] **Step 2: Update routes/mod.rs**

Add `pub mod playlists;` and `.merge(playlists::router())` to `create_router`.

- [ ] **Step 3: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 4: Smoke-test playlists**

```bash
TOKEN="<token>"

# Create a playlist
curl -s -X POST "http://localhost:8080/Playlists" \
  -H "Authorization: MediaBrowser Token=\"$TOKEN\"" \
  -H "Content-Type: application/json" \
  -d '{"Name":"My Test Playlist"}' | jq '.Id'

# List its items (empty)
PLAYLIST_ID="<id from above>"
curl -s "http://localhost:8080/Playlists/$PLAYLIST_ID/Items" \
  -H "Authorization: MediaBrowser Token=\"$TOKEN\"" | jq '.TotalRecordCount'
# Expected: 0
```

- [ ] **Step 5: Commit**

```bash
git add packages/api/src/routes/playlists.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Playlists CRUD + smart playlist custom routes"
```

---

## Task 3: User data routes (favorites, ratings, played state)

**Files:**
- Create: `packages/api/src/routes/user_data.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/user_data.rs**

Create `packages/api/src/routes/user_data.rs`:

```rust
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, post},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{
        models::UserData,
        schema::user_data,
    },
    error::ApiError,
    state::AppState,
    types::UserItemDataDto,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/UserFavoriteItems/{item_id}", post(mark_favorite).delete(unmark_favorite))
        .route("/UserItems/{item_id}/Rating", post(rate_item))
        .route("/UserPlayedItems/{item_id}", post(mark_played).delete(mark_unplayed))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserQuery {
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RatingQuery {
    pub likes: Option<bool>,
    pub user_id: Option<Uuid>,
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

/// Upsert a user_data row. Returns the current state.
async fn upsert_user_data(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: Uuid,
    item_id: Uuid,
    item_type: &str,
    f: impl FnOnce(&mut UserData),
) -> Result<UserData, diesel::result::Error> {
    use crate::db::models::NewUserData;

    // Insert default row if not exists.
    diesel::insert_into(user_data::table)
        .values(&NewUserData {
            user_id,
            item_id,
            item_type: item_type.to_string(),
        })
        .on_conflict((user_data::user_id, user_data::item_id))
        .do_nothing()
        .execute(conn)
        .await?;

    let mut ud: UserData = user_data::table
        .filter(user_data::user_id.eq(user_id))
        .filter(user_data::item_id.eq(item_id))
        .first(conn)
        .await?;

    f(&mut ud);

    diesel::update(
        user_data::table
            .filter(user_data::user_id.eq(user_id))
            .filter(user_data::item_id.eq(item_id)),
    )
    .set((
        user_data::is_favorite.eq(ud.is_favorite),
        user_data::likes.eq(ud.likes),
        user_data::play_count.eq(ud.play_count),
        user_data::last_played_date.eq(ud.last_played_date),
        user_data::played.eq(ud.played),
        user_data::playback_position_ticks.eq(ud.playback_position_ticks),
    ))
    .execute(conn)
    .await?;

    Ok(ud)
}

/// POST /UserFavoriteItems/{itemId}
async fn mark_favorite(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<UserQuery>,
) -> Result<Json<UserItemDataDto>, ApiError> {
    let user_id = params.user_id.unwrap_or(auth.user.id);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.is_favorite = true;
    }).await?;
    Ok(Json(user_data_to_dto(&ud)))
}

/// DELETE /UserFavoriteItems/{itemId}
async fn unmark_favorite(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<UserQuery>,
) -> Result<Json<UserItemDataDto>, ApiError> {
    let user_id = params.user_id.unwrap_or(auth.user.id);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.is_favorite = false;
    }).await?;
    Ok(Json(user_data_to_dto(&ud)))
}

/// POST /UserItems/{itemId}/Rating?likes=true|false
async fn rate_item(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<RatingQuery>,
) -> Result<Json<UserItemDataDto>, ApiError> {
    let user_id = params.user_id.unwrap_or(auth.user.id);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.likes = params.likes; // None = clear rating
    }).await?;
    Ok(Json(user_data_to_dto(&ud)))
}

/// POST /UserPlayedItems/{itemId}
async fn mark_played(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<UserQuery>,
) -> Result<Json<UserItemDataDto>, ApiError> {
    let user_id = params.user_id.unwrap_or(auth.user.id);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.played = true;
        d.play_count += 1;
        d.last_played_date = Some(Utc::now());
    }).await?;
    Ok(Json(user_data_to_dto(&ud)))
}

/// DELETE /UserPlayedItems/{itemId}
async fn mark_unplayed(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
    Query(params): Query<UserQuery>,
) -> Result<Json<UserItemDataDto>, ApiError> {
    let user_id = params.user_id.unwrap_or(auth.user.id);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.played = false;
    }).await?;
    Ok(Json(user_data_to_dto(&ud)))
}
```

- [ ] **Step 2: Update routes/mod.rs**

Add `pub mod user_data;` and `.merge(user_data::router())`.

- [ ] **Step 3: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/user_data.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /UserFavoriteItems, /UserItems/Rating, /UserPlayedItems routes"
```

---

## Task 4: Playback session routes

**Files:**
- Create: `packages/api/src/routes/sessions.rs`
- Modify: `packages/api/src/routes/mod.rs`

- [ ] **Step 1: Create routes/sessions.rs**

Create `packages/api/src/routes/sessions.rs`:

```rust
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::post,
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::NewUserData, schema::user_data},
    error::ApiError,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Sessions/Playing", post(report_playback_start))
        .route("/Sessions/Playing/Progress", post(report_playback_progress))
        .route("/Sessions/Playing/Stopped", post(report_playback_stopped))
        .route("/Sessions/Playing/Ping", post(ping_session))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackStartInfo {
    pub item_id: Option<Uuid>,
    pub position_ticks: Option<i64>,
    pub play_session_id: Option<String>,
    pub is_paused: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressQuery {
    pub item_id: Option<Uuid>,
    pub position_ticks: Option<i64>,
    pub play_session_id: Option<String>,
    pub is_paused: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackStopInfo {
    pub item_id: Option<Uuid>,
    pub position_ticks: Option<i64>,
    pub play_session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingQuery {
    pub play_session_id: Option<String>,
}

async fn ensure_user_data_row(
    conn: &mut diesel_async::AsyncPgConnection,
    user_id: Uuid,
    item_id: Uuid,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(user_data::table)
        .values(&NewUserData {
            user_id,
            item_id,
            item_type: "Audio".to_string(),
        })
        .on_conflict((user_data::user_id, user_data::item_id))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(())
}

/// POST /Sessions/Playing — record playback start, store position
async fn report_playback_start(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<PlaybackStartInfo>,
) -> Result<StatusCode, ApiError> {
    let Some(item_id) = body.item_id else {
        return Ok(StatusCode::NO_CONTENT);
    };
    let ticks = body.position_ticks.unwrap_or(0);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    ensure_user_data_row(&mut conn, auth.user.id, item_id).await?;
    diesel::update(
        user_data::table
            .filter(user_data::user_id.eq(auth.user.id))
            .filter(user_data::item_id.eq(item_id)),
    )
    .set(user_data::playback_position_ticks.eq(ticks))
    .execute(&mut conn)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /Sessions/Playing/Progress — update resume position
async fn report_playback_progress(
    auth: AuthUser,
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ProgressQuery>,
) -> Result<StatusCode, ApiError> {
    let Some(item_id) = params.item_id else {
        return Ok(StatusCode::NO_CONTENT);
    };
    let ticks = params.position_ticks.unwrap_or(0);
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    ensure_user_data_row(&mut conn, auth.user.id, item_id).await?;
    diesel::update(
        user_data::table
            .filter(user_data::user_id.eq(auth.user.id))
            .filter(user_data::item_id.eq(item_id)),
    )
    .set(user_data::playback_position_ticks.eq(ticks))
    .execute(&mut conn)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /Sessions/Playing/Stopped — increment play count, record last played
async fn report_playback_stopped(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<PlaybackStopInfo>,
) -> Result<StatusCode, ApiError> {
    let Some(item_id) = body.item_id else {
        return Ok(StatusCode::NO_CONTENT);
    };
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    ensure_user_data_row(&mut conn, auth.user.id, item_id).await?;
    diesel::update(
        user_data::table
            .filter(user_data::user_id.eq(auth.user.id))
            .filter(user_data::item_id.eq(item_id)),
    )
    .set((
        user_data::play_count.eq(user_data::play_count + 1),
        user_data::played.eq(true),
        user_data::last_played_date.eq(Some(Utc::now())),
        user_data::playback_position_ticks.eq(body.position_ticks.unwrap_or(0)),
    ))
    .execute(&mut conn)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /Sessions/Playing/Ping — keep-alive, no-op beyond updating token last_seen_at
/// (last_seen_at is already updated by AuthUser extractor on every request)
async fn ping_session(
    _auth: AuthUser,
    _state: State<AppState>,
    axum::extract::Query(_params): axum::extract::Query<PingQuery>,
) -> StatusCode {
    StatusCode::NO_CONTENT
}
```

- [ ] **Step 2: Update routes/mod.rs (final state)**

Replace `packages/api/src/routes/mod.rs`:

```rust
pub mod albums;
pub mod artists;
pub mod audio;
pub mod custom;
pub mod genres;
pub mod images;
pub mod items;
pub mod playlists;
pub mod query;
pub mod search;
pub mod sessions;
pub mod user_data;
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
        .merge(playlists::router())
        .merge(user_data::router())
        .merge(sessions::router())
        .merge(custom::router())
        .with_state(state)
}
```

- [ ] **Step 3: Build**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 4: Commit**

```bash
git add packages/api/src/routes/sessions.rs packages/api/src/routes/mod.rs
git commit -m "feat(api): add /Sessions/Playing start/progress/stopped/ping routes — backend complete"
```

---

## Task 5: Add reqwest to ui crate and create api_client module

**Files:**
- Modify: `packages/ui/Cargo.toml`
- Modify: `packages/api/src/types.rs` (make SmartPlaylistRules pub to all, already is)
- Create: `packages/ui/src/api_client.rs`

- [ ] **Step 1: Add reqwest to ui Cargo.toml**

In `packages/ui/Cargo.toml`, add:

```toml
reqwest = { workspace = true, features = ["json"] }
uuid = { workspace = true, features = ["serde"] }
```

If a `[features]` section exists for `web`/`server`, `reqwest` does NOT need to be feature-gated — `reqwest` supports WASM via its `"wasm-client"` feature detection at build time.

- [ ] **Step 2: Verify reqwest workspace dep has required features**

In root `Cargo.toml`, ensure:
```toml
reqwest = { version = "0", features = ["json"] }
```

- [ ] **Step 3: Create api_client.rs**

Create `packages/ui/src/api_client.rs`:

```rust
//! HTTP client for the Jellyfin-compatible API.
//! Works on both native (reqwest with hyper) and WASM (reqwest with fetch).

use reqwest::Client;
use uuid::Uuid;
use dioxus_music_api::types::{
    AuthenticationResult, BaseItemDto, CreatePlaylistRequest, CreateSmartPlaylistRequest,
    ItemsResult, SmartPlaylistRules, UpdatePlaylistRequest, UserItemDataDto,
};

/// Shared API client. Holds the base URL and current auth token.
#[derive(Clone, Debug)]
pub struct ApiClient {
    pub client: Client,
    pub base_url: String,
    pub token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            token: None,
        }
    }

    fn auth_header(&self) -> String {
        match &self.token {
            Some(t) => format!(
                r#"MediaBrowser Client="DioxusMusic", Device="Web", DeviceId="web-1", Version="1.0", Token="{t}""#
            ),
            None => r#"MediaBrowser Client="DioxusMusic", Device="Web", DeviceId="web-1", Version="1.0""#
                .to_string(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    // ── Auth ──────────────────────────────────────────────────────────────

    pub async fn authenticate(&mut self, username: &str, password: &str)
        -> Result<AuthenticationResult, reqwest::Error>
    {
        let result: AuthenticationResult = self.client
            .post(self.url("/Users/AuthenticateByName"))
            .header("Authorization", self.auth_header())
            .json(&serde_json::json!({ "Username": username, "Pw": password }))
            .send()
            .await?
            .json()
            .await?;
        self.token = Some(result.access_token.clone());
        Ok(result)
    }

    // ── Library ───────────────────────────────────────────────────────────

    pub async fn get_albums(&self, parent_id: Option<Uuid>)
        -> Result<ItemsResult, reqwest::Error>
    {
        let mut url = self.url("/Items?IncludeItemTypes=MusicAlbum&SortBy=SortName&SortOrder=Ascending");
        if let Some(id) = parent_id {
            url.push_str(&format!("&ParentId={id}"));
        }
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn get_album_tracks(&self, album_id: Uuid)
        -> Result<ItemsResult, reqwest::Error>
    {
        let url = self.url(&format!(
            "/Items?IncludeItemTypes=Audio&ParentId={album_id}&SortBy=ParentIndexNumber,IndexNumber&SortOrder=Ascending"
        ));
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn get_artists(&self)
        -> Result<ItemsResult, reqwest::Error>
    {
        let url = self.url("/Artists/AlbumArtists?SortBy=SortName&SortOrder=Ascending");
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn get_item(&self, item_id: Uuid)
        -> Result<BaseItemDto, reqwest::Error>
    {
        let url = self.url(&format!("/Items/{item_id}"));
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn get_genres(&self)
        -> Result<ItemsResult, reqwest::Error>
    {
        let url = self.url("/MusicGenres");
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn search(&self, term: &str, limit: u32)
        -> Result<crate::types::SearchHintsResult, reqwest::Error>
    {
        let url = self.url(&format!(
            "/Search/Hints?SearchTerm={}&Limit={limit}",
            urlencoding::encode(term)
        ));
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    // ── Playlists ─────────────────────────────────────────────────────────

    pub async fn get_playlists(&self)
        -> Result<ItemsResult, reqwest::Error>
    {
        let url = self.url("/Items?IncludeItemTypes=Playlist");
        self.client.get(&url)
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn get_playlist(&self, id: Uuid)
        -> Result<BaseItemDto, reqwest::Error>
    {
        self.client.get(self.url(&format!("/Playlists/{id}")))
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn get_playlist_items(&self, id: Uuid)
        -> Result<ItemsResult, reqwest::Error>
    {
        self.client.get(self.url(&format!("/Playlists/{id}/Items")))
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn create_playlist(&self, name: &str)
        -> Result<BaseItemDto, reqwest::Error>
    {
        self.client.post(self.url("/Playlists"))
            .header("Authorization", self.auth_header())
            .json(&CreatePlaylistRequest {
                name: name.to_string(),
                ids: None,
                user_id: None,
                media_type: Some("Audio".to_string()),
            })
            .send().await?.json().await
    }

    pub async fn create_smart_playlist(&self, name: &str, rules: SmartPlaylistRules)
        -> Result<BaseItemDto, reqwest::Error>
    {
        self.client.post(self.url("/custom/playlists/smart"))
            .header("Authorization", self.auth_header())
            .json(&CreateSmartPlaylistRequest { name: name.to_string(), rules, user_id: None })
            .send().await?.json().await
    }

    pub async fn update_playlist(&self, id: Uuid, name: Option<String>)
        -> Result<(), reqwest::Error>
    {
        self.client.post(self.url(&format!("/Playlists/{id}")))
            .header("Authorization", self.auth_header())
            .json(&UpdatePlaylistRequest { name, overview: None })
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_playlist(&self, id: Uuid) -> Result<(), reqwest::Error> {
        self.client.delete(self.url(&format!("/Playlists/{id}")))
            .header("Authorization", self.auth_header())
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn add_to_playlist(&self, playlist_id: Uuid, track_ids: &[Uuid])
        -> Result<(), reqwest::Error>
    {
        let ids = track_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
        let url = self.url(&format!("/Playlists/{playlist_id}/Items?Ids={ids}"));
        self.client.post(&url)
            .header("Authorization", self.auth_header())
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn remove_from_playlist(&self, playlist_id: Uuid, entry_ids: &[Uuid])
        -> Result<(), reqwest::Error>
    {
        let ids = entry_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
        let url = self.url(&format!("/Playlists/{playlist_id}/Items?EntryIds={ids}"));
        self.client.delete(&url)
            .header("Authorization", self.auth_header())
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn update_smart_rules(&self, playlist_id: Uuid, rules: SmartPlaylistRules)
        -> Result<(), reqwest::Error>
    {
        self.client.post(self.url(&format!("/custom/playlists/{playlist_id}/rules")))
            .header("Authorization", self.auth_header())
            .json(&rules)
            .send().await?.error_for_status()?;
        Ok(())
    }

    // ── User data ─────────────────────────────────────────────────────────

    pub async fn mark_favorite(&self, item_id: Uuid) -> Result<UserItemDataDto, reqwest::Error> {
        self.client.post(self.url(&format!("/UserFavoriteItems/{item_id}")))
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn unmark_favorite(&self, item_id: Uuid) -> Result<UserItemDataDto, reqwest::Error> {
        self.client.delete(self.url(&format!("/UserFavoriteItems/{item_id}")))
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    pub async fn mark_played(&self, item_id: Uuid) -> Result<UserItemDataDto, reqwest::Error> {
        self.client.post(self.url(&format!("/UserPlayedItems/{item_id}")))
            .header("Authorization", self.auth_header())
            .send().await?.json().await
    }

    // ── Streaming ─────────────────────────────────────────────────────────

    pub fn stream_url(&self, track_id: Uuid) -> String {
        self.url(&format!("/Audio/{track_id}/stream"))
    }

    pub fn image_url(&self, item_id: Uuid, image_type: &str) -> String {
        match &self.token {
            Some(t) => self.url(&format!(
                "/Items/{item_id}/Images/{image_type}?api_key={t}"
            )),
            None => self.url(&format!("/Items/{item_id}/Images/{image_type}")),
        }
    }
}
```

- [ ] **Step 4: Add urlencoding dependency**

In root `Cargo.toml` workspace deps:
```toml
urlencoding = "2"
```

In `packages/ui/Cargo.toml`:
```toml
urlencoding = { workspace = true }
```

- [ ] **Step 5: Expose api_client from ui/src/lib.rs**

In `packages/ui/src/lib.rs`, add:
```rust
pub mod api_client;
```

- [ ] **Step 6: Verify compilation**

```bash
cargo check -p dioxus_music_ui --features web
cargo check -p dioxus_music_ui --features server
```
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add packages/ui/src/api_client.rs packages/ui/src/lib.rs packages/ui/Cargo.toml Cargo.toml
git commit -m "feat(ui): add ApiClient HTTP wrapper for Jellyfin API"
```

---

## Task 6: Provide ApiClient via context + auth flow

**Files:**
- Modify: `packages/web/src/main.rs`

The `AppLayout` component provides `ApiClient` via context. On first render it reads the token from `localStorage` (or prompts to authenticate).

- [ ] **Step 1: Update AppLayout to provide ApiClient context**

In `packages/web/src/main.rs`, replace the `AppLayout` component:

```rust
use dioxus_music_ui::api_client::ApiClient;

#[component]
fn AppLayout() -> Element {
    use_context_provider(|| {
        // In production, read base_url from window.location.origin or env.
        // For local dev, the Dioxus dev server serves both frontend and API on the same origin.
        let base_url = "".to_string(); // empty = same-origin
        ApiClient::new(base_url)
    });
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

- [ ] **Step 2: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 3: Commit**

```bash
git add packages/web/src/main.rs
git commit -m "feat(web): provide ApiClient via Dioxus context in AppLayout"
```

---

## Task 7: Migrate Library view

**Files:**
- Modify: `packages/web/src/views/library.rs`

Replace the "Coming soon" stub with a real implementation using `ApiClient`.

- [ ] **Step 1: Rewrite library.rs**

Replace `packages/web/src/views/library.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_ui::api_client::ApiClient;
use dioxus_music_api::types::BaseItemDto;

use crate::Route;

#[component]
pub fn Library() -> Element {
    let client = use_context::<ApiClient>();
    let albums = use_resource(move || {
        let client = client.clone();
        async move { client.get_albums(None).await.ok() }
    });

    rsx! {
        div { class: "library",
            match &*albums.read() {
                Some(Some(result)) => rsx! {
                    div { class: "album-grid",
                        for album in &result.items {
                            AlbumCard { album: album.clone() }
                        }
                    }
                },
                Some(None) => rsx! { p { "Failed to load library." } },
                None => rsx! { p { "Loading…" } },
            }
        }
    }
}

#[component]
fn AlbumCard(album: BaseItemDto) -> Element {
    let client = use_context::<ApiClient>();
    let image_url = album.image_tags
        .as_ref()
        .and_then(|t| t.get("Primary"))
        .map(|_| client.image_url(album.id, "Primary"));

    rsx! {
        Link {
            class: "album-card",
            to: Route::AlbumDetail { name: album.name.clone() },
            if let Some(url) = image_url {
                img { class: "album-card__art", src: url }
            } else {
                div { class: "album-card__art album-card__art--placeholder" }
            }
            div { class: "album-card__info",
                p { class: "album-card__title", "{album.name}" }
                if let Some(artist) = &album.album_artist {
                    p { class: "album-card__artist", "{artist}" }
                }
                if let Some(year) = album.production_year {
                    p { class: "album-card__year", "{year}" }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 3: Visual test**

Start the server and navigate to `/`. Expect the album grid to render with album cards (or "Loading…" briefly).

- [ ] **Step 4: Commit**

```bash
git add packages/web/src/views/library.rs
git commit -m "feat(web): migrate Library view to Jellyfin /Items API"
```

---

## Task 8: Migrate AlbumDetail view

**Files:**
- Modify: `packages/web/src/views/album_detail.rs`

- [ ] **Step 1: Rewrite album_detail.rs**

Replace `packages/web/src/views/album_detail.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_api::types::BaseItemDto;
use dioxus_music_ui::{TrackList, api_client::ApiClient, player_state::use_player_state};

#[component]
pub fn AlbumDetail(name: String) -> Element {
    let client = use_context::<ApiClient>();

    // Fetch all albums, find the one matching this name.
    let albums = use_resource(move || {
        let client = client.clone();
        let name = name.clone();
        async move {
            let result = client.get_albums(None).await.ok()?;
            result.items.into_iter().find(|a| a.name == name)
        }
    });

    let album = albums.read();
    let album = match &*album {
        Some(Some(a)) => a.clone(),
        Some(None) => return rsx! { p { "Album not found." } },
        None => return rsx! { p { "Loading…" } },
    };

    let album_id = album.id;
    let client2 = use_context::<ApiClient>();
    let tracks = use_resource(move || {
        let client = client2.clone();
        async move { client.get_album_tracks(album_id).await.ok() }
    });

    let image_url = {
        let client3 = use_context::<ApiClient>();
        album.image_tags
            .as_ref()
            .and_then(|t| t.get("Primary"))
            .map(|_| client3.image_url(album.id, "Primary"))
    };

    rsx! {
        div { class: "album-detail",
            div { class: "album-detail__header",
                if let Some(url) = image_url {
                    img { class: "album-detail__art", src: url }
                }
                div { class: "album-detail__meta",
                    h1 { "{album.name}" }
                    if let Some(artist) = &album.album_artist {
                        p { "{artist}" }
                    }
                    if let Some(year) = album.production_year {
                        p { "{year}" }
                    }
                }
            }
            match &*tracks.read() {
                Some(Some(result)) => rsx! {
                    TrackList { tracks: result.items.clone() }
                },
                Some(None) => rsx! { p { "Failed to load tracks." } },
                None => rsx! { p { "Loading tracks…" } },
            }
        }
    }
}
```

- [ ] **Step 2: Update TrackList props in ui crate**

`TrackList` currently takes `Vec<TrackSummary>` (old type). Update its props to accept `Vec<BaseItemDto>` from `dioxus_music_api::types`. In `packages/ui/src/components/track_list.rs`, change the prop type:

```rust
use dioxus_music_api::types::BaseItemDto;

#[derive(Props, Clone, PartialEq)]
pub struct TrackListProps {
    pub tracks: Vec<BaseItemDto>,
    pub show_download_status: Option<bool>,
}
```

Update the rendering inside `TrackList` to use `BaseItemDto` field names:
- `track.name` instead of `track.title`
- `track.artists.as_ref().and_then(|a| a.first())` instead of `track.artist`
- `track.album.as_deref().unwrap_or("")` instead of `track.album`
- `track.run_time_ticks.map(|t| t / 10_000_000)` for duration in seconds

- [ ] **Step 3: Update PlayerState to use BaseItemDto**

In `packages/ui/src/player_state.rs`, change `TrackSummary` references to `BaseItemDto`. The `stream_url` for the audio element is now `client.stream_url(track.id)`. Update `play_track` and related methods accordingly.

- [ ] **Step 4: Build and verify**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 5: Commit**

```bash
git add packages/web/src/views/album_detail.rs packages/ui/src/
git commit -m "feat(web): migrate AlbumDetail view and update TrackList + PlayerState to BaseItemDto"
```

---

## Task 9: Migrate Artists view

**Files:**
- Modify: `packages/web/src/views/artists.rs`

- [ ] **Step 1: Rewrite artists.rs**

Replace `packages/web/src/views/artists.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_api::types::BaseItemDto;
use dioxus_music_ui::api_client::ApiClient;

use crate::Route;

#[component]
pub fn Artists() -> Element {
    let client = use_context::<ApiClient>();
    let artists = use_resource(move || {
        let client = client.clone();
        async move { client.get_artists().await.ok() }
    });

    rsx! {
        div { class: "artists",
            match &*artists.read() {
                Some(Some(result)) => rsx! {
                    div { class: "artist-list",
                        for artist in &result.items {
                            ArtistRow { artist: artist.clone() }
                        }
                    }
                },
                Some(None) => rsx! { p { "Failed to load artists." } },
                None => rsx! { p { "Loading…" } },
            }
        }
    }
}

#[component]
fn ArtistRow(artist: BaseItemDto) -> Element {
    let client = use_context::<ApiClient>();
    let image_url = artist.image_tags
        .as_ref()
        .and_then(|t| t.get("Primary"))
        .map(|_| client.image_url(artist.id, "Primary"));

    rsx! {
        div { class: "artist-row",
            if let Some(url) = image_url {
                img { class: "artist-row__art", src: url }
            }
            span { class: "artist-row__name", "{artist.name}" }
        }
    }
}
```

- [ ] **Step 2: Build**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 3: Commit**

```bash
git add packages/web/src/views/artists.rs
git commit -m "feat(web): migrate Artists view to /Artists/AlbumArtists API"
```

---

## Task 10: Migrate Playlists and PlaylistView

**Files:**
- Modify: `packages/web/src/views/playlists.rs`
- Modify: `packages/web/src/views/playlist_view.rs`

- [ ] **Step 1: Rewrite playlists.rs**

Replace `packages/web/src/views/playlists.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_api::types::{BaseItemDto, SmartPlaylistRules};
use dioxus_music_ui::api_client::ApiClient;
use uuid::Uuid;

use crate::Route;

#[component]
pub fn Playlists() -> Element {
    rsx! {
        div { class: "playlists-page",
            PlaylistSidebarSection {}
        }
    }
}

#[component]
pub fn PlaylistSidebarSection() -> Element {
    let client = use_context::<ApiClient>();
    let playlists = use_resource(move || {
        let client = client.clone();
        async move { client.get_playlists().await.ok() }
    });
    let mut show_create = use_signal(|| false);

    rsx! {
        div { class: "playlist-sidebar",
            div { class: "playlist-sidebar__actions",
                button { onclick: move |_| show_create.set(true), "+" }
            }
            match &*playlists.read() {
                Some(Some(result)) => rsx! {
                    for p in &result.items {
                        Link {
                            class: "sidebar__nav-item",
                            to: Route::PlaylistView { id: p.id },
                            "{p.name}"
                        }
                    }
                },
                _ => rsx! {},
            }
            if show_create() {
                CreatePlaylistModal {
                    on_save: move |_| {
                        show_create.set(false);
                        playlists.restart();
                    },
                    on_cancel: move |_| show_create.set(false),
                }
            }
        }
    }
}

#[component]
fn CreatePlaylistModal(on_save: EventHandler<()>, on_cancel: EventHandler<()>) -> Element {
    let client = use_context::<ApiClient>();
    let mut name = use_signal(String::new);
    let mut is_smart = use_signal(|| false);
    let mut include_genres = use_signal(|| Vec::<String>::new());
    let mut exclude_genres = use_signal(|| Vec::<String>::new());

    let genres_resource = use_resource(move || {
        let client = client.clone();
        async move { client.get_genres().await.ok() }
    });

    rsx! {
        div { class: "modal",
            input {
                placeholder: "Playlist name",
                value: name(),
                oninput: move |e| name.set(e.value()),
            }
            label {
                input { r#type: "checkbox", checked: is_smart(), onchange: move |e| is_smart.set(e.checked()) }
                " Smart playlist"
            }
            if is_smart() {
                // Genre selector — simplified for brevity
                p { "Genre filters (advanced UI in future iteration)" }
            }
            div { class: "modal__actions",
                button {
                    onclick: move |_| {
                        let n = name();
                        let smart = is_smart();
                        let inc = include_genres();
                        let exc = exclude_genres();
                        let client = client.clone();
                        spawn(async move {
                            if smart {
                                let _ = client.create_smart_playlist(&n, SmartPlaylistRules {
                                    include_genres: inc,
                                    exclude_genres: exc,
                                }).await;
                            } else {
                                let _ = client.create_playlist(&n).await;
                            }
                        });
                        on_save.call(());
                    },
                    "Save"
                }
                button { onclick: move |_| on_cancel.call(()), "Cancel" }
            }
        }
    }
}
```

- [ ] **Step 2: Rewrite playlist_view.rs**

Replace `packages/web/src/views/playlist_view.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_ui::{TrackList, api_client::ApiClient};
use uuid::Uuid;

#[component]
pub fn PlaylistView(id: Uuid) -> Element {
    let client = use_context::<ApiClient>();

    let playlist = use_resource(move || {
        let client = client.clone();
        async move { client.get_playlist(id).await.ok() }
    });

    let tracks = use_resource(move || {
        let client = client.clone();
        async move { client.get_playlist_items(id).await.ok() }
    });

    rsx! {
        div { class: "playlist-view",
            match &*playlist.read() {
                Some(Some(p)) => rsx! { h1 { "{p.name}" } },
                _ => rsx! { h1 { "Playlist" } },
            }
            match &*tracks.read() {
                Some(Some(result)) => rsx! {
                    TrackList { tracks: result.items.clone() }
                },
                Some(None) => rsx! { p { "Failed to load tracks." } },
                None => rsx! { p { "Loading…" } },
            }
        }
    }
}
```

- [ ] **Step 3: Build**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 4: Commit**

```bash
git add packages/web/src/views/playlists.rs packages/web/src/views/playlist_view.rs
git commit -m "feat(web): migrate Playlists and PlaylistView to Jellyfin API"
```

---

## Task 11: Update PlayerState to report sessions + migrate NowPlaying

**Files:**
- Modify: `packages/ui/src/player_state.rs`
- Modify: `packages/web/src/views/now_playing.rs`

- [ ] **Step 1: Update PlayerState to report playback start/progress/stop**

In `packages/ui/src/player_state.rs`, after calling `document::eval` to start audio playback, also call the session reporting endpoints:

```rust
// After setting current_track in play_track():
if let Some(track) = &state.current_track {
    let client = state.api_client.clone();
    let track_id = track.id;
    spawn(async move {
        let _ = client.client
            .post(format!("{}/Sessions/Playing", client.base_url))
            .header("Authorization", client.auth_header())
            .json(&serde_json::json!({
                "ItemId": track_id,
                "PositionTicks": 0,
                "IsPaused": false,
            }))
            .send()
            .await;
    });
}
```

Add an `api_client: ApiClient` field to `PlayerState`. The `use_player_state_provider` function reads `ApiClient` from context when initializing the state.

- [ ] **Step 2: Rewrite now_playing.rs**

Replace `packages/web/src/views/now_playing.rs`:

```rust
use dioxus::prelude::*;
use dioxus_music_ui::{api_client::ApiClient, player_state::use_player_state};

#[component]
pub fn NowPlaying() -> Element {
    let player = use_player_state();
    let client = use_context::<ApiClient>();
    let state = player.read();

    rsx! {
        div { class: "now-playing",
            if let Some(track) = &state.current_track {
                div { class: "now-playing__art",
                    if track.image_tags.as_ref().and_then(|t| t.get("Primary")).is_some() {
                        img { src: client.image_url(track.id, "Primary") }
                    }
                }
                div { class: "now-playing__info",
                    h2 { "{track.name}" }
                    if let Some(artists) = &track.artists {
                        if let Some(artist) = artists.first() {
                            p { "{artist}" }
                        }
                    }
                    if let Some(album) = &track.album {
                        p { "{album}" }
                    }
                }
            } else {
                p { "Nothing playing." }
            }
        }
    }
}
```

- [ ] **Step 3: Build**

```bash
cargo build -p dioxus_music_web --features server
```

- [ ] **Step 4: Full end-to-end test**

```bash
dx serve --package dioxus_music_web
```

Navigate to:
- `/` — album grid loads
- Click an album → track list shows
- Click a track → audio plays
- `/artists` — artist list loads
- `/playlists` → create a playlist, add tracks, view it
- `/now-playing` → shows current track with art

- [ ] **Step 5: Commit**

```bash
git add packages/ui/src/player_state.rs packages/web/src/views/now_playing.rs
git commit -m "feat(web): migrate NowPlaying view and wire PlayerState to session reporting"
```

---

## Task 12: Remove dead code and final cleanup

**Files:**
- Review all files under `packages/api/src/` and `packages/ui/src/`

- [ ] **Step 1: Delete remaining stubs**

Check for any remaining `// populated in Task N` stub comments:
```bash
rg "// populated in Task" packages/api/src/ packages/ui/src/
```
Expected: no matches.

- [ ] **Step 2: Remove old ui imports from web crate**

Check that the web crate no longer imports old types:
```bash
rg "TrackSummary\|PlaylistSummary\|PlaylistDetail\|SmartPlaylistRules" packages/web/src/
```
Expected: no matches (these old types are gone).

- [ ] **Step 3: Final workspace build**

```bash
cargo build --workspace
```
Expected: clean.

- [ ] **Step 4: Run all tests**

```bash
cargo test --workspace --features server 2>&1 | tail -20
```
Expected: all tests pass.

- [ ] **Step 5: Final commit**

```bash
git add -A
git commit -m "chore: remove dead code, final cleanup — Jellyfin migration complete"
```

---

## Plan 3 Complete — Migration Done

At this point the full Jellyfin API migration is complete:

**Backend:**
- 10-table normalized schema
- Full Jellyfin auth (users, tokens, `Authorization: MediaBrowser Token=...`)
- All music library routes: `/Items`, `/Artists`, `/Albums`, `/Genres`, `/Audio`, `/Images`, `/Search/Hints`
- Playlist CRUD + smart playlists via `/custom/` routes
- User data (favorites, ratings, play state)
- Playback session reporting

**Frontend:**
- `ApiClient` provides all server calls via HTTP
- All views migrated from server functions to `use_resource` + `ApiClient`
- `TrackList` and `PlayerState` use `BaseItemDto`
- NowPlaying reports playback start/stop to the server
