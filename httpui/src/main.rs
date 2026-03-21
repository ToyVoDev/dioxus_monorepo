// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use kinetic_ui::KineticTheme;
use views::{
    collection::CollectionSection,
    layout_grid::LayoutGrid,
    request::{NewRequestSection, RequestSection},
    settings::SettingsSection,
    space::SpaceSection,
};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a state module that contains all state management for our app.
mod state;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/request/:id")]
    RequestSection { id: i32 },
    #[route("/space/:id")]
    SpaceSection { id: i32 },
    #[route("/collection/:id")]
    CollectionSection { id: i32 },
    #[route("/settings")]
    SettingsSection {},
    #[route("/")]
    NewRequestSection {},
}

#[component]
fn HomeSection() -> Element {
    rsx! {
        div {
            id: "home",
            "Home"
        }
    }
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
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Initialize state signals
    let spaces = use_signal(|| Vec::new());
    let collections = use_signal(|| Vec::new());
    let requests = use_signal(|| Vec::new());
    let next_space_id = use_signal(|| 1);
    let next_collection_id = use_signal(|| 1);
    let next_request_id = use_signal(|| 1);
    let open_requests = use_signal(|| Vec::new());
    let selected_request = use_signal(|| None);
    let response = use_signal(|| String::new());
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
        response,
        active_sidebar_nav,
        active_topbar_nav,
        active_editor_tab,
        http_response,
    );

    // Provide the state to all child components via context
    use_context_provider(|| app_state);

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        // We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
        // The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        KineticTheme {
            LayoutGrid {
                Router::<Route> {}
            }
        }
    }
}
