use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 60vh; gap: var(--k-space-4); text-align: center; padding: var(--k-space-8);",
            h1 {
                style: "font-family: var(--k-font-display); font-size: 2.5rem; font-weight: 700; color: var(--k-primary); letter-spacing: -0.02em;",
                "KINETIC"
            }
            p {
                style: "font-family: var(--k-font-mono); font-size: 0.75rem; color: var(--k-on-surface-variant); text-transform: uppercase; letter-spacing: 0.1em;",
                "Offline-First Music Player"
            }
            p {
                style: "color: var(--k-on-surface-variant); font-size: 0.875rem; max-width: 400px; line-height: 1.6;",
                "Your high-fidelity localized audio repository. Low latency, zero tracking, pure sound."
            }
            div {
                style: "display: flex; gap: var(--k-space-3); margin-top: var(--k-space-4);",
                p {
                    style: "font-family: var(--k-font-mono); font-size: 0.6875rem; color: var(--k-on-surface-variant); opacity: 0.5;",
                    "Connect to a server to sync your library"
                }
            }
        }
    }
}
