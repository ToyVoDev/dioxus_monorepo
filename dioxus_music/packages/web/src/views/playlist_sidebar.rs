use dioxus::prelude::*;
use dioxus_music_ui::api_client::use_api_client;

use crate::Route;

#[component]
pub fn PlaylistSidebarSection() -> Element {
    let client = use_api_client();
    let playlists = use_resource(move || {
        let client = client.clone();
        async move { client.get_playlists().await.ok() }
    });

    rsx! {
        div { class: "playlist-sidebar",
            match &*playlists.read() {
                Some(Some(result)) => rsx! {
                    for p in &result.items {
                        Link {
                            class: "sidebar__nav-item",
                            to: Route::PlaylistView { id: p.id },
                            "{p.name}"
                        }
                    }
                },
                _ => rsx! {},
            }
        }
    }
}
