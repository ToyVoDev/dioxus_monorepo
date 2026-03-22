use dioxus::prelude::*;

const MOBILE_NAV_CSS: Asset = asset!("/assets/mobile-nav.css");

#[component]
pub fn MobileNav() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MOBILE_NAV_CSS }
        nav { class: "mobile-nav",
            // Home
            button { class: "mobile-nav__item", "data-active": "true",
                svg { class: "mobile-nav__icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                }
                "Home"
            }
            // Search
            button { class: "mobile-nav__item",
                svg { class: "mobile-nav__icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    circle { cx: "11", cy: "11", r: "8" }
                    line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                }
                "Search"
            }
            // Library
            button { class: "mobile-nav__item",
                svg { class: "mobile-nav__icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    path { d: "M9 18V5l12-2v13" }
                    circle { cx: "6", cy: "18", r: "3" }
                    circle { cx: "18", cy: "16", r: "3" }
                }
                "Library"
            }
            // Downloads
            button { class: "mobile-nav__item",
                svg { class: "mobile-nav__icon", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                    polyline { points: "7 10 12 15 17 10" }
                    line { x1: "12", y1: "15", x2: "12", y2: "3" }
                }
                "Downloads"
            }
        }
    }
}
