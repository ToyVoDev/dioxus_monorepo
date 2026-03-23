use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(diesel::Queryable, diesel::Selectable),
    diesel(table_name = crate::schema::tracks)
)]
pub struct TrackSummary {
    pub id: Uuid,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub duration_secs: i32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "server")]
#[derive(Debug, diesel::Insertable, diesel::AsChangeset)]
#[diesel(table_name = crate::schema::tracks)]
pub struct NewTrack {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub duration_secs: i32,
    pub file_path: String,
}

// -- Playlist models --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SmartPlaylistRules {
    pub include_genres: Vec<String>,
    pub exclude_genres: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(diesel::Queryable, diesel::Selectable),
    diesel(table_name = crate::schema::playlists)
)]
pub struct PlaylistSummary {
    pub id: Uuid,
    pub name: String,
    pub playlist_type: String,
}

/// Assembled on the server — not directly Queryable (rules parsed from JSONB).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistDetail {
    pub id: Uuid,
    pub name: String,
    pub playlist_type: String,
    pub rules: Option<SmartPlaylistRules>,
}

#[cfg(feature = "server")]
#[derive(Debug, diesel::Insertable)]
#[diesel(table_name = crate::schema::playlists)]
pub struct NewPlaylist {
    pub name: String,
    pub playlist_type: String,
    pub rules: Option<serde_json::Value>,
}

#[cfg(feature = "server")]
#[derive(Debug, diesel::Insertable)]
#[diesel(table_name = crate::schema::playlist_tracks)]
pub struct NewPlaylistTrack {
    pub playlist_id: Uuid,
    pub track_id: Uuid,
    pub position: i32,
}
