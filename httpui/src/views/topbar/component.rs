use crate::state::{AppState, TopBarNav};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_action_icons::{MdAccountCircle, MdSettings};
use kinetic_ui::{
    IconButton, KSelect, KSelectList, KSelectOption, KSelectTrigger, KSelectValue, SearchInput,
};
use strum::IntoEnumIterator;

#[component]
pub fn TopBar() -> Element {
    let mut state = use_context::<AppState>();
    let active_nav = state.active_topbar_nav;

    let environments: Signal<Vec<String>> = use_signal(|| {
        vec![
            String::from("Development"),
            String::from("Staging"),
            String::from("Production"),
        ]
    });

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
                            println!("Environment selected: {selected}");
                        }
                    },
                    KSelectTrigger {
                        KSelectValue {}
                    }
                    KSelectList {
                        for (i, env_name) in environments().iter().enumerate() {
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
