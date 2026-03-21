#![allow(clippy::volatile_composites)]

use dioxus::prelude::*;

use kinetic_ui::KineticTheme;
use views::{layout_grid::LayoutGrid, settings::SettingsSection};

/// Define a state module that contains all state management for our app.
mod state;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/settings")]
    SettingsSection {},
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    rsx! { {} }
}

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::Config;
    use dioxus::desktop::tao::window::WindowBuilder;

    let mut wb = WindowBuilder::new();

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
    // Initialize state signals
    let spaces = use_signal(Vec::new);
    let collections = use_signal(Vec::new);
    let requests = use_signal(Vec::new);
    let next_space_id = use_signal(|| 1);
    let next_collection_id = use_signal(|| 1);
    let next_request_id = use_signal(|| 1);
    let open_requests = use_signal(Vec::new);
    let selected_request = use_signal(|| None);
    let active_sidebar_nav = use_signal(|| state::SideNavItem::Collections);
    let active_topbar_nav = use_signal(|| state::TopBarNav::Collections);
    let active_editor_tab = use_signal(|| state::EditorTab::Params);
    let http_response = use_signal(|| None);

    // Create the app state
    let app_state = state::AppState::new(
        spaces,
        collections,
        requests,
        next_space_id,
        next_collection_id,
        next_request_id,
        open_requests,
        selected_request,
        active_sidebar_nav,
        active_topbar_nav,
        active_editor_tab,
        http_response,
    );

    // Provide the state to all child components via context
    use_context_provider(|| app_state);

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        KineticTheme {
            LayoutGrid {
                Router::<Route> {}
            }
        }
    }
}
