use dioxus::prelude::*;
use dioxus_music_ui::{api_client::use_api_client, TrackList};
use uuid::Uuid;

#[component]
pub fn PlaylistView(id: Uuid) -> Element {
    let client = use_api_client();
    let client_playlist = client.clone();

    let playlist = use_resource(move || {
        let client = client_playlist.clone();
        async move { client.get_playlist(id).await.ok() }
    });

    let tracks = use_resource(move || {
        let client = client.clone();
        async move { client.get_playlist_items(id).await.ok() }
    });

    rsx! {
        div { class: "playlist-view",
            match &*playlist.read() {
                Some(Some(p)) => rsx! { h1 { "{p.name}" } },
                _ => rsx! { h1 { "Playlist" } },
            }
            match &*tracks.read() {
                Some(Some(result)) => rsx! {
                    TrackList { tracks: result.items.clone() }
                },
                Some(None) => rsx! { p { "Failed to load tracks." } },
                None => rsx! { p { "Loading…" } },
            }
        }
    }
}
