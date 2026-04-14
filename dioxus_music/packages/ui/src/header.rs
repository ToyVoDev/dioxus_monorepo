use crate::api_client::use_api_client;
use dioxus::prelude::*;
use kinetic_ui::{IconButton, SearchInput};

const HEADER_CSS: Asset = asset!("/assets/styling/header.css");
const SETTINGS_CSS: Asset = asset!("/assets/styling/settings-dropdown.css");

#[component]
pub fn Header() -> Element {
    let mut settings_open = use_signal(|| false);
    let api_client = use_api_client();

    rsx! {
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        document::Link { rel: "stylesheet", href: SETTINGS_CSS }
        header { class: "header",
            span { class: "header__brand", {env!("CARGO_PKG_NAME")} }
            div { class: "header__spacer" }
            div { class: "header__actions",
                SearchInput { placeholder: "Search...".to_string() }
                div { class: "header__sync-dot", title: "Synced" }
                div { class: "settings-dropdown",
                    IconButton {
                        onclick: move |_| {
                            settings_open.set(!settings_open());
                        },
                        // Settings icon (gear SVG)
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                    }
                    if settings_open() {
                        div { class: "settings-dropdown__menu",
                            button {
                                class: "settings-dropdown__item",
                                onclick: move |_| {
                                    settings_open.set(false);
                                    let client = api_client.clone();
                                    spawn(async move {
                                        let _ = client.rescan_library().await;
                                    });
                                },
                                "Rescan Library"
                            }
                        }
                    }
                }
                IconButton {
                    // Account icon (user SVG)
                    svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                        circle { cx: "12", cy: "7", r: "4" }
                    }
                }
            }
        }
    }
}
