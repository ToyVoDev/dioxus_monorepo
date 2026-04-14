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
                ontimeupdate: move |_| {
                    spawn(async move {
                        let mut eval = document::eval(r#"
                            let a = document.getElementById('main-audio');
                            dioxus.send(a ? [a.currentTime, isFinite(a.duration) ? a.duration : 0] : [0, 0]);
                        "#);
                        if let Ok(val) = eval.recv::<serde_json::Value>().await {
                            if let (Some(ct), Some(dur)) = (
                                val.get(0).and_then(|v| v.as_f64()),
                                val.get(1).and_then(|v| v.as_f64()),
                            ) {
                                player.with_mut(|p| {
                                    p.current_time = ct;
                                    p.duration = dur;
                                });
                            }
                        }
                    });
                },
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
