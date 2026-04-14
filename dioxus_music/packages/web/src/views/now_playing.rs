use dioxus::prelude::*;
use dioxus_music_ui::{api_client::ApiClient, player_state::use_player_state};

#[component]
pub fn NowPlaying() -> Element {
    let player = use_player_state();
    let client = use_context::<ApiClient>();
    let state = player.read();

    rsx! {
        div { class: "now-playing",
            if let Some(track) = &state.current_track {
                div { class: "now-playing__art",
                    if track.image_tags.as_ref().and_then(|t| t.get("Primary")).is_some() {
                        img { src: client.image_url(track.id, "Primary") }
                    }
                }
                div { class: "now-playing__info",
                    h2 { "{track.name}" }
                    if let Some(artists) = &track.artists {
                        if let Some(artist) = artists.first() {
                            p { "{artist}" }
                        }
                    }
                    if let Some(album) = &track.album {
                        p { "{album}" }
                    }
                }
            } else {
                p { "Nothing playing." }
            }
        }
    }
}
