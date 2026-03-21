use dioxus::prelude::*;

#[component]
pub fn Library() -> Element {
    rsx! {
        div {
            style: "padding: var(--k-space-6);",
            div {
                style: "margin-bottom: var(--k-space-6);",
                h1 {
                    style: "font-family: var(--k-font-display); font-size: 1.75rem; font-weight: 700; color: var(--k-on-surface);",
                    "Library"
                }
                p {
                    style: "color: var(--k-on-surface-variant); font-size: 0.875rem; margin-top: var(--k-space-1);",
                    "Your high-fidelity audio repository"
                }
            }
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 30vh; color: var(--k-on-surface-variant);",
                p { "Connect to a server to browse your library" }
            }
        }
    }
}
