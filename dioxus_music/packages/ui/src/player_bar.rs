use crate::player_state::{RepeatMode, use_player_state};
use dioxus::prelude::*;

const PLAYER_BAR_CSS: Asset = asset!("/assets/styling/player-bar.css");

#[component]
pub fn PlayerBar(
    #[props(default)] compact: bool,
    #[props(default)] hidden: bool,
    #[props(default)] on_expand: Option<EventHandler<()>>,
) -> Element {
    if hidden {
        return rsx! {};
    }

    let mut player = use_player_state();

    let track_info = player.read().current_track.clone();
    let is_playing = player.read().is_playing;
    let repeat_mode = player.read().repeat_mode;
    let is_shuffled = player.read().is_shuffled;
    let show_queue = player.read().show_queue;

    let has_track = track_info.is_some();

    let repeat_label = match repeat_mode {
        RepeatMode::Off => "R",
        RepeatMode::All => "RA",
        RepeatMode::One => "R1",
    };
    let repeat_active = repeat_mode != RepeatMode::Off;

    let bar_class = if compact {
        "player-bar player-bar--compact"
    } else {
        "player-bar"
    };

    rsx! {
        document::Link { rel: "stylesheet", href: PLAYER_BAR_CSS }

        div { class: "{bar_class}",
            // Clickable info area (album art + track info)
            div {
                class: "player-bar__info-area",
                style: if on_expand.is_some() { "cursor: pointer;" } else { "" },
                onclick: move |_| {
                    if let Some(ref handler) = on_expand {
                        handler.call(());
                    }
                },
                // Album art placeholder
                div { class: "player-bar__art" }

                // Now playing info
                div { class: "player-bar__info",
                    if let Some(track) = &track_info {
                        div { class: "player-bar__title", "{track.name}" }
                        div { class: "player-bar__artist",
                            {
                                track.artists
                                    .as_ref()
                                    .and_then(|a| a.first())
                                    .map(|s| s.as_str())
                                    .unwrap_or("")
                            }
                        }
                    } else {
                        div { class: "player-bar__title player-bar__title--empty", "No track selected" }
                    }
                }
            }

            // Spacer
            div { class: "player-bar__spacer" }

            // Transport controls
            div { class: "player-bar__controls",
                if !compact {
                    // Shuffle button
                    button {
                        class: if is_shuffled { "player-bar__btn player-bar__btn--secondary player-bar__btn--active" } else { "player-bar__btn player-bar__btn--secondary" },
                        disabled: !has_track,
                        onclick: move |_| {
                            player.with_mut(|p| p.toggle_shuffle());
                        },
                        "Sh"
                    }

                    button {
                        class: "player-bar__btn",
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
                }

                button {
                    class: "player-bar__btn player-bar__btn--play",
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
                    class: "player-bar__btn",
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

                if !compact {
                    // Repeat button
                    button {
                        class: if repeat_active { "player-bar__btn player-bar__btn--secondary player-bar__btn--active" } else { "player-bar__btn player-bar__btn--secondary" },
                        disabled: !has_track,
                        onclick: move |_| {
                            player.with_mut(|p| p.toggle_repeat());
                        },
                        "{repeat_label}"
                    }

                    // Queue button
                    button {
                        class: if show_queue { "player-bar__btn player-bar__btn--secondary player-bar__btn--active" } else { "player-bar__btn player-bar__btn--secondary" },
                        onclick: move |_| {
                            player.with_mut(|p| p.toggle_queue());
                        },
                        "Q"
                    }
                }
            }

            // Placeholder progress bar
            if !compact {
                div { class: "player-bar__progress",
                    span { "0:00" }
                    div { class: "player-bar__progress-bar",
                        div { class: "player-bar__progress-fill" }
                    }
                    span { "0:00" }
                }
            }

        }
    }
}
