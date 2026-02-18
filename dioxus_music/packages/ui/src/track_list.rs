use crate::player_state::use_player_state;
use dioxus::prelude::*;
use dioxus_music_api::models::TrackSummary;

const TRACK_LIST_CSS: Asset = asset!("/assets/styling/track_list.css");

fn format_duration(secs: i32) -> String {
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{minutes}:{seconds:02}")
}

#[component]
pub fn TrackList(tracks: Vec<TrackSummary>) -> Element {
    let mut player = use_player_state();
    let current_id = player.read().current_track.as_ref().map(|t| t.id);

    rsx! {
        document::Link { rel: "stylesheet", href: TRACK_LIST_CSS }

        div { class: "track-list",
            div { class: "track-list__header",
                span { class: "track-list__col track-list__col--num", "#" }
                span { class: "track-list__col track-list__col--title", "Title" }
                span { class: "track-list__col track-list__col--artist", "Artist" }
                span { class: "track-list__col track-list__col--album", "Album" }
                span { class: "track-list__col track-list__col--duration", "Duration" }
            }

            for (idx, track) in tracks.iter().enumerate() {
                {
                    let track_clone = track.clone();
                    let all_tracks = tracks.clone();
                    let is_active = current_id == Some(track.id);

                    rsx! {
                        div {
                            class: if is_active { "track-list__row track-list__row--active" } else { "track-list__row" },
                            onclick: move |_| {
                                let t = track_clone.clone();
                                let q = all_tracks.clone();
                                player.with_mut(|p| p.play_track(t, q, idx));
                                let _ = document::eval(r#"
                                    let a = document.getElementById('main-audio');
                                    if (a) { a.load(); a.play(); }
                                "#);
                            },
                            span { class: "track-list__col track-list__col--num", "{idx + 1}" }
                            span { class: "track-list__col track-list__col--title", "{track.title}" }
                            span { class: "track-list__col track-list__col--artist", "{track.artist}" }
                            span { class: "track-list__col track-list__col--album", "{track.album}" }
                            span { class: "track-list__col track-list__col--duration", "{format_duration(track.duration_secs)}" }
                        }
                    }
                }
            }
        }
    }
}
