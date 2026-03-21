use dioxus::prelude::*;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, Sidebar};
use views::Home;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopLayout)]
        #[route("/")]
        Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::Config;
    use dioxus::desktop::tao::window::WindowBuilder;

    let mut wb = WindowBuilder::new().with_title("Kinetic Music");

    #[cfg(target_os = "macos")]
    {
        use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;
        wb = wb
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true);
    }

    let config = Config::new().with_window(wb);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[cfg(not(feature = "desktop"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[cfg(feature = "desktop")]
fn drag_regions() -> Element {
    rsx! {
        div {
            class: "drag-region-top",
            onmousedown: move |_| { dioxus::desktop::window().drag(); },
            ondoubleclick: move |_| { dioxus::desktop::window().toggle_maximized(); },
        }
        div {
            class: "drag-region-side",
            onmousedown: move |_| { dioxus::desktop::window().drag(); },
            ondoubleclick: move |_| { dioxus::desktop::window().toggle_maximized(); },
        }
    }
}

#[cfg(not(feature = "desktop"))]
fn drag_regions() -> Element {
    rsx! {}
}

#[component]
fn DesktopLayout() -> Element {
    use_player_state_provider();
    rsx! {
        // Drag regions for macOS window dragging
        {drag_regions()}
        AppShell {
            sidebar: rsx! {
                Sidebar {
                    Link { class: "sidebar__nav-item", to: Route::Home {}, "Home" }
                }
            },
            Outlet::<Route> {}
        }
    }
}
