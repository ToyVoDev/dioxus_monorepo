use axum::{
    extract::{FromRequestParts, Query},
    http::{StatusCode, request::Parts},
};
use serde::Deserialize;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        models::{AccessToken, User},
        schema::{access_tokens, users},
    },
    state::AppState,
};

/// Extracted from the `Authorization: MediaBrowser Token="..."` header.
/// All protected routes receive this as a parameter.
pub struct AuthUser {
    pub user: User,
    pub token: String,
}

/// Admin-only variant — returns 403 if the user is not an admin.
pub struct AdminUser(pub AuthUser);

#[derive(Deserialize)]
struct ApiKeyQuery {
    api_key: Option<String>,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Try Authorization header first (used by API calls).
        let token_from_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|h| extract_token(h));

        // Fall back to ?api_key= query param (used by browser <img> tags).
        let token = if let Some(t) = token_from_header {
            t
        } else {
            let Query(q) = Query::<ApiKeyQuery>::from_request_parts(parts, state)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            q.api_key.ok_or(StatusCode::UNAUTHORIZED)?
        };

        let mut conn = state
            .pool
            .get()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Look up token + join user in one query.
        let result = access_tokens::table
            .inner_join(users::table)
            .filter(access_tokens::token.eq(&token))
            .select((AccessToken::as_select(), User::as_select()))
            .first::<(AccessToken, User)>(&mut conn)
            .await
            .optional()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let (_, user) = result.ok_or(StatusCode::UNAUTHORIZED)?;

        // Update last_seen_at (best-effort, don't fail the request if this errors).
        let _ = diesel::update(
            access_tokens::table.filter(access_tokens::token.eq(&token)),
        )
        .set(access_tokens::last_seen_at.eq(chrono::Utc::now()))
        .execute(&mut conn)
        .await;

        Ok(AuthUser { user, token })
    }
}

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = AuthUser::from_request_parts(parts, state).await?;
        if !auth.user.is_admin {
            return Err(StatusCode::FORBIDDEN);
        }
        Ok(AdminUser(auth))
    }
}

/// Parse `Token="<value>"` from a MediaBrowser Authorization header.
/// Header format: `MediaBrowser Client="...", Device="...", DeviceId="...", Version="...", Token="..."`
pub fn extract_token(header: &str) -> Option<String> {
    header.split(',').map(str::trim).find_map(|part| {
        let (key, val) = part.split_once('=')?;
        if key.trim().eq_ignore_ascii_case("token") {
            Some(val.trim().trim_matches('"').to_string())
        } else {
            None
        }
    })
}

/// Parse all key=value pairs from a MediaBrowser Authorization header.
pub fn parse_auth_header(header: &str) -> std::collections::HashMap<String, String> {
    let header = header
        .trim_start_matches("MediaBrowser ")
        .trim_start_matches("mediabrowser ");
    header
        .split(',')
        .filter_map(|part| {
            let (k, v) = part.trim().split_once('=')?;
            Some((
                k.trim().to_string(),
                v.trim().trim_matches('"').to_string(),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_token_from_full_header() {
        let header = r#"MediaBrowser Client="Finamp", Device="iPhone", DeviceId="abc", Version="1.0", Token="mytoken123""#;
        assert_eq!(extract_token(header), Some("mytoken123".to_string()));
    }

    #[test]
    fn returns_none_when_token_absent() {
        let header = r#"MediaBrowser Client="App", Device="PC", DeviceId="xyz", Version="1.0""#;
        assert_eq!(extract_token(header), None);
    }

    #[test]
    fn parses_all_fields() {
        let header = r#"MediaBrowser Client="MyApp", Device="Desktop", DeviceId="dev1", Version="2.0", Token="tok""#;
        let map = parse_auth_header(header);
        assert_eq!(map.get("Client").map(String::as_str), Some("MyApp"));
        assert_eq!(map.get("DeviceId").map(String::as_str), Some("dev1"));
        assert_eq!(map.get("Token").map(String::as_str), Some("tok"));
    }
}
