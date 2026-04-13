use dioxus::prelude::*;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, ServerConfig, Sidebar};
use uuid::Uuid;
use views::{
    AlbumDetail, Artists, Downloads, Library, NowPlaying, PlaylistSidebarSection, PlaylistView,
    Playlists,
};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
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
        #[route("/now-playing")]
        NowPlaying {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        dioxus_logger::initialize_default();
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        let music_dir = std::env::var("MUSIC_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::audio_dir().unwrap_or_else(|| {
                    dirs::home_dir().expect("home dir must exist").join("Music")
                })
            });

        let image_cache_dir = std::env::var("IMAGE_CACHE_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::data_local_dir()
                    .unwrap_or_else(|| {
                        dirs::home_dir().expect("home dir must exist").join(".local/share")
                    })
                    .join("dioxus_music/images")
            });

        // Run Diesel migrations (blocking, must complete before serving).
        {
            let url = database_url.clone();
            tokio::task::spawn_blocking(move || dioxus_music_api::run_migrations(&url))
                .await
                .expect("Migration thread panicked");
        }

        let pool = dioxus_music_api::create_pool(&database_url).await;

        let server_id: Uuid = std::env::var("SERVER_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(Uuid::new_v4);

        let state = dioxus_music_api::AppState {
            pool,
            image_cache_dir,
            server_id,
            music_dir,
        };

        // Create default admin user if no users exist.
        dioxus_music_api::bootstrap(&state).await;

        // Mount the Jellyfin REST router alongside the Dioxus app.
        let api_router = dioxus_music_api::create_router(state);

        let router = dioxus::server::router(App).merge(api_router);

        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn AppLayout() -> Element {
    use_context_provider(|| ServerConfig { base_url: String::new() });
    use_player_state_provider();
    let nav = navigator();
    let current_route = use_route::<Route>();
    let is_now_playing = matches!(current_route, Route::NowPlaying {});

    rsx! {
        AppShell {
            player_bar_hidden: is_now_playing,
            on_player_expand: move |_| { nav.push(Route::NowPlaying {}); },
            sidebar: rsx! {
                Sidebar {
                    Link { class: "sidebar__nav-item", to: Route::Artists {}, "Artists" }
                    Link { class: "sidebar__nav-item", to: Route::Library {}, "Albums" }
                    Link { class: "sidebar__nav-item", to: Route::Playlists {}, "Playlists" }
                    Link { class: "sidebar__nav-item", to: Route::Downloads {}, "Downloads" }
                    PlaylistSidebarSection {}
                }
            },
            Outlet::<Route> {}
        }
    }
}
