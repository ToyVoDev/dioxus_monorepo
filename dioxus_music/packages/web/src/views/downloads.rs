use dioxus::prelude::*;

const DOWNLOADS_CSS: Asset = asset!("/assets/downloads.css");

#[component]
pub fn Downloads() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DOWNLOADS_CSS }

        div { class: "sync-manager",
            // Header
            div { class: "sync-manager__header",
                h1 { class: "sync-manager__title", "Sync Manager" }
                div { class: "sync-manager__status",
                    span { class: "sync-manager__status-dot" }
                    span { "Last synced: —" }
                }
            }

            // Bento grid
            div { class: "sync-manager__bento",
                // Storage card
                div { class: "sync-manager__card sync-manager__card--wide",
                    div { class: "sync-manager__card-header",
                        span { class: "sync-manager__card-title", "Local Storage" }
                        span { class: "sync-manager__card-subtitle", "— GB Available" }
                    }
                    div { class: "sync-manager__storage-bar",
                        div { class: "sync-manager__storage-fill", style: "width: 0%;" }
                    }
                    div { class: "sync-manager__storage-legend",
                        div { class: "sync-manager__legend-item",
                            span { class: "sync-manager__legend-dot sync-manager__legend-dot--music" }
                            "Music"
                        }
                        div { class: "sync-manager__legend-item",
                            span { class: "sync-manager__legend-dot sync-manager__legend-dot--cache" }
                            "Cache"
                        }
                    }
                }

                // Active downloads card
                div { class: "sync-manager__card",
                    div { class: "sync-manager__card-icon",
                        svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line { x1: "12", y1: "15", x2: "12", y2: "3" }
                        }
                    }
                    div { class: "sync-manager__card-value", "0" }
                    div { class: "sync-manager__card-label", "Tracks in queue" }
                }

                // Sync status card
                div { class: "sync-manager__card",
                    div { class: "sync-manager__card-icon",
                        svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            polyline { points: "23 4 23 10 17 10" }
                            path { d: "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" }
                        }
                    }
                    div { class: "sync-manager__card-value", "Idle" }
                    div { class: "sync-manager__card-label", "Sync status" }
                }
            }

            // Synced library
            div { class: "sync-manager__section",
                div { class: "sync-manager__section-header",
                    div { class: "sync-manager__section-accent sync-manager__section-accent--secondary" }
                    span { class: "sync-manager__section-title", "SYNCED LIBRARY" }
                }
                div { class: "sync-manager__empty",
                    svg { width: "48", height: "48", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line { x1: "12", y1: "15", x2: "12", y2: "3" }
                    }
                    p { class: "sync-manager__empty-title", "No offline content yet" }
                    p { class: "sync-manager__empty-hint", "Synced albums and playlists will appear here. Automatic sync keeps your library up to date." }
                }
            }
        }
    }
}
