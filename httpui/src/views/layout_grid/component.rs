use crate::views::canvas::Canvas;
use crate::views::explorer::Explorer;
use crate::views::sidenav::SideNav;
use crate::views::statusbar::StatusBar;
use crate::views::topbar::TopBar;
use dioxus::prelude::*;

#[component]
pub fn LayoutGrid(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { id: "layout-grid",
            // Invisible drag region for macOS window dragging
            div { id: "layout-drag-region" }
            SideNav {}
            Explorer {}
            TopBar {}
            div { style: "grid-area: canvas;",
                Canvas {}
            }
            StatusBar {}
        }
    }
}
