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
        .route("/Items/Filters", get(get_filters))
        .route("/Items/{item_id}", get(get_item))
        .route("/Items", get(list_items))
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
    _auth: AuthUser,
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
            let pattern = format!("%{}%", term);
            q = q.filter(
                tracks::title.ilike(pattern.clone())
                    .or(artists::name.ilike(pattern)),
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
                artist,
                album.as_ref(),
                album.as_ref().map(|_| artist),
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
            let pattern = format!("%{}%", term);
            q = q.filter(albums::title.ilike(pattern));
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
            let pattern = format!("%{}%", term);
            q = q.filter(artists::name.ilike(pattern));
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

/// GET /Items/{item_id}
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
        return Ok(Json(query::track_to_dto(
            &track,
            &artist,
            album.as_ref(),
            album.as_ref().map(|_| &artist),
            image.as_ref(),
            None,
            state.server_id,
        )));
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
        let count = tracks::table
            .filter(tracks::album_id.eq(album.id))
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
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

/// GET /Items/Filters — returns available genres for filter UI
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
