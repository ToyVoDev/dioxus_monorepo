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

#[component]
fn DesktopLayout() -> Element {
    use_player_state_provider();
    rsx! {
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
