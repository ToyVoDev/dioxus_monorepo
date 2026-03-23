use crate::Route;
use crate::components::UserHeader;
use dioxus::prelude::*;
const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let route: Route = use_route();
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { class: "layout",
            nav { class: "sidebar",
                div { class: "sidebar__nav",
                    NavLink { to: Route::Me {}, current: route.clone(), "Me" }
                    NavLink { to: Route::Work {}, current: route.clone(), "Work" }
                    NavLink { to: Route::Projects {}, current: route.clone(), "Projects" }
                    NavLink { to: Route::Education {}, current: route.clone(), "Education" }
                }
            }
            div {
                class: "content",
                UserHeader { }
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn NavLink(to: Route, current: Route, children: Element) -> Element {
    let is_active = current == to;
    rsx! {
        Link {
            to: to,
            class: if is_active { "nav-link nav-link--active" } else { "nav-link" },
            {children}
        }
    }
}
