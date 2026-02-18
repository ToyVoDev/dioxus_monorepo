use dioxus::prelude::*;
use dioxus_music_ui::{AppShell, Sidebar};
use views::Home;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MobileLayout)]
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
fn MobileLayout() -> Element {
    rsx! {
        AppShell {
            sidebar: rsx! {
                Sidebar {
                    Link { to: Route::Home {}, "Home" }
                }
            },
            Outlet::<Route> {}
        }
    }
}
