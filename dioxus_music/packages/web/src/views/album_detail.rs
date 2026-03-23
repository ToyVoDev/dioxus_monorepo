use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::TrackList;
use kinetic_ui::{IconButton, KBadge, KBadgeVariant, KButton, KButtonVariant};

const ALBUM_DETAIL_CSS: Asset = asset!("/assets/album_detail.css");

#[component]
pub fn AlbumDetail(name: String) -> Element {
    let tracks = use_server_future(dioxus_music_api::get_library)?;
    let result = tracks.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: ALBUM_DETAIL_CSS }

        div { class: "album-detail",
            Link { class: "album-detail__back", to: Route::Library {},
                "\u{2190} Back to Library"
            }

            match result {
                Some(Ok(all_tracks)) => {
                    let album_tracks: Vec<_> = all_tracks
                        .iter()
                        .filter(|t| t.album == name)
                        .cloned()
                        .collect();

                    let artist = {
                        let artists: std::collections::HashSet<&str> = album_tracks
                            .iter()
                            .map(|t| t.artist.as_str())
                            .collect();
                        if artists.len() == 1 {
                            artists.into_iter().next().unwrap_or("Unknown").to_string()
                        } else {
                            "Various Artists".to_string()
                        }
                    };

                    let genre = album_tracks
                        .first()
                        .map(|t| t.genre.clone())
                        .unwrap_or_default();

                    let track_count = album_tracks.len();

                    let total_duration_secs: i32 = album_tracks
                        .iter()
                        .map(|t| t.duration_secs)
                        .sum();

                    let minutes = total_duration_secs / 60;
                    let seconds = total_duration_secs % 60;

                    let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();

                    rsx! {
                        div { class: "album-detail__header",
                            div { class: "album-detail__art",
                                "{initial}"
                            }
                            div { class: "album-detail__meta",
                                span { class: "album-detail__label", "NOW VIEWING" }
                                h1 { class: "album-detail__title", "{name}" }
                                div { class: "album-detail__artist-row",
                                    span { "{artist}" }
                                    if !genre.is_empty() {
                                        KBadge { variant: KBadgeVariant::Secondary, "{genre}" }
                                    }
                                }
                                div { class: "album-detail__actions",
                                    KButton { variant: KButtonVariant::Primary, "Download Album" }
                                    IconButton { "\u{2661}" }
                                }
                            }
                        }

                        div { class: "album-detail__tracklist",
                            TrackList { tracks: album_tracks, show_download_status: true }
                        }

                        div { class: "album-detail__footer",
                            "{track_count} tracks \u{2022} {minutes}:{seconds:02}"
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    p { "Error loading library: {e}" }
                },
                None => rsx! {
                    p { "Loading..." }
                },
            }
        }
    }
}
