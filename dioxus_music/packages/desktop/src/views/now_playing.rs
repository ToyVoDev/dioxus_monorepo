use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::player_state::{RepeatMode, use_player_state};

const NOW_PLAYING_CSS: Asset = asset!("/assets/now_playing.css");

fn format_duration(ticks: i64) -> String {
    let secs = (ticks / 10_000_000) as i32;
    let m = secs / 60;
    let s = secs % 60;
    format!("{m}:{s:02}")
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
                onclick: move |_| {
                    nav.push(Route::Library {});
                },
                "\u{2190} Back"
            }

            if let Some(track) = &track_info {
                {
                    let artist = track.artists.as_ref().and_then(|a| a.first()).cloned().unwrap_or_default();
                    let album = track.album.as_deref().unwrap_or("").to_string();
                    let genre = track.genres.as_ref().and_then(|g| g.first()).cloned().unwrap_or_default();
                    let duration_ticks = track.run_time_ticks.unwrap_or(0);

                    rsx! {
                        div { class: "now-playing__layout",
                            // Left column: art, title, controls
                            div { class: "now-playing__left",
                                div { class: "now-playing__art",
                                    "{track.name.chars().next().unwrap_or('?')}"
                                }
                                div { class: "now-playing__title", "{track.name}" }
                                div { class: "now-playing__artist", "{artist}" }

                                // Progress bar (placeholder)
                                div { class: "now-playing__progress",
                                    span { "0:00" }
                                    div { class: "now-playing__progress-bar",
                                        div { class: "now-playing__progress-fill" }
                                    }
                                    span { "{format_duration(duration_ticks)}" }
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

                                // Chips row (shown at 768px, hidden on desktop)
                                div { class: "now-playing__chips",
                                    span { "{album}" }
                                    if !genre.is_empty() {
                                        span { "{genre}" }
                                    }
                                    span { "{format_duration(duration_ticks)}" }
                                }
                            }

                            // Right column: metadata, queue, volume
                            div { class: "now-playing__right",
                                // Metadata bento
                                span { class: "now-playing__section-label", "Details" }
                                div { class: "now-playing__bento",
                                    div { class: "now-playing__bento-card",
                                        span { class: "now-playing__bento-card__label", "Album" }
                                        span { class: "now-playing__bento-card__value", "{album}" }
                                    }
                                    div { class: "now-playing__bento-card",
                                        span { class: "now-playing__bento-card__label", "Genre" }
                                        span { class: "now-playing__bento-card__value",
                                            {
                                                if genre.is_empty() { "Unknown".to_string() } else { genre.clone() }
                                            }
                                        }
                                    }
                                    div { class: "now-playing__bento-card",
                                        span { class: "now-playing__bento-card__label", "Duration" }
                                        span { class: "now-playing__bento-card__value",
                                            "{format_duration(duration_ticks)}"
                                        }
                                    }
                                }

                                // Queue
                                if !queue.is_empty() {
                                    span { class: "now-playing__section-label", "Up Next" }
                                    div { class: "now-playing__queue",
                                        for (i, q_track) in queue.iter().enumerate() {
                                            {
                                                let is_active = i == queue_index;
                                                let item_class = if is_active {
                                                    "now-playing__queue-item now-playing__queue-item--active"
                                                } else {
                                                    "now-playing__queue-item"
                                                };
                                                let q_ticks = q_track.run_time_ticks.unwrap_or(0);
                                                rsx! {
                                                    div {
                                                        key: "{q_track.id}",
                                                        class: "{item_class}",
                                                        onclick: move |_| {
                                                            player.with_mut(|p| p.jump_to(i));
                                                            let _ = document::eval(r#"
                                                                let a = document.getElementById('main-audio');
                                                                if (a) { a.load(); a.play(); }
                                                            "#);
                                                        },
                                                        span { class: "now-playing__queue-item__number", "{i + 1}" }
                                                        span { class: "now-playing__queue-item__title", "{q_track.name}" }
                                                        span { class: "now-playing__queue-item__duration",
                                                            "{format_duration(q_ticks)}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Volume (placeholder)
                                div { class: "now-playing__volume",
                                    span { "\u{1F50A}" }
                                    span { "Volume" }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "now-playing__placeholder",
                    "No track playing"
                }
            }
        }
    }
}
