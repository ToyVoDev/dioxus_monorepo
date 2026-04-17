//! Shared helpers to assemble BaseItemDto from DB rows.

use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    db::models::{Album, Artist, Image, Track, UserData},
    types::{BaseItemDto, NameGuidPair, UserItemDataDto},
};

pub fn track_to_dto(
    track: &Track,
    artist: &Artist,
    album: Option<&Album>,
    album_artist: Option<&Artist>,
    image: Option<&Image>,
    user_data: Option<&UserData>,
    server_id: Uuid,
) -> BaseItemDto {
    let image_tags = image.map(|img| {
        let mut m = HashMap::new();
        m.insert("Primary".to_string(), img.tag.clone());
        m
    });

    let album_primary_image_tag = album.and_then(|_| image.map(|img| img.tag.clone()));

    BaseItemDto {
        id: track.id,
        name: track.title.clone(),
        sort_name: Some(track.sort_title.clone()),
        item_type: "Audio".to_string(),
        server_id,
        album: album.map(|a| a.title.clone()),
        album_id: track.album_id,
        album_primary_image_tag,
        album_artist: album_artist.map(|a| a.name.clone()),
        album_artists: album_artist.map(|a| {
            vec![NameGuidPair {
                name: a.name.clone(),
                id: a.id,
            }]
        }),
        artists: Some(vec![artist.name.clone()]),
        artist_items: Some(vec![NameGuidPair {
            name: artist.name.clone(),
            id: artist.id,
        }]),
        genre_items: None,
        genres: if track.genre.is_empty() {
            None
        } else {
            Some(vec![track.genre.clone()])
        },
        run_time_ticks: Some(track.duration_ticks),
        track_number: track.track_number,
        index_number: track.track_number,
        parent_index_number: Some(track.disc_number),
        container: Some(track.container.clone()),
        media_type: Some("Audio".to_string()),
        production_year: None,
        image_tags,
        user_data: user_data.map(user_data_to_dto),
        date_created: Some(track.updated_at),
    }
}

pub fn album_to_dto(
    album: &Album,
    artist: &Artist,
    image: Option<&Image>,
    _track_count: i64,
    user_data: Option<&UserData>,
    server_id: Uuid,
) -> BaseItemDto {
    let image_tags = image.map(|img| {
        let mut m = HashMap::new();
        m.insert("Primary".to_string(), img.tag.clone());
        m
    });

    BaseItemDto {
        id: album.id,
        name: album.title.clone(),
        sort_name: Some(album.sort_title.clone()),
        item_type: "MusicAlbum".to_string(),
        server_id,
        album: None,
        album_id: None,
        album_primary_image_tag: image.map(|i| i.tag.clone()),
        album_artist: Some(artist.name.clone()),
        album_artists: Some(vec![NameGuidPair {
            name: artist.name.clone(),
            id: artist.id,
        }]),
        artists: Some(vec![artist.name.clone()]),
        artist_items: Some(vec![NameGuidPair {
            name: artist.name.clone(),
            id: artist.id,
        }]),
        genre_items: None,
        genres: None,
        run_time_ticks: None,
        track_number: None,
        index_number: None,
        parent_index_number: None,
        container: None,
        media_type: Some("Audio".to_string()),
        production_year: album.year,
        image_tags,
        user_data: user_data.map(user_data_to_dto),
        date_created: Some(album.updated_at),
    }
}

pub fn artist_to_dto(
    artist: &Artist,
    image: Option<&Image>,
    user_data: Option<&UserData>,
    server_id: Uuid,
) -> BaseItemDto {
    let image_tags = image.map(|img| {
        let mut m = HashMap::new();
        m.insert("Primary".to_string(), img.tag.clone());
        m
    });

    BaseItemDto {
        id: artist.id,
        name: artist.name.clone(),
        sort_name: Some(artist.sort_name.clone()),
        item_type: "MusicArtist".to_string(),
        server_id,
        album: None,
        album_id: None,
        album_primary_image_tag: None,
        album_artist: None,
        album_artists: None,
        artists: None,
        artist_items: None,
        genre_items: None,
        genres: None,
        run_time_ticks: None,
        track_number: None,
        index_number: None,
        parent_index_number: None,
        container: None,
        media_type: None,
        production_year: None,
        image_tags,
        user_data: user_data.map(user_data_to_dto),
        date_created: Some(artist.updated_at),
    }
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
