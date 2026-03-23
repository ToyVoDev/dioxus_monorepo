use crate::views::PlaylistSidebarSection;
use dioxus::prelude::*;

#[component]
pub fn Playlists() -> Element {
    rsx! {
        div { style: "padding: var(--k-space-4);",
            h2 {
                style: "font-family: var(--k-font-display); font-size: 1.75rem; color: var(--secondary-color-3); margin-bottom: var(--k-space-4);",
                "Playlists"
            }
            PlaylistSidebarSection {}
        }
    }
}
