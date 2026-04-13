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
        .route(
            "/Playlists/{playlist_id}",
            get(get_playlist)
                .post(update_playlist)
                .delete(delete_playlist),
        )
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
        .route(
            "/Playlists/{item_id}/InstantMix",
            get(instant_mix_from_playlist),
        )
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    if let Some(name) = body.name {
        diesel::update(playlists::table.filter(playlists::id.eq(playlist_id)))
            .set((
                playlists::name.eq(name),
                playlists::updated_at.eq(Utc::now()),
            ))
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
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

        q.select((
            Track::as_select(),
            Artist::as_select(),
            Option::<Album>::as_select(),
        ))
        .order((
            artists::sort_name.asc(),
            albums::sort_title.asc().nulls_last(),
            tracks::disc_number.asc(),
            tracks::track_number.asc().nulls_last(),
        ))
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
            .select((
                Track::as_select(),
                Artist::as_select(),
                Option::<Album>::as_select(),
            ))
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
            track,
            &artist,
            album.as_ref(),
            album.as_ref().map(|_| &artist),
            image.as_ref(),
            None,
            state.server_id,
        ));
    }
    Ok(Json(ItemsResult {
        items,
        total_record_count: total,
        start_index: start as i32,
    }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddItemsQuery {
    pub ids: String, // comma-delimited UUIDs
    pub user_id: Option<Uuid>,
}

/// POST /Playlists/{playlistId}/Items
async fn add_playlist_items(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Query(params): Query<AddItemsQuery>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let playlist: Playlist = playlists::table
        .filter(playlists::id.eq(playlist_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Playlist not found".to_string()))?;

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
    pub entry_ids: String, // comma-delimited entry UUIDs (playlist_items.id)
}

/// DELETE /Playlists/{playlistId}/Items
async fn remove_playlist_items(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(playlist_id): Path<Uuid>,
    Query(params): Query<RemoveItemsQuery>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let playlist: Playlist = playlists::table
        .filter(playlists::id.eq(playlist_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Playlist not found".to_string()))?;

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
            playlist_items::playlist_id
                .eq(playlist_id)
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Load all items in order.
    let mut entries: Vec<(Uuid, Uuid, i32)> = playlist_items::table
        .filter(playlist_items::playlist_id.eq(playlist_id))
        .order(playlist_items::position.asc())
        .select((
            playlist_items::id,
            playlist_items::item_id,
            playlist_items::position,
        ))
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
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
        .select((
            Track::as_select(),
            Artist::as_select(),
            Option::<Album>::as_select(),
        ))
        .limit(limit)
        .load(&mut conn)
        .await?;

    let total = rows.len() as i64;
    let mut items = Vec::with_capacity(rows.len());
    for (track, artist, album) in &rows {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
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
    Ok(Json(ItemsResult {
        items,
        total_record_count: total,
        start_index: 0,
    }))
}

// ── Custom smart playlist routes ──────────────────────────────────────────

/// POST /custom/playlists/smart
async fn create_smart_playlist(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateSmartPlaylistRequest>,
) -> Result<Json<BaseItemDto>, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let rules_json =
        serde_json::to_string(&body.rules).map_err(|e| ApiError::Internal(e.to_string()))?;

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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let rules_json =
        serde_json::to_string(&rules).map_err(|e| ApiError::Internal(e.to_string()))?;

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
