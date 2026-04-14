use crate::api_client::use_api_client;
use crate::player_state::{RepeatMode, use_player_state};
use dioxus::prelude::*;

pub fn render_audio_element() -> Element {
    let mut player = use_player_state();
    let track_info = player.read().current_track.clone();
    let repeat_mode = player.read().repeat_mode;

    let client = use_api_client();
    let audio_src = track_info
        .as_ref()
        .map(|t| client.stream_url(t.id))
        .unwrap_or_default();
    let has_track = track_info.is_some();
    let is_looping = repeat_mode == RepeatMode::One;

    rsx! {
        if has_track {
            audio {
                id: "main-audio",
                src: "{audio_src}",
                autoplay: true,
                r#loop: is_looping,
                onended: move |_| {
                    player.with_mut(|p| p.next_track());
                    let _ = document::eval(r#"
                        let a = document.getElementById('main-audio');
                        if (a && a.src) { a.load(); a.play(); }
                    "#);
                },
            }
        }
    }
}
