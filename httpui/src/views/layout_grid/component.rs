use crate::views::sidenav::SideNav;
use dioxus::prelude::*;

#[component]
pub fn LayoutGrid(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { id: "layout-grid",
            SideNav {}
            div { style: "grid-area: explorer; background: var(--k-surface-high);",
                "Explorer placeholder"
            }
            div { style: "grid-area: topbar; background: var(--k-surface-low);",
                "TopBar placeholder"
            }
            div { style: "grid-area: canvas;",
                {children}
            }
            div { style: "grid-area: statusbar; background: var(--k-surface-low); min-height: 24px;",
            }
        }
    }
}
