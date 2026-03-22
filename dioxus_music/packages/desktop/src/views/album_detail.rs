use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::TrackList;
use kinetic_ui::{Badge, BadgeVariant, Button, ButtonVariant, IconButton};

fn format_duration_minutes(total_secs: i32) -> String {
    let minutes = total_secs / 60;
    if minutes < 60 {
        format!("{minutes} minutes")
    } else {
        let hours = minutes / 60;
        let remaining_mins = minutes % 60;
        format!("{hours}h {remaining_mins}m")
    }
}

#[component]
pub fn AlbumDetail(name: String) -> Element {
    let tracks = use_server_future(dioxus_music_api::get_library)?;
    let result = tracks.read().clone();

    rsx! {
        div {
            style: "padding: var(--k-space-6);",
            Link {
                style: "color: var(--k-on-surface-variant); text-decoration: none; font-size: 0.875rem; display: inline-flex; align-items: center; gap: var(--k-space-1); margin-bottom: var(--k-space-4);",
                to: Route::Library {},
                "← Back"
            }

            match result {
                Some(Ok(all_tracks)) => {
                    let album_tracks: Vec<_> = all_tracks.into_iter()
                        .filter(|t| t.album == name)
                        .collect();

                    if album_tracks.is_empty() {
                        rsx! { p { style: "color: var(--k-on-surface-variant);", "No tracks found for this album." } }
                    } else {
                        let artist = {
                            let first = &album_tracks[0].artist;
                            if album_tracks.iter().all(|t| t.artist == *first) {
                                first.clone()
                            } else {
                                "Various Artists".to_string()
                            }
                        };
                        let genre = album_tracks[0].genre.clone();
                        let track_count = album_tracks.len();
                        let total_secs: i32 = album_tracks.iter().map(|t| t.duration_secs).sum();
                        let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();

                        rsx! {
                            div {
                                style: "display: grid; grid-template-columns: 200px 1fr; gap: var(--k-space-6); margin-bottom: var(--k-space-6);",
                                div {
                                    style: "aspect-ratio: 1; background: var(--k-surface-highest); border-radius: var(--k-radius-xl); display: flex; align-items: center; justify-content: center; font-family: var(--k-font-display); font-size: 3rem; color: var(--k-on-surface-variant);",
                                    "{initial}"
                                }
                                div {
                                    style: "display: flex; flex-direction: column; gap: var(--k-space-2); justify-content: center;",
                                    span {
                                        style: "font-family: var(--k-font-mono); color: var(--k-secondary); text-transform: uppercase; font-size: 0.6875rem; letter-spacing: 0.08em;",
                                        "NOW VIEWING"
                                    }
                                    h1 {
                                        style: "font-family: var(--k-font-display); font-size: 2rem; font-weight: 700; color: var(--k-on-surface);",
                                        "{name}"
                                    }
                                    div {
                                        style: "display: flex; align-items: center; gap: var(--k-space-2); color: var(--k-on-surface-variant);",
                                        span { "{artist}" }
                                        Badge { variant: BadgeVariant::Muted, "{genre}" }
                                    }
                                    div {
                                        style: "display: flex; gap: var(--k-space-2); margin-top: var(--k-space-2);",
                                        Button { variant: ButtonVariant::Primary, "Download Album" }
                                        IconButton {
                                            svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                                path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                                            }
                                        }
                                    }
                                }
                            }
                            div { style: "margin-top: var(--k-space-4);",
                                TrackList { tracks: album_tracks, show_download_status: true }
                            }
                            div {
                                style: "font-family: var(--k-font-mono); color: var(--k-on-surface-variant); font-size: 0.75rem; margin-top: var(--k-space-4);",
                                "{track_count} tracks • {format_duration_minutes(total_secs)}"
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { style: "color: var(--k-error);", "Error: {e}" } },
                None => rsx! { p { style: "color: var(--k-on-surface-variant);", "Loading..." } },
            }
        }
    }
}
