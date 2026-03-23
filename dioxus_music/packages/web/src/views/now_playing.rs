use dioxus::prelude::*;
use dioxus_music_ui::player_state::{RepeatMode, use_player_state};

const NOW_PLAYING_CSS: Asset = asset!("/assets/now_playing.css");

fn format_duration(secs: i32) -> String {
    format!("{}:{:02}", secs / 60, secs % 60)
}

#[component]
pub fn NowPlaying() -> Element {
    let mut player = use_player_state();
    let nav = navigator();

    let track_info = player.read().current_track.clone();
    let is_playing = player.read().is_playing;
    let repeat_mode = player.read().repeat_mode;
    let is_shuffled = player.read().is_shuffled;
    let queue = player.read().queue.clone();
    let queue_index = player.read().queue_index;

    let has_track = track_info.is_some();

    let repeat_label = match repeat_mode {
        RepeatMode::Off => "R",
        RepeatMode::All => "RA",
        RepeatMode::One => "R1",
    };
    let repeat_active = repeat_mode != RepeatMode::Off;

    rsx! {
        document::Link { rel: "stylesheet", href: NOW_PLAYING_CSS }

        div { class: "now-playing",
            button {
                class: "now-playing__back",
                onclick: move |_| { nav.go_back(); },
                "\u{2190} Back"
            }

            if let Some(track) = &track_info {
                // Get first character of album for art placeholder
                {
                    let album_initial = track.album.chars().next().unwrap_or('?').to_string();
                    let duration_str = format_duration(track.duration_secs);

                    rsx! {
                        div { class: "now-playing__layout",
                            // Left column
                            div { class: "now-playing__left",
                                div { class: "now-playing__art", "{album_initial}" }
                                div { class: "now-playing__title", "{track.title}" }
                                div { class: "now-playing__artist", "{track.artist}" }

                                // Progress bar (visual only)
                                div { class: "now-playing__progress",
                                    span { class: "now-playing__progress-time", "0:00" }
                                    div { class: "now-playing__progress-bar",
                                        div { class: "now-playing__progress-fill" }
                                    }
                                    span { class: "now-playing__progress-time", "{duration_str}" }
                                }

                                // Transport controls
                                div { class: "now-playing__controls",
                                    button {
                                        class: if is_shuffled { "now-playing__btn now-playing__btn--active" } else { "now-playing__btn" },
                                        disabled: !has_track,
                                        onclick: move |_| {
                                            player.with_mut(|p| p.toggle_shuffle());
                                        },
                                        "Sh"
                                    }
                                    button {
                                        class: "now-playing__btn",
                                        disabled: !has_track,
                                        onclick: move |_| {
                                            player.with_mut(|p| p.prev_track());
                                            let _ = document::eval(r#"
                                                let a = document.getElementById('main-audio');
                                                if (a) { a.load(); a.play(); }
                                            "#);
                                        },
                                        "\u{23EE}"
                                    }
                                    button {
                                        class: "now-playing__btn now-playing__btn--play",
                                        disabled: !has_track,
                                        onclick: move |_| {
                                            let currently_playing = player.read().is_playing;
                                            if currently_playing {
                                                player.with_mut(|p| p.is_playing = false);
                                                let _ = document::eval("document.getElementById('main-audio')?.pause()");
                                            } else {
                                                player.with_mut(|p| p.is_playing = true);
                                                let _ = document::eval("document.getElementById('main-audio')?.play()");
                                            }
                                        },
                                        if is_playing { "\u{23F8}" } else { "\u{25B6}" }
                                    }
                                    button {
                                        class: "now-playing__btn",
                                        disabled: !has_track,
                                        onclick: move |_| {
                                            player.with_mut(|p| p.next_track());
                                            let _ = document::eval(r#"
                                                let a = document.getElementById('main-audio');
                                                if (a) { a.load(); a.play(); }
                                            "#);
                                        },
                                        "\u{23ED}"
                                    }
                                    button {
                                        class: if repeat_active { "now-playing__btn now-playing__btn--active" } else { "now-playing__btn" },
                                        disabled: !has_track,
                                        onclick: move |_| {
                                            player.with_mut(|p| p.toggle_repeat());
                                        },
                                        "{repeat_label}"
                                    }
                                }

                                // Mobile-only chips
                                div { class: "now-playing__chips",
                                    span { class: "now-playing__chip", "FLAC" }
                                    span { class: "now-playing__chip", "Local" }
                                }
                            }

                            // Right column (hidden on mobile via CSS)
                            div { class: "now-playing__right",
                                span { class: "now-playing__section-label", "CURRENTLY STREAMING" }

                                // Bento cards
                                div { class: "now-playing__bento",
                                    div { class: "now-playing__bento-card",
                                        span { class: "now-playing__bento-value", "FLAC" }
                                        span { class: "now-playing__bento-label", "Codec" }
                                    }
                                    div { class: "now-playing__bento-card",
                                        span { class: "now-playing__bento-value", "\u{2014} kbps" }
                                        span { class: "now-playing__bento-label", "Bitrate" }
                                    }
                                    div { class: "now-playing__bento-card",
                                        span { class: "now-playing__bento-value", "Local" }
                                        span { class: "now-playing__bento-label", "Source" }
                                    }
                                }

                                // Queue section
                                div { class: "now-playing__queue-header",
                                    span { class: "now-playing__section-label", "UP NEXT" }
                                    span { class: "now-playing__queue-count", "{queue.len()} tracks" }
                                }
                                div { class: "now-playing__queue",
                                    for (i, q_track) in queue.iter().enumerate() {
                                        div {
                                            key: "{q_track.id}",
                                            class: if i == queue_index { "now-playing__queue-item now-playing__queue-item--active" } else { "now-playing__queue-item" },
                                            onclick: move |_| {
                                                player.with_mut(|p| p.jump_to(i));
                                                let _ = document::eval(r#"
                                                    let a = document.getElementById('main-audio');
                                                    if (a) { a.load(); a.play(); }
                                                "#);
                                            },
                                            div {
                                                span { class: "now-playing__queue-title", "{q_track.title}" }
                                                span { class: "now-playing__queue-artist", "{q_track.artist}" }
                                            }
                                        }
                                    }
                                }

                                // Volume slider (visual only)
                                div { class: "now-playing__volume",
                                    span { "\u{1F50A}" }
                                    div { class: "now-playing__volume-bar",
                                        div { class: "now-playing__volume-fill" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "now-playing__placeholder",
                    "No track playing \u{2014} select a song from your library"
                }
            }
        }
    }
}
