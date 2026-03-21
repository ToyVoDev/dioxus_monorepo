use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AlbumDetail(name: String) -> Element {
    rsx! {
        div {
            style: "padding: var(--k-space-6);",
            Link {
                style: "color: var(--k-on-surface-variant); text-decoration: none; font-size: 0.875rem;",
                to: Route::Library {},
                "← Back"
            }
            h1 {
                style: "font-family: var(--k-font-display); font-size: 2rem; font-weight: 700; color: var(--k-on-surface); margin-top: var(--k-space-4);",
                "{name}"
            }
            p {
                style: "color: var(--k-on-surface-variant); margin-top: var(--k-space-2);",
                "Connect to a server to view album details"
            }
        }
    }
}
