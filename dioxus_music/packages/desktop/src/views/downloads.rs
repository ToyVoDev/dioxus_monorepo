use dioxus::prelude::*;

#[component]
pub fn Downloads() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 40vh; color: var(--k-on-surface-variant);",
            h2 { style: "font-family: var(--k-font-display); color: var(--k-on-surface);", "Downloads" }
            p { "Coming soon" }
        }
    }
}
