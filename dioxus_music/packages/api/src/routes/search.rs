use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{
        models::{Album, Artist, Track},
        schema::{albums, artists, tracks},
    },
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let limit = params.limit.unwrap_or(20).min(100);
    let pattern = format!("%{}%", params.search_term.trim());

    let types: Vec<&str> = params
        .include_item_types
        .as_deref()
        .map(|s| s.split(',').map(str::trim).collect())
        .unwrap_or_else(|| vec!["Audio", "MusicAlbum", "MusicArtist"]);

    let mut hints: Vec<SearchHint> = Vec::new();

    if types.contains(&"Audio") {
        let track_rows: Vec<(Track, Artist, Option<Album>)> = tracks::table
            .inner_join(artists::table.on(tracks::artist_id.eq(artists::id)))
            .left_join(albums::table.on(tracks::album_id.eq(albums::id.nullable())))
            .filter(
                tracks::title
                    .ilike(&pattern)
                    .or(artists::name.ilike(&pattern)),
            )
            .select((
                Track::as_select(),
                Artist::as_select(),
                Option::<Album>::as_select(),
            ))
            .limit(limit)
            .load(&mut conn)
            .await?;

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
            .filter(albums::title.ilike(&pattern))
            .select((Album::as_select(), Artist::as_select()))
            .limit(limit)
            .load(&mut conn)
            .await?;

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
            .filter(artists::name.ilike(&pattern))
            .limit(limit)
            .load(&mut conn)
            .await?;

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
    Ok(Json(SearchHintsResult {
        search_hints: hints,
        total_record_count: total,
    }))
}
