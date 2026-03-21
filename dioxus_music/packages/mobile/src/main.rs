use dioxus::prelude::*;
use dioxus_music_ui::PlayerBar;
use kinetic_ui::KineticTheme;
use views::mobile_nav::MobileNav;
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
        KineticTheme {
            main {
                style: "padding-bottom: 128px; min-height: 100vh; background: var(--k-surface);",
                Outlet::<Route> {}
            }
            PlayerBar { compact: true }
            MobileNav {}
        }
    }
}
