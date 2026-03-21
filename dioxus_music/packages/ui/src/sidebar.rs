use dioxus::prelude::*;

const SIDEBAR_CSS: Asset = asset!("/assets/styling/sidebar.css");

#[component]
pub fn Sidebar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SIDEBAR_CSS }
        nav { class: "sidebar",
            div { class: "sidebar__brand",
                span { class: "sidebar__brand-name", {env!("CARGO_PKG_NAME")} }
                span { class: "sidebar__subtitle", "v{env!(\"CARGO_PKG_VERSION\")}" }
            }
            div { class: "sidebar__nav",
                {children}
            }
            div { class: "sidebar__footer",
                span { "Core Engine v2.4" }
            }
        }
    }
}
