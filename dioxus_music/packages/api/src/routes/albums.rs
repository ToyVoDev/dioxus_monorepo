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

    let source: Album = albums::table
        .filter(albums::id.eq(item_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Album not found".to_string()))?;

    let genres: Vec<String> = tracks::table
        .filter(tracks::album_id.eq(item_id))
        .filter(tracks::genre.ne(""))
        .select(tracks::genre)
        .distinct()
        .load(&mut conn)
        .await?;

    // Start with other albums by the same artist.
    let mut similar: Vec<(Album, Artist)> = albums::table
        .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
        .filter(albums::id.ne(item_id))
        .filter(albums::artist_id.eq(source.artist_id))
        .select((Album::as_select(), Artist::as_select()))
        .limit(limit)
        .load(&mut conn)
        .await?;

    // Fill remainder from albums sharing the same genre.
    if (similar.len() as i64) < limit {
        let album_ids_from_genre: Vec<Uuid> = tracks::table
            .filter(tracks::genre.eq_any(&genres))
            .filter(tracks::album_id.ne(item_id).and(tracks::album_id.is_not_null()))
            .select(tracks::album_id.assume_not_null())
            .distinct()
            .limit(limit - similar.len() as i64)
            .load(&mut conn)
            .await?;

        let extra: Vec<(Album, Artist)> = albums::table
            .inner_join(artists::table.on(albums::artist_id.eq(artists::id)))
            .filter(albums::id.eq_any(&album_ids_from_genre))
            .select((Album::as_select(), Artist::as_select()))
            .load(&mut conn)
            .await?;
        similar.extend(extra);
    }

    let total = similar.len() as i64;
    let mut items = Vec::with_capacity(similar.len());
    for (album, artist) in &similar {
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
            .first(&mut conn)
            .await
            .optional()?;
        items.push(query::track_to_dto(track, artist, album.as_ref(), album.as_ref().map(|_| artist), image.as_ref(), None, state.server_id));
    }
    Ok(Json(ItemsResult { items, total_record_count: total, start_index: 0 }))
}
