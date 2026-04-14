use dioxus::prelude::*;
use dioxus_music_ui::api_client::use_api_client;

use crate::Route;

const PLAYLISTS_CSS: Asset = asset!("/assets/playlists.css");

#[component]
pub fn Playlists() -> Element {
    let client = use_api_client();
    let playlists = use_resource(move || {
        let client = client.clone();
        async move { client.get_playlists().await.ok() }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: PLAYLISTS_CSS }
        div { class: "playlists-page",
            match &*playlists.read() {
                Some(Some(result)) if result.items.is_empty() => rsx! {
                    p { class: "playlists-page__empty", "No playlists yet." }
                },
                Some(Some(result)) => rsx! {
                    div { class: "playlist-list",
                        for p in &result.items {
                            Link {
                                class: "playlist-card",
                                to: Route::PlaylistView { id: p.id },
                                "{p.name}"
                            }
                        }
                    }
                },
                Some(None) => rsx! { p { "Failed to load playlists." } },
                None => rsx! { p { "Loading\u{2026}" } },
            }
        }
    }
}
