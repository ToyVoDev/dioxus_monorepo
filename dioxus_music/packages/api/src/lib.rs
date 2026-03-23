pub mod models;

#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod scanner;
#[cfg(feature = "server")]
pub mod schema;
#[cfg(feature = "server")]
pub mod streaming;

use dioxus::prelude::*;
use models::{PlaylistDetail, PlaylistSummary, SmartPlaylistRules, TrackSummary};
use uuid::Uuid;

/// Log an error on the server and return it as a ServerFnError.
#[cfg(feature = "server")]
fn server_err(msg: String) -> ServerFnError {
    tracing::error!("{msg}");
    ServerFnError::new(msg)
}

#[get("/api/library")]
pub async fn get_library() -> Result<Vec<TrackSummary>, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let tracks = schema::tracks::table
        .select(TrackSummary::as_select())
        .order(schema::tracks::title.asc())
        .load(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    Ok(tracks)
}

#[post("/api/rescan")]
pub async fn rescan_library() -> Result<(), ServerFnError> {
    use axum::Extension;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;

    scanner::full_scan(pool).await;

    Ok(())
}

// -- Playlist endpoints --

#[get("/api/playlists")]
pub async fn get_playlists() -> Result<Vec<PlaylistSummary>, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let playlists = schema::playlists::table
        .select(PlaylistSummary::as_select())
        .order(schema::playlists::name.asc())
        .load(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    Ok(playlists)
}

#[post("/api/playlists/detail")]
pub async fn get_playlist(id: Uuid) -> Result<PlaylistDetail, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let row: (Uuid, String, String, Option<serde_json::Value>) = schema::playlists::table
        .filter(schema::playlists::id.eq(id))
        .select((
            schema::playlists::id,
            schema::playlists::name,
            schema::playlists::playlist_type,
            schema::playlists::rules,
        ))
        .first(&mut conn)
        .await
        .map_err(|e| server_err(format!("Playlist not found: {e}")))?;

    let rules = row
        .3
        .map(|v| serde_json::from_value::<SmartPlaylistRules>(v).unwrap_or_default());

    Ok(PlaylistDetail {
        id: row.0,
        name: row.1,
        playlist_type: row.2,
        rules,
    })
}

#[post("/api/playlists/manual")]
pub async fn create_manual_playlist(name: String) -> Result<PlaylistSummary, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use models::NewPlaylist;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let new = NewPlaylist {
        name,
        playlist_type: "manual".to_string(),
        rules: None,
    };

    let created = diesel::insert_into(schema::playlists::table)
        .values(&new)
        .returning(PlaylistSummary::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| server_err(format!("Insert error: {e}")))?;

    Ok(created)
}

#[post("/api/playlists/smart")]
pub async fn create_smart_playlist(
    name: String,
    rules: SmartPlaylistRules,
) -> Result<PlaylistSummary, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use models::NewPlaylist;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let rules_json =
        serde_json::to_value(&rules).map_err(|e| server_err(format!("JSON error: {e}")))?;

    let new = NewPlaylist {
        name,
        playlist_type: "smart".to_string(),
        rules: Some(rules_json),
    };

    let created = diesel::insert_into(schema::playlists::table)
        .values(&new)
        .returning(PlaylistSummary::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| server_err(format!("Insert error: {e}")))?;

    Ok(created)
}

#[post("/api/playlists/rename")]
pub async fn rename_playlist(id: Uuid, name: String) -> Result<(), ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    diesel::update(schema::playlists::table.filter(schema::playlists::id.eq(id)))
        .set(schema::playlists::name.eq(name))
        .execute(&mut conn)
        .await
        .map_err(|e| server_err(format!("Update error: {e}")))?;

    Ok(())
}

#[post("/api/playlists/update-rules")]
pub async fn update_smart_playlist_rules(
    id: Uuid,
    rules: SmartPlaylistRules,
) -> Result<(), ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let rules_json =
        serde_json::to_value(&rules).map_err(|e| server_err(format!("JSON error: {e}")))?;

    diesel::update(schema::playlists::table.filter(schema::playlists::id.eq(id)))
        .set(schema::playlists::rules.eq(Some(rules_json)))
        .execute(&mut conn)
        .await
        .map_err(|e| server_err(format!("Update error: {e}")))?;

    Ok(())
}

#[post("/api/playlists/delete")]
pub async fn delete_playlist(id: Uuid) -> Result<(), ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    diesel::delete(schema::playlists::table.filter(schema::playlists::id.eq(id)))
        .execute(&mut conn)
        .await
        .map_err(|e| server_err(format!("Delete error: {e}")))?;

    Ok(())
}

#[post("/api/playlists/tracks")]
pub async fn get_playlist_tracks(id: Uuid) -> Result<Vec<TrackSummary>, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    // First get the playlist type and rules
    let (playlist_type, rules_json): (String, Option<serde_json::Value>) = schema::playlists::table
        .filter(schema::playlists::id.eq(id))
        .select((schema::playlists::playlist_type, schema::playlists::rules))
        .first(&mut conn)
        .await
        .map_err(|e| server_err(format!("Playlist not found: {e}")))?;

    match playlist_type.as_str() {
        "manual" => {
            let tracks = schema::tracks::table
                .inner_join(
                    schema::playlist_tracks::table
                        .on(schema::playlist_tracks::track_id.eq(schema::tracks::id)),
                )
                .filter(schema::playlist_tracks::playlist_id.eq(id))
                .order(schema::playlist_tracks::position.asc())
                .select(TrackSummary::as_select())
                .load(&mut conn)
                .await
                .map_err(|e| server_err(format!("DB query error: {e}")))?;
            Ok(tracks)
        }
        "smart" => {
            let rules = rules_json
                .map(|v| serde_json::from_value::<SmartPlaylistRules>(v).unwrap_or_default())
                .unwrap_or_default();

            let mut query = schema::tracks::table
                .select(TrackSummary::as_select())
                .into_boxed();

            if !rules.include_genres.is_empty() {
                query = query.filter(schema::tracks::genre.eq_any(&rules.include_genres));
            }
            if !rules.exclude_genres.is_empty() {
                query = query.filter(schema::tracks::genre.ne_all(&rules.exclude_genres));
            }

            let tracks = query
                .order(schema::tracks::title.asc())
                .load(&mut conn)
                .await
                .map_err(|e| server_err(format!("DB query error: {e}")))?;
            Ok(tracks)
        }
        _ => Err(server_err(format!(
            "Unknown playlist type: {playlist_type}"
        ))),
    }
}

#[post("/api/playlists/add-track")]
pub async fn add_track_to_playlist(playlist_id: Uuid, track_id: Uuid) -> Result<(), ServerFnError> {
    use axum::Extension;
    use diesel::dsl::max;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use models::NewPlaylistTrack;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let max_pos: Option<i32> = schema::playlist_tracks::table
        .filter(schema::playlist_tracks::playlist_id.eq(playlist_id))
        .select(max(schema::playlist_tracks::position))
        .first(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    let new = NewPlaylistTrack {
        playlist_id,
        track_id,
        position: max_pos.unwrap_or(-1) + 1,
    };

    diesel::insert_into(schema::playlist_tracks::table)
        .values(&new)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await
        .map_err(|e| server_err(format!("Insert error: {e}")))?;

    Ok(())
}

#[post("/api/playlists/remove-track")]
pub async fn remove_track_from_playlist(
    playlist_id: Uuid,
    track_id: Uuid,
) -> Result<(), ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    // Delete the track entry
    diesel::delete(
        schema::playlist_tracks::table
            .filter(schema::playlist_tracks::playlist_id.eq(playlist_id))
            .filter(schema::playlist_tracks::track_id.eq(track_id)),
    )
    .execute(&mut conn)
    .await
    .map_err(|e| server_err(format!("Delete error: {e}")))?;

    // Recompact positions: load remaining rows ordered by position, then update each
    let remaining: Vec<(Uuid, i32)> = schema::playlist_tracks::table
        .filter(schema::playlist_tracks::playlist_id.eq(playlist_id))
        .order(schema::playlist_tracks::position.asc())
        .select((
            schema::playlist_tracks::id,
            schema::playlist_tracks::position,
        ))
        .load(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    for (i, (row_id, _)) in remaining.iter().enumerate() {
        diesel::update(
            schema::playlist_tracks::table.filter(schema::playlist_tracks::id.eq(row_id)),
        )
        .set(schema::playlist_tracks::position.eq(i as i32))
        .execute(&mut conn)
        .await
        .map_err(|e| server_err(format!("Recompact error: {e}")))?;
    }

    Ok(())
}

#[get("/api/genres")]
pub async fn get_genres() -> Result<Vec<String>, ServerFnError> {
    use axum::Extension;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let genres: Vec<String> = schema::tracks::table
        .select(schema::tracks::genre)
        .distinct()
        .order(schema::tracks::genre.asc())
        .load(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    Ok(genres)
}

#[server]
pub async fn get_library_version() -> Result<Option<chrono::DateTime<chrono::Utc>>, ServerFnError> {
    use diesel::dsl::max;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use axum::Extension;

    let Extension(pool): Extension<db::DbPool> =
        dioxus::fullstack::FullstackContext::extract().await?;
    let mut conn = pool
        .get()
        .await
        .map_err(|e| server_err(format!("DB pool error: {e}")))?;

    let result: Option<chrono::DateTime<chrono::Utc>> = schema::tracks::table
        .select(max(schema::tracks::updated_at))
        .first(&mut conn)
        .await
        .map_err(|e| server_err(format!("DB query error: {e}")))?;

    Ok(result)
}

#[server]
pub async fn get_health() -> Result<String, ServerFnError> {
    Ok("ok".to_string())
}
