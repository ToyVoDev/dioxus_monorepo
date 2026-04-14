use dioxus::prelude::*;

use super::PlaylistSidebarSection;

#[component]
pub fn Playlists() -> Element {
    rsx! {
        div { class: "playlists-page",
            PlaylistSidebarSection {}
        }
    }
}
