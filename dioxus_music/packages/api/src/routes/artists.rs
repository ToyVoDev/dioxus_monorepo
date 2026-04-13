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
        let pattern = format!("%{}%", term);
        q = q.filter(artists::name.ilike(&pattern));
    }

    let all: Vec<Artist> = q
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
            .first(&mut conn)
            .await
            .optional()?;
        items.push(query::artist_to_dto(artist, image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: start as i32 }))
}

async fn list_album_artists(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<ArtistQuery>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(50).min(500);
    let start = params.start_index.unwrap_or(0);

    // Album artists are artists who own at least one album.
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
            .first(&mut conn)
            .await
            .optional()?;
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
        .first(&mut conn)
        .await
        .optional()?;
    Ok(Json(query::artist_to_dto(&artist, image.as_ref(), None, state.server_id)))
}

async fn similar_artists(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<ItemsResult>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Find genres of this artist's tracks.
    let genres: Vec<String> = tracks::table
        .filter(tracks::artist_id.eq(item_id))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre)
        .distinct()
        .load(&mut conn)
        .await?;

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
        .load(&mut conn)
        .await?;

    let similar: Vec<Artist> = artists::table
        .filter(artists::id.eq_any(&similar_artist_ids))
        .load(&mut conn)
        .await?;

    let total = similar.len() as i64;
    let mut items = Vec::with_capacity(similar.len());
    for artist in &similar {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(artist.id))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()?;
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
        .load(&mut conn)
        .await?;

    let rows: Vec<(crate::db::models::Track, Artist, Option<crate::db::models::Album>)> =
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

    let total = rows.len() as i64;
    let mut items = Vec::with_capacity(rows.len());
    for (track, artist, album) in &rows {
        let image: Option<Image> = images::table
            .filter(images::item_id.eq(album.as_ref().map(|a| a.id).unwrap_or(track.id)))
            .filter(images::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()?;
        items.push(query::track_to_dto(track, artist, album.as_ref(), album.as_ref().map(|_| artist), image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}
