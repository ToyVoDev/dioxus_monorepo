use dioxus::prelude::*;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, Sidebar};
use views::{AlbumDetail, Artists, Downloads, Home, Library, Playlists};

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
        #[route("/downloads")]
        Downloads {},
        #[route("/home")]
        Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use axum::Extension;
        use dioxus::server::axum::routing::get;

        dotenvy::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or environment");

        // Run migrations synchronously on a blocking thread
        {
            let url = database_url.clone();
            tokio::task::spawn_blocking(move || dioxus_music_api::db::run_migrations(&url))
                .await
                .expect("Migration thread panicked");
        }

        let pool = dioxus_music_api::db::create_pool(&database_url).await;

        // Spawn background quick scan
        tokio::spawn(dioxus_music_api::scanner::quick_scan(pool.clone()));

        let router = dioxus::server::router(App)
            .route(
                "/stream/{track_id}",
                get(dioxus_music_api::streaming::stream_track),
            )
            .layer(Extension(pool));

        Ok(router)
    });

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::Config;
        use dioxus::desktop::tao::window::WindowBuilder;

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

    #[cfg(not(any(feature = "desktop", feature = "server")))]
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
