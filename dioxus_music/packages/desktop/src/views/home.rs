use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 60vh; gap: var(--k-space-4); text-align: center; padding: var(--k-space-8);",
            h1 {
                style: "font-family: var(--k-font-display); font-size: 2.5rem; font-weight: 700; color: var(--k-primary); letter-spacing: -0.02em;",
                {env!("CARGO_PKG_NAME")}
            }
            p {
                style: "font-family: var(--k-font-mono); font-size: 0.75rem; color: var(--secondary-color-4); text-transform: uppercase; letter-spacing: 0.1em;",
                "v{env!(\"CARGO_PKG_VERSION\")}"
            }
        }
    }
}
