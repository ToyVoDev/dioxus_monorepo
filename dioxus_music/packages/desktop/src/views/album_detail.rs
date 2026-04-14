use dioxus::prelude::*;

#[component]
pub fn AlbumDetail(name: String) -> Element {
    rsx! { p { "Album: {name} — coming soon" } }
}
