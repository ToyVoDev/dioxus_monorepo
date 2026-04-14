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
    types::{BaseItemDto, ItemsResult},
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
        let pattern = format!("%{}%", term);
        q = q.filter(genres::name.ilike(pattern));
    }

    let all: Vec<Genre> = q
        .order(genres::name.asc())
        .offset(start)
        .limit(limit)
        .load(&mut conn)
        .await?;
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
