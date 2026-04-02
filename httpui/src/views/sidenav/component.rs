use crate::state::{AppState, CreateType, SideNavItem};
use dioxus::prelude::*;
use dioxus_free_icons::icons::md_action_icons::{MdDns, MdHelp, MdHistory, MdSettings};
use dioxus_free_icons::icons::md_file_icons::MdFolder;
use dioxus_free_icons::Icon;
use kinetic_ui::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};

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
                DropdownMenu {
                    DropdownMenuTrigger {
                        class: "sidenav__add-btn",
                        "+"
                    }
                    DropdownMenuContent {
                        DropdownMenuItem::<CreateType> {
                            value: CreateType::Space,
                            index: 0usize,
                            on_select: move |create_type: CreateType| {
                                state.create_modal_type.set(Some(create_type));
                            },
                            "New Space"
                        }
                        DropdownMenuItem::<CreateType> {
                            value: CreateType::Collection,
                            index: 1usize,
                            on_select: move |create_type: CreateType| {
                                state.create_modal_type.set(Some(create_type));
                            },
                            "New Collection"
                        }
                        DropdownMenuItem::<CreateType> {
                            value: CreateType::Request,
                            index: 2usize,
                            on_select: move |create_type: CreateType| {
                                state.create_modal_type.set(Some(create_type));
                            },
                            "New Request"
                        }
                        DropdownMenuItem::<CreateType> {
                            value: CreateType::Environment,
                            index: 3usize,
                            on_select: move |create_type: CreateType| {
                                state.create_modal_type.set(Some(create_type));
                            },
                            "New Environment"
                        }
                    }
                }
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
