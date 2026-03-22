#![allow(clippy::volatile_composites)]

use dioxus::prelude::*;

use kinetic_ui::KineticTheme;
use state::{Collection, KeyValue, Request, Space};
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
    // Initialize state signals with seed data
    let spaces = use_signal(|| {
        vec![Space {
            id: 1,
            name: "Default".into(),
            icon: None,
            color: None,
            environments: vec![],
            variables: vec![],
        }]
    });
    let collections = use_signal(|| {
        vec![
            Collection {
                id: 1,
                space_id: 1,
                name: "User Auth API".into(),
                icon: None,
                color: None,
            },
            Collection {
                id: 2,
                space_id: 1,
                name: "Billing Service".into(),
                icon: None,
                color: None,
            },
        ]
    });
    let requests = use_signal(|| {
        vec![
            Request {
                id: 1,
                collection_id: Some(1),
                name: "Login User".into(),
                method: "POST".into(),
                url: "https://api.example.com/v1/auth/login".into(),
                headers: vec![],
                params: vec![],
                body: None,
                inherit_cookies_header: false,
                inherit_authorization_header: false,
            },
            Request {
                id: 2,
                collection_id: Some(1),
                name: "Get User Profile".into(),
                method: "GET".into(),
                url: "https://api.example.com/v1/users/me".into(),
                headers: vec![],
                params: vec![KeyValue {
                    id: 1,
                    key: "api_version".into(),
                    value: "2024-09-01".into(),
                    description: "API version".into(),
                    enabled: true,
                }],
                body: None,
                inherit_cookies_header: false,
                inherit_authorization_header: false,
            },
            Request {
                id: 3,
                collection_id: Some(1),
                name: "Reset Password".into(),
                method: "PUT".into(),
                url: "https://api.example.com/v1/auth/reset".into(),
                headers: vec![],
                params: vec![],
                body: None,
                inherit_cookies_header: false,
                inherit_authorization_header: false,
            },
            Request {
                id: 4,
                collection_id: Some(2),
                name: "List Invoices".into(),
                method: "GET".into(),
                url: "https://api.example.com/v1/billing/invoices".into(),
                headers: vec![],
                params: vec![],
                body: None,
                inherit_cookies_header: false,
                inherit_authorization_header: false,
            },
            Request {
                id: 5,
                collection_id: Some(2),
                name: "Delete Invoice".into(),
                method: "DELETE".into(),
                url: "https://api.example.com/v1/billing/invoices/123".into(),
                headers: vec![],
                params: vec![],
                body: None,
                inherit_cookies_header: false,
                inherit_authorization_header: false,
            },
        ]
    });
    let next_space_id = use_signal(|| 2);
    let next_collection_id = use_signal(|| 3);
    let next_request_id = use_signal(|| 6);
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
