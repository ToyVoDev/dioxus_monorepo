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
        div {
            id: "layout-drag-top",
            onmousedown: move |_| {
                dioxus::desktop::window().drag()
            },
            ondoubleclick: move |_| {
                dioxus::desktop::window().toggle_maximized();
            },
        }
        div {
            id: "layout-drag-side",
            onmousedown: move |_| {
                dioxus::desktop::window().drag()
            },
            ondoubleclick: move |_| {
                dioxus::desktop::window().toggle_maximized();
            },
        }
        div { id: "layout-grid",
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
