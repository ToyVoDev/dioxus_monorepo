use crate::state::{AppState, SideNavItem};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_action_icons::{MdDns, MdHelp, MdHistory, MdSettings};
use dioxus_free_icons::icons::md_file_icons::MdFolder;
use kinetic_ui::{Button, ButtonVariant};

#[component]
pub fn SideNav() -> Element {
    let mut state = use_context::<AppState>();
    let active = state.active_sidebar_nav;

    let nav_items: Vec<(SideNavItem, &str, Element)> = vec![
        (
            SideNavItem::Collections,
            "Collections",
            rsx! { Icon { icon: MdFolder, width: 20, height: 20 } },
        ),
        (
            SideNavItem::History,
            "History",
            rsx! { Icon { icon: MdHistory, width: 20, height: 20 } },
        ),
        (
            SideNavItem::Apis,
            "APIs",
            rsx! { Icon { icon: MdDns, width: 20, height: 20 } },
        ),
        (
            SideNavItem::MockServers,
            "Mock",
            rsx! { Icon { icon: MdDns, width: 20, height: 20 } },
        ),
    ];

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "sidenav",
            // Brand
            div { class: "sidenav__brand",
                {env!("CARGO_PKG_NAME")}
                span { class: "sidenav__version", "v{env!(\"CARGO_PKG_VERSION\")}" }
            }

            // CTA: new request button
            div { class: "sidenav__cta",
                Button { variant: ButtonVariant::Primary, "+" }
            }

            // Nav items
            nav { class: "sidenav__nav",
                for (item, label, icon) in nav_items {
                    button {
                        class: "sidenav__nav-item",
                        "data-active": if active() == item { "true" } else { "false" },
                        onclick: move |_| state.active_sidebar_nav.set(item),
                        {icon}
                        "{label}"
                    }
                }
            }

            // Footer
            div { class: "sidenav__footer",
                button {
                    class: "sidenav__nav-item",
                    Icon { icon: MdSettings, width: 20, height: 20 }
                    "Settings"
                }
                button {
                    class: "sidenav__nav-item",
                    Icon { icon: MdHelp, width: 20, height: 20 }
                    "Help"
                }
            }
        }
    }
}
