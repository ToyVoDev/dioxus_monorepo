#![allow(clippy::volatile_composites)]

use dioxus::prelude::*;

use kinetic_ui::KineticTheme;
use views::{
    layout_grid::LayoutGrid,
    modals::CreateModal,
    settings::{CollectionSettings, EnvironmentSettings, SettingsSection},
};

/// Define a state module that contains all state management for our app.
mod state;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/settings/collection/:id")]
    CollectionSettings { id: i32 },
    #[route("/settings/environment/:id")]
    EnvironmentSettings { id: i32 },
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
    dioxus_logger::initialize_default();

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
    dioxus_logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let initial_state = use_signal(state::load_state);

    // Initialize state signals with data from persistence
    let spaces = use_signal(|| initial_state.read().spaces.clone());
    let collections = use_signal(|| initial_state.read().collections.clone());
    let requests = use_signal(|| initial_state.read().requests.clone());
    let next_space_id = use_signal(|| initial_state.read().next_space_id);
    let next_collection_id = use_signal(|| initial_state.read().next_collection_id);
    let next_request_id = use_signal(|| initial_state.read().next_request_id);
    let open_requests = use_signal(|| initial_state.read().open_requests.clone());
    let selected_request = use_signal(|| initial_state.read().selected_request);
    let active_sidebar_nav = use_signal(|| initial_state.read().active_sidebar_nav);
    let active_topbar_nav = use_signal(|| initial_state.read().active_topbar_nav);
    let active_editor_tab = use_signal(|| initial_state.read().active_editor_tab);
    let http_response = use_signal(|| None);
    let create_modal_type = use_signal(|| initial_state.read().create_modal_type);
    let selected_space = use_signal(|| initial_state.read().selected_space);
    let selected_collection = use_signal(|| initial_state.read().selected_collection);
    let next_environment_id = use_signal(|| initial_state.read().next_environment_id);
    let environments = use_signal(|| initial_state.read().environments.clone());

    // Save state whenever relevant signals change
    use_effect(move || {
        let state = state::PersistentState {
            spaces: spaces.read().clone(),
            collections: collections.read().clone(),
            requests: requests.read().clone(),
            next_space_id: *next_space_id.read(),
            next_collection_id: *next_collection_id.read(),
            next_request_id: *next_request_id.read(),
            open_requests: open_requests.read().clone(),
            selected_request: *selected_request.read(),
            active_sidebar_nav: *active_sidebar_nav.read(),
            active_topbar_nav: *active_topbar_nav.read(),
            active_editor_tab: *active_editor_tab.read(),
            create_modal_type: *create_modal_type.read(),
            selected_space: *selected_space.read(),
            selected_collection: *selected_collection.read(),
            next_environment_id: *next_environment_id.read(),
            environments: environments.read().clone(),
        };
        state::save_state(&state);
    });

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
        create_modal_type,
        selected_space,
        selected_collection,
        next_environment_id,
        environments,
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
        CreateModal {}
    }
}
