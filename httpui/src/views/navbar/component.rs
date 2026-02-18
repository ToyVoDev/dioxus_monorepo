use crate::Route;
use crate::components::button::{ButtonVariant, LinkButton};
use crate::views::library::Library;
use crate::views::tabbar::Tabbar;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::vsc_icons::VscHome, icons::vsc_icons::VscSettingsGear};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        if cfg!(all(feature = "desktop", target_os = "macos")) {
            div {
                // enough space to fit the macos traffic lights
                style: "position: absolute; top: 0; left: 0; right: 0; height: 31px; z-index: 100; cursor: default;",
                onmousedown: move |_| {
                    #[cfg(feature = "desktop")]
                    dioxus::desktop::window().drag()
                },
                ondoubleclick: move |_| {
                    #[cfg(feature = "desktop")]
                    dioxus::desktop::window().toggle_maximized();
                },
            }
            div {
                // enough space to fit the macos traffic lights
                style: "position: absolute; top: 0; left: 0; bottom: 0; width: 74px; z-index: 100; cursor: default;",
                onmousedown: move |_| {
                    #[cfg(feature = "desktop")]
                    dioxus::desktop::window().drag()
                },
                ondoubleclick: move |_| {
                    #[cfg(feature = "desktop")]
                    dioxus::desktop::window().toggle_maximized();
                },
            }
        }
        div {
            id: "navbar",
            // enough space to fit the macos traffic lights
            style: if cfg!(all(feature = "desktop", target_os = "macos")) { "min-width: 74px; padding-top: 31px; cursor: default;" },
            LinkButton {
                variant: ButtonVariant::Ghost,
                draggable: false,
                style: "z-index: 101;",
                to: Route::RequestSection { id: 0 },
                Icon {
                    width: 30,
                    height: 30,
                    fill: "inherit",
                    icon: VscHome,
                }
            }
            LinkButton {
                variant: ButtonVariant::Ghost,
                draggable: false,
                style: "z-index: 101; margin-top: auto; margin-bottom: 8px;",
                to: Route::SettingsSection { },
                Icon {
                    width: 30,
                    height: 30,
                    fill: "inherit",
                    icon: VscSettingsGear,
                }
            }
        }
        Library {}
        Tabbar {}

        Outlet::<Route> {}
    }
}
