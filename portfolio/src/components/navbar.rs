use crate::components::UserHeader;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "d-flex",
            div {
                class: "sidebar",
                Link {
                    to: Route::Me {},
                    active_class: "active",
                    "Me"
                }
                Link {
                    to: Route::Work { },
                    active_class: "active",
                    "Work"
                }
                Link {
                    to: Route::Projects { },
                    active_class: "active",
                    "Projects"
                }
                Link {
                    to: Route::Education { },
                    active_class: "active",
                    "Education"
                }
            }
            div {
                class: "p2 flex-grow-1",
                UserHeader { }
                Outlet::<Route> {}
            }
        }
    }
}
