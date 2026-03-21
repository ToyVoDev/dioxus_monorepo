use dioxus_music_api::models::TrackSummary;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct AlbumSummary {
    pub name: String,
    pub artist: String,
    pub genre: String,
    pub track_count: usize,
    pub total_duration_secs: i32,
}

pub fn group_tracks_into_albums(tracks: &[TrackSummary]) -> Vec<AlbumSummary> {
    let mut groups: BTreeMap<String, Vec<&TrackSummary>> = BTreeMap::new();
    for track in tracks {
        groups.entry(track.album.clone()).or_default().push(track);
    }

    groups
        .into_iter()
        .map(|(album_name, album_tracks)| {
            let first_artist = &album_tracks[0].artist;
            let artist = if album_tracks.iter().all(|t| t.artist == *first_artist) {
                first_artist.clone()
            } else {
                "Various Artists".to_string()
            };
            let genre = album_tracks[0].genre.clone();
            let track_count = album_tracks.len();
            let total_duration_secs: i32 = album_tracks.iter().map(|t| t.duration_secs).sum();

            AlbumSummary {
                name: album_name,
                artist,
                genre,
                track_count,
                total_duration_secs,
            }
        })
        .collect()
}
