use dioxus::prelude::*;
use dioxus_music_ui::player_state::use_player_state_provider;
use dioxus_music_ui::{AppShell, Sidebar};
use uuid::Uuid;
use views::{Library, PlaylistSidebarSection, PlaylistView};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Library {},
        #[route("/playlist/:id")]
        PlaylistView { id: Uuid },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
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

        // Spawn background quick scan (non-blocking, skips files already in DB)
        tokio::spawn(dioxus_music_api::scanner::quick_scan(pool.clone()));

        let router = dioxus::server::router(App)
            .route(
                "/stream/{track_id}",
                get(dioxus_music_api::streaming::stream_track),
            )
            .layer(Extension(pool));

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
    use_player_state_provider();

    rsx! {
        AppShell {
            sidebar: rsx! {
                Sidebar {
                    Link { to: Route::Library {}, "All Songs" }
                    PlaylistSidebarSection {}
                }
            },
            Outlet::<Route> {}
        }
    }
}
