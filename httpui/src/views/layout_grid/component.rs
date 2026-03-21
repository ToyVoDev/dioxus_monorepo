use crate::views::canvas::Canvas;
use crate::views::explorer::Explorer;
use crate::views::sidenav::SideNav;
use crate::views::topbar::TopBar;
use dioxus::prelude::*;

#[component]
pub fn LayoutGrid(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { id: "layout-grid",
            SideNav {}
            Explorer {}
            TopBar {}
            div { style: "grid-area: canvas;",
                Canvas {}
            }
            div { style: "grid-area: statusbar; background: var(--k-surface-low); min-height: 24px;",
            }
        }
    }
}
