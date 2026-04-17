use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{auth::middleware::AuthUser, db::schema::tracks, error::ApiError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/custom/library/rescan", post(rescan_library))
        .route("/custom/library/version", get(library_version))
        .route("/custom/health", get(health))
}

/// POST /custom/library/rescan — trigger full rescan in background
async fn rescan_library(_auth: AuthUser, State(state): State<AppState>) -> StatusCode {
    tokio::spawn(crate::scanner::full_scan(state));
    StatusCode::ACCEPTED
}

/// GET /custom/library/version — returns max updated_at for client cache invalidation
async fn library_version(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let version: Option<DateTime<Utc>> = tracks::table
        .select(diesel::dsl::max(tracks::updated_at))
        .first(&mut conn)
        .await?;
    Ok(Json(serde_json::json!({ "Version": version })))
}

/// GET /custom/health
async fn health() -> StatusCode {
    StatusCode::OK
}
