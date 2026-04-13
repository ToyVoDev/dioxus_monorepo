use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

// ── Users ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub is_admin: bool,
}

// ── Access Tokens ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::access_tokens)]
#[diesel(primary_key(token))]
pub struct AccessToken {
    pub token: String,
    pub user_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub client_name: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::access_tokens)]
pub struct NewAccessToken {
    pub token: String,
    pub user_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub client_name: String,
}

// ── Artists ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::artists)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::artists)]
pub struct NewArtist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
}

// ── Albums ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::albums)]
pub struct Album {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub year: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::albums)]
pub struct NewAlbum {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub year: Option<i32>,
}

// ── Tracks ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::tracks)]
pub struct Track {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub genre: String,
    pub duration_ticks: i64,
    pub track_number: Option<i32>,
    pub disc_number: i32,
    pub file_path: String,
    pub container: String,
    pub bit_rate: Option<i32>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::tracks)]
pub struct NewTrack {
    pub id: Uuid,
    pub title: String,
    pub sort_title: String,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub genre: String,
    pub duration_ticks: i64,
    pub track_number: Option<i32>,
    pub disc_number: i32,
    pub file_path: String,
    pub container: String,
    pub bit_rate: Option<i32>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
}

// ── Genres ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::genres)]
pub struct Genre {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::genres)]
pub struct NewGenre {
    pub id: Uuid,
    pub name: String,
}

// ── Images ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::images)]
pub struct Image {
    pub item_id: Uuid,
    pub image_type: String,
    pub file_path: String,
    pub tag: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::images)]
pub struct NewImage {
    pub item_id: Uuid,
    pub image_type: String,
    pub file_path: String,
    pub tag: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

// ── Playlists ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::playlists)]
pub struct Playlist {
    pub id: Uuid,
    pub name: String,
    pub overview: Option<String>,
    pub is_smart: bool,
    pub user_id: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::playlists)]
pub struct NewPlaylist {
    pub id: Uuid,
    pub name: String,
    pub overview: Option<String>,
    pub is_smart: bool,
    pub user_id: Option<Uuid>,
}

// ── Playlist Items ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::playlist_items)]
pub struct PlaylistItem {
    pub id: Uuid,
    pub playlist_id: Uuid,
    pub item_id: Uuid,
    pub position: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::playlist_items)]
pub struct NewPlaylistItem {
    pub id: Uuid,
    pub playlist_id: Uuid,
    pub item_id: Uuid,
    pub position: i32,
}

// ── User Data ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::user_data)]
pub struct UserData {
    pub user_id: Uuid,
    pub item_id: Uuid,
    pub item_type: String,
    pub is_favorite: bool,
    pub likes: Option<bool>,
    pub play_count: i32,
    pub last_played_date: Option<DateTime<Utc>>,
    pub played: bool,
    pub playback_position_ticks: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::user_data)]
pub struct NewUserData {
    pub user_id: Uuid,
    pub item_id: Uuid,
    pub item_type: String,
}
