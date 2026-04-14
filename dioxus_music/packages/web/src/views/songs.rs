use dioxus::prelude::*;
use dioxus_music_ui::{TrackList, api_client::use_api_client};

const SONGS_CSS: Asset = asset!("/assets/songs.css");

#[component]
pub fn Songs() -> Element {
    let client = use_api_client();
    let tracks = use_resource(move || {
        let client = client.clone();
        async move { client.get_tracks().await.ok() }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: SONGS_CSS }
        div { class: "songs",
            match &*tracks.read() {
                Some(Some(result)) => rsx! {
                    TrackList { tracks: result.items.clone() }
                },
                Some(None) => rsx! { p { "Failed to load songs." } },
                None => rsx! { p { "Loading\u{2026}" } },
            }
        }
    }
}
