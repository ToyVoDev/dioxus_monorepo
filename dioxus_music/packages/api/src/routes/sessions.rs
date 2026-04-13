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
