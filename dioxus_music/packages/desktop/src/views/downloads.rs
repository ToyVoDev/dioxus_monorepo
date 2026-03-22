use dioxus::prelude::*;

#[component]
pub fn Downloads() -> Element {
    rsx! {
        div { style: "padding: var(--k-space-6);",
            // Header
            div { style: "margin-bottom: var(--k-space-6);",
                h1 {
                    style: "font-family: var(--k-font-display); font-size: 2rem; font-weight: 700; color: var(--k-on-surface); margin-bottom: var(--k-space-1);",
                    "Sync Manager"
                }
                div {
                    style: "display: flex; align-items: center; gap: var(--k-space-2); font-family: var(--k-font-mono); font-size: 0.75rem; color: var(--k-on-surface-variant);",
                    span { style: "width: 6px; height: 6px; border-radius: 50%; background: var(--k-on-surface-variant);" }
                    span { "Last synced: —" }
                }
            }

            // Bento grid
            div {
                style: "display: grid; grid-template-columns: 2fr 1fr 1fr; gap: var(--k-space-3); margin-bottom: var(--k-space-6);",

                // Storage card
                div {
                    style: "background: var(--k-surface-low); border-radius: var(--k-radius-lg); padding: var(--k-space-4); display: flex; flex-direction: column; gap: var(--k-space-2);",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: baseline;",
                        span { style: "font-family: var(--k-font-display); font-size: 1rem; font-weight: 600; color: var(--k-on-surface);", "Local Storage" }
                        span { style: "font-size: 0.75rem; color: var(--k-on-surface-variant);", "— GB Available" }
                    }
                    div {
                        style: "height: 8px; background: var(--k-surface-highest); border-radius: 4px; overflow: hidden;",
                        div { style: "height: 100%; background: var(--k-primary); border-radius: 4px; width: 0%;" }
                    }
                    div {
                        style: "display: flex; gap: var(--k-space-4); margin-top: var(--k-space-1);",
                        div {
                            style: "display: flex; align-items: center; gap: var(--k-space-1); font-size: 0.6875rem; color: var(--k-on-surface-variant);",
                            span { style: "width: 8px; height: 8px; border-radius: 50%; background: var(--k-primary);" }
                            "Music"
                        }
                        div {
                            style: "display: flex; align-items: center; gap: var(--k-space-1); font-size: 0.6875rem; color: var(--k-on-surface-variant);",
                            span { style: "width: 8px; height: 8px; border-radius: 50%; background: var(--k-secondary);" }
                            "Cache"
                        }
                    }
                }

                // Active downloads
                div {
                    style: "background: var(--k-surface-low); border-radius: var(--k-radius-lg); padding: var(--k-space-4); display: flex; flex-direction: column; gap: var(--k-space-2);",
                    svg { style: "color: var(--k-on-surface-variant);", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line { x1: "12", y1: "15", x2: "12", y2: "3" }
                    }
                    span { style: "font-family: var(--k-font-display); font-size: 2rem; font-weight: 700; color: var(--k-on-surface);", "0" }
                    span { style: "font-size: 0.75rem; color: var(--k-on-surface-variant);", "Tracks in queue" }
                }

                // Sync status
                div {
                    style: "background: var(--k-surface-low); border-radius: var(--k-radius-lg); padding: var(--k-space-4); display: flex; flex-direction: column; gap: var(--k-space-2);",
                    svg { style: "color: var(--k-on-surface-variant);", width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        polyline { points: "23 4 23 10 17 10" }
                        path { d: "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" }
                    }
                    span { style: "font-family: var(--k-font-display); font-size: 2rem; font-weight: 700; color: var(--k-on-surface);", "Idle" }
                    span { style: "font-size: 0.75rem; color: var(--k-on-surface-variant);", "Sync status" }
                }
            }

            // Synced Library section
            div {
                div {
                    style: "display: flex; align-items: center; gap: var(--k-space-2); margin-bottom: var(--k-space-3);",
                    span { style: "width: 3px; height: 16px; border-radius: 2px; background: var(--k-secondary);" }
                    span {
                        style: "font-family: var(--k-font-mono); font-size: 0.6875rem; font-weight: 600; color: var(--k-on-surface-variant); text-transform: uppercase; letter-spacing: 0.08em;",
                        "SYNCED LIBRARY"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: var(--k-space-8); border: 1px dashed rgba(91, 64, 62, 0.2); border-radius: var(--k-radius-lg); text-align: center; color: var(--k-on-surface-variant); gap: var(--k-space-2);",
                    svg { width: "48", height: "48", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line { x1: "12", y1: "15", x2: "12", y2: "3" }
                    }
                    p { style: "font-size: 1rem; color: var(--k-on-surface); margin-top: var(--k-space-2);", "No offline content yet" }
                    p { style: "font-size: 0.8125rem; max-width: 400px; line-height: 1.5;", "Synced albums and playlists will appear here. Automatic sync keeps your library up to date." }
                }
            }
        }
    }
}
