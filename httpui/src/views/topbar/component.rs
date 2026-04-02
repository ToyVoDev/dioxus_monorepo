use crate::state::{AppState, TopBarNav};
use dioxus::prelude::*;
use dioxus_free_icons::icons::md_action_icons::{MdAccountCircle, MdSettings};
use dioxus_free_icons::Icon;
use kinetic_ui::{
    IconButton, KSelect, KSelectList, KSelectOption, KSelectTrigger, KSelectValue, SearchInput,
};
use strum::IntoEnumIterator;

#[component]
pub fn TopBar() -> Element {
    let mut state = use_context::<AppState>();
    let active_nav = state.active_topbar_nav;

    // Get environments from the selected space, or first space if none selected
    let environments: Vec<String> = {
        let spaces = state.spaces.read();
        let selected_space_id = state.selected_space.read();

        let space = if let Some(id) = *selected_space_id {
            spaces.iter().find(|s| s.id == id)
        } else {
            spaces.first()
        };

        space
            .map(|s| s.environments.iter().map(|e| e.name.clone()).collect())
            .unwrap_or_default()
    };

    rsx! {
            document::Link { rel: "stylesheet", href: asset!("./style.css") }

            div { class: "topbar",
                // Brand
                span { class: "topbar__brand", {env!("CARGO_PKG_NAME")} }

                // Nav links
                nav { class: "topbar__nav",
                    for item in TopBarNav::iter() {
                        {
                            let is_active = active_nav() == item;
                            let label = item.to_string();
                            rsx! {
                                button {
                                    class: "topbar__nav-link",
                                    "data-active": is_active.to_string(),
                                    onclick: move |_| {
                                        state.active_topbar_nav.set(item);
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }

                // Spacer
                div { class: "topbar__spacer" }

                // Actions
                div { class: "topbar__actions",
                    // Search
                    SearchInput { placeholder: "Search...".to_string() }

                    // Environment selector
                    KSelect::<String> {
                        on_value_change: move |selection: Option<String>| {
                            if let Some(selected) = selection {
                                tracing::info!("Environment selected: {selected}");
                            }
                        },
                        KSelectTrigger {
    KSelectValue {
                                aria_placeholder: if environments.is_empty() {
                                    "No environments".to_string()
                                } else {
                                    "Select environment".to_string()
                                }
                            }
                        }
                        KSelectList {
                            for (i, env_name) in environments.iter().enumerate() {
                                KSelectOption::<String> {
                                    index: i,
                                    value: env_name.clone(),
                                    text_value: env_name.clone(),
                                    "{env_name}"
                                }
                            }
                        }
                    }

                    // Settings
                    IconButton {
                        Icon { icon: MdSettings, width: 20, height: 20 }
                    }

                    // Account
                    IconButton {
                        Icon { icon: MdAccountCircle, width: 20, height: 20 }
                    }
                }
            }
        }
}
