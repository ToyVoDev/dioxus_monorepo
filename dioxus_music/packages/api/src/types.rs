//! Shared response DTOs — serialized by the server, deserialized by the client.
//! No feature gate: compiled for both server and WASM.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The universal Jellyfin item object. `item_type` discriminates between
/// Audio, MusicAlbum, and MusicArtist.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemDto {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: String,
    pub server_id: Uuid,

    // Track-specific
    pub album: Option<String>,
    pub album_id: Option<Uuid>,
    pub album_primary_image_tag: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Option<Vec<NameGuidPair>>,
    pub artists: Option<Vec<String>>,
    pub artist_items: Option<Vec<NameGuidPair>>,
    pub genre_items: Option<Vec<NameGuidPair>>,
    pub genres: Option<Vec<String>>,
    pub run_time_ticks: Option<i64>,
    pub track_number: Option<i32>,
    pub index_number: Option<i32>,      // same as track_number, Jellyfin alias
    pub parent_index_number: Option<i32>, // disc number
    pub container: Option<String>,
    pub media_type: Option<String>,

    // Album-specific
    pub production_year: Option<i32>,

    // Shared
    pub image_tags: Option<std::collections::HashMap<String, String>>,
    pub user_data: Option<UserItemDataDto>,
    pub date_created: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameGuidPair {
    pub name: String,
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserItemDataDto {
    pub is_favorite: bool,
    pub likes: Option<bool>,
    pub play_count: i32,
    pub last_played_date: Option<DateTime<Utc>>,
    pub played: bool,
    pub playback_position_ticks: i64,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemsResult {
    pub items: Vec<BaseItemDto>,
    pub total_record_count: i64,
    pub start_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserDto {
    pub id: Uuid,
    pub name: String,
    pub server_id: Uuid,
    pub has_password: bool,
    pub has_configured_password: bool,
    pub enable_auto_login: bool,
    pub last_login_date: Option<DateTime<Utc>>,
    pub last_activity_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResult {
    pub user: UserDto,
    pub access_token: String,
    pub server_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSourceInfo {
    pub id: String,
    pub path: Option<String>,
    pub protocol: String,           // "File"
    pub media_type: Option<String>, // "Audio"
    pub container: Option<String>,
    pub size: Option<i64>,
    pub bit_rate: Option<i32>,
    pub default_audio_stream_index: Option<i32>,
    pub supports_direct_play: bool,
    pub supports_direct_stream: bool,
    pub supports_transcoding: bool,
    pub is_remote: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaybackInfoResponse {
    pub media_sources: Vec<MediaSourceInfo>,
    pub play_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHint {
    pub item_id: Uuid,
    pub name: String,
    #[serde(rename = "Type")]
    pub item_type: String,
    pub album: Option<String>,
    pub album_id: Option<Uuid>,
    pub album_artist: Option<String>,
    pub primary_image_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchHintsResult {
    pub search_hints: Vec<SearchHint>,
    pub total_record_count: i64,
}
