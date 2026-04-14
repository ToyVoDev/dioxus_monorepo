use dioxus::prelude::*;
use dioxus_music_ui::api_client::ApiClient;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, ServerConfig, Sidebar};
use uuid::Uuid;
use views::{AlbumDetail, Artists, Downloads, Home, Library, NowPlaying, PlaylistView, Playlists};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopLayout)]
        #[route("/")]
        Library {},
        #[route("/album/:name")]
        AlbumDetail { name: String },
        #[route("/artists")]
        Artists {},
        #[route("/playlists")]
        Playlists {},
        #[route("/playlist/:id")]
        PlaylistView { id: Uuid },
        #[route("/downloads")]
        Downloads {},
        #[route("/home")]
        Home {},
        #[route("/now-playing")]
        NowPlaying {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[cfg(feature = "desktop")]
fn main() {
    dioxus_logger::initialize_default();
    
    use dioxus::desktop::Config;
    use dioxus::desktop::tao::window::WindowBuilder;

    // Point server function calls at the remote web server
    let server_url: &'static str = Box::leak(
        std::env::var("SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string())
            .into_boxed_str(),
    );
    dioxus::fullstack::set_server_url(server_url);

    let mut wb = WindowBuilder::new().with_title(env!("CARGO_PKG_NAME"));

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
    let server_url =
        std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    use_context_provider(|| ApiClient::new(server_url.clone()));
    use_context_provider(|| ServerConfig {
        base_url: server_url,
    });
    use_player_state_provider();
    let route = use_route::<Route>();
    let on_now_playing = matches!(route, Route::NowPlaying {});
    let nav = navigator();
    rsx! {
        {drag_regions()}
        AppShell {
            player_bar_hidden: on_now_playing,
            on_player_expand: move |_| { nav.push(Route::NowPlaying {}); },
            sidebar: rsx! {
                Sidebar {
                    Link { class: "sidebar__nav-item", to: Route::Artists {}, "Artists" }
                    Link { class: "sidebar__nav-item", to: Route::Library {}, "Albums" }
                    Link { class: "sidebar__nav-item", to: Route::Playlists {}, "Playlists" }
                    Link { class: "sidebar__nav-item", to: Route::Downloads {}, "Downloads" }
                }
            },
            Outlet::<Route> {}
        }
    }
}
