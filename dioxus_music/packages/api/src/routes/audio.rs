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
        .route(
            "/Audio/{item_id}/stream.{container}",
            get(stream_audio_with_container),
        )
        .route("/Audio/{item_id}/universal", get(stream_audio))
        .route("/Audio/{item_id}/Lyrics", get(get_lyrics))
        .route(
            "/Items/{item_id}/PlaybackInfo",
            get(get_playback_info).post(get_playback_info),
        )
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

async fn stream_track_file(state: &AppState, item_id: Uuid) -> Result<Response, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let track: Track = tracks::table
        .filter(tracks::id.eq(item_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Track not found".to_string()))?;

    let file = File::open(&track.file_path)
        .await
        .map_err(|e| ApiError::Internal(format!("File open error: {e}")))?;

    let metadata = file
        .metadata()
        .await
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
    // Always stream the original file regardless of requested container.
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let track: Track = tracks::table
        .filter(tracks::id.eq(item_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Track not found".to_string()))?;

    let file_size = std::fs::metadata(&track.file_path)
        .ok()
        .map(|m| m.len() as i64);

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
