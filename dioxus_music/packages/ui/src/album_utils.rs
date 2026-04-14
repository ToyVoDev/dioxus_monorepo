use dioxus_music_api::types::BaseItemDto;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct AlbumSummary {
    pub name: String,
    pub artist: String,
    pub genre: String,
    pub track_count: usize,
    pub total_duration_secs: i32,
}

pub fn group_tracks_into_albums(tracks: &[BaseItemDto]) -> Vec<AlbumSummary> {
    let mut groups: BTreeMap<String, Vec<&BaseItemDto>> = BTreeMap::new();
    for track in tracks {
        let album_name = track.album.clone().unwrap_or_default();
        groups.entry(album_name).or_default().push(track);
    }

    groups
        .into_iter()
        .map(|(album_name, album_tracks)| {
            let first_artist = album_tracks[0]
                .artists
                .as_ref()
                .and_then(|a| a.first())
                .cloned()
                .unwrap_or_default();
            let artist = if album_tracks.iter().all(|t| {
                t.artists
                    .as_ref()
                    .and_then(|a| a.first())
                    .map(|s| s.as_str())
                    .unwrap_or("")
                    == first_artist
            }) {
                first_artist
            } else {
                "Various Artists".to_string()
            };
            let genre = album_tracks[0]
                .genres
                .as_ref()
                .and_then(|g| g.first())
                .cloned()
                .unwrap_or_default();
            let track_count = album_tracks.len();
            let total_duration_secs: i32 = album_tracks
                .iter()
                .map(|t| t.run_time_ticks.map(|ticks| (ticks / 10_000_000) as i32).unwrap_or(0))
                .sum();

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
