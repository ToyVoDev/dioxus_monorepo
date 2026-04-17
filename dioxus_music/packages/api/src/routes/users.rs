use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::{
        middleware::{AdminUser, AuthUser, parse_auth_header},
        password, token,
    },
    db::{
        models::{NewAccessToken, NewUser, User},
        schema::{access_tokens, users},
    },
    error::ApiError,
    state::AppState,
    types::{AuthenticationResult, UserDto},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Users/AuthenticateByName", post(authenticate_by_name))
        .route("/Users", get(list_users))
        .route("/Users/{user_id}", get(get_user))
        .route("/Users/{user_id}/Password", post(change_password))
        .route("/Users/{user_id}", delete(delete_user))
        .route("/Sessions/Logout", delete(logout))
        .route("/Auth/Keys", get(list_api_keys))
        .route("/Auth/Keys", post(create_api_key))
        .route("/Auth/Keys/{key}", delete(delete_api_key))
}

// ── DTOs ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateByNameRequest {
    pub username: String,
    pub pw: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangePasswordRequest {
    pub current_pw: String,
    pub new_pw: String,
}

fn user_to_dto(user: &User, server_id: Uuid) -> UserDto {
    UserDto {
        id: user.id,
        name: user.name.clone(),
        server_id,
        has_password: true,
        has_configured_password: true,
        enable_auto_login: false,
        last_login_date: None,
        last_activity_date: None,
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────

/// POST /Users/AuthenticateByName
async fn authenticate_by_name(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<AuthenticateByNameRequest>,
) -> Result<Json<AuthenticationResult>, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let user: Option<User> = users::table
        .filter(users::name.eq(&body.username))
        .first::<User>(&mut conn)
        .await
        .optional()?;

    let user = user.ok_or(ApiError::Unauthorized)?;

    if !password::verify_password(&body.pw, &user.password_hash) {
        return Err(ApiError::Unauthorized);
    }

    // Parse client/device info from Authorization header (present without Token on pre-auth).
    let auth_info = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(parse_auth_header)
        .unwrap_or_default();

    let new_token = NewAccessToken {
        token: token::generate(),
        user_id: user.id,
        device_id: auth_info.get("DeviceId").cloned().unwrap_or_default(),
        device_name: auth_info.get("Device").cloned().unwrap_or_default(),
        client_name: auth_info.get("Client").cloned().unwrap_or_default(),
    };

    let inserted_token: crate::db::models::AccessToken = diesel::insert_into(access_tokens::table)
        .values(&new_token)
        .get_result(&mut conn)
        .await?;

    Ok(Json(AuthenticationResult {
        user: user_to_dto(&user, state.server_id),
        access_token: inserted_token.token,
        server_id: state.server_id,
    }))
}

/// GET /Users — admin only
async fn list_users(
    AdminUser(_): AdminUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserDto>>, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let all: Vec<User> = users::table.load(&mut conn).await?;
    let dtos: Vec<UserDto> = all
        .iter()
        .map(|u| user_to_dto(u, state.server_id))
        .collect();
    Ok(Json(dtos))
}

/// GET /Users/{userId}
async fn get_user(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserDto>, ApiError> {
    // Non-admin users can only read their own profile.
    if auth.user.id != user_id && !auth.user.is_admin {
        return Err(ApiError::Forbidden);
    }
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let user: User = users::table
        .filter(users::id.eq(user_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;
    Ok(Json(user_to_dto(&user, state.server_id)))
}

/// POST /Users/{userId}/Password
async fn change_password(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    if auth.user.id != user_id && !auth.user.is_admin {
        return Err(ApiError::Forbidden);
    }
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let user: User = users::table
        .filter(users::id.eq(user_id))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    if !password::verify_password(&body.current_pw, &user.password_hash) {
        return Err(ApiError::Unauthorized);
    }

    let new_hash =
        password::hash_password(&body.new_pw).map_err(|e| ApiError::Internal(e.to_string()))?;

    diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(users::password_hash.eq(new_hash))
        .execute(&mut conn)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /Users/{userId} — admin only
async fn delete_user(
    AdminUser(_): AdminUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(users::table.filter(users::id.eq(user_id)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /Sessions/Logout
async fn logout(auth: AuthUser, State(state): State<AppState>) -> Result<StatusCode, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(access_tokens::table.filter(access_tokens::token.eq(&auth.token)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /Auth/Keys — admin only (stub: tokens serve as API keys for now)
async fn list_api_keys(
    AdminUser(auth): AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let tokens: Vec<crate::db::models::AccessToken> = access_tokens::table
        .filter(access_tokens::user_id.eq(auth.user.id))
        .load(&mut conn)
        .await?;
    let items: Vec<serde_json::Value> = tokens
        .iter()
        .map(|t| {
            serde_json::json!({
                "AccessToken": t.token,
                "DeviceId": t.device_id,
                "AppName": t.client_name,
                "DateCreated": t.created_at,
            })
        })
        .collect();
    let count = items.len();
    Ok(Json(
        serde_json::json!({ "Items": items, "TotalRecordCount": count }),
    ))
}

/// POST /Auth/Keys — admin only (creates a new static token for the admin user)
async fn create_api_key(
    AdminUser(auth): AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let new_token = NewAccessToken {
        token: token::generate(),
        user_id: auth.user.id,
        device_id: "api-key".to_string(),
        device_name: "API Key".to_string(),
        client_name: "Static".to_string(),
    };
    let inserted: crate::db::models::AccessToken = diesel::insert_into(access_tokens::table)
        .values(&new_token)
        .get_result(&mut conn)
        .await?;
    Ok(Json(serde_json::json!({ "AccessToken": inserted.token })))
}

/// DELETE /Auth/Keys/{key} — admin only
async fn delete_api_key(
    AdminUser(_): AdminUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<StatusCode, ApiError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    diesel::delete(access_tokens::table.filter(access_tokens::token.eq(key)))
        .execute(&mut conn)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
