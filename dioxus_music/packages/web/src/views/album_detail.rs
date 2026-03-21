use dioxus::prelude::*;

#[component]
pub fn AlbumDetail(name: String) -> Element {
    rsx! {
        div { style: "padding: var(--k-space-4);",
            h2 { style: "font-family: var(--k-font-display); color: var(--k-on-surface);", "Album: {name}" }
            p { style: "color: var(--k-on-surface-variant);", "Album detail coming in next task." }
        }
    }
}
