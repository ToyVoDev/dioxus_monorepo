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
    db::{models::UserData, schema::user_data},
    error::ApiError,
    state::AppState,
    types::UserItemDataDto,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/UserFavoriteItems/{item_id}",
            post(mark_favorite).delete(unmark_favorite),
        )
        .route("/UserItems/{item_id}/Rating", post(rate_item))
        .route(
            "/UserPlayedItems/{item_id}",
            post(mark_played).delete(mark_unplayed),
        )
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.is_favorite = true;
    })
    .await?;
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.is_favorite = false;
    })
    .await?;
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.likes = params.likes; // None = clear rating
    })
    .await?;
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.played = true;
        d.play_count += 1;
        d.last_played_date = Some(Utc::now());
    })
    .await?;
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
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let ud = upsert_user_data(&mut conn, user_id, item_id, "Audio", |d| {
        d.played = false;
    })
    .await?;
    Ok(Json(user_data_to_dto(&ud)))
}
