use dioxus::prelude::*;

const SIDEBAR_CSS: Asset = asset!("/assets/styling/sidebar.css");

#[component]
pub fn Sidebar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SIDEBAR_CSS }
        nav { class: "sidebar",
            div { class: "sidebar__nav",
                {children}
            }
            div { class: "sidebar__footer",
                span { "{env!(\"CARGO_PKG_NAME\")} v{env!(\"CARGO_PKG_VERSION\")}" }
            }
        }
    }
}
