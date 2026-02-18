use crate::views::{
    request_editor::RequestEditor, response_viewer::ResponseViewer, urlbar::Urlbar,
};
use dioxus::prelude::*;

#[component]
pub fn RequestSection(id: i32) -> Element {
    rsx! {
        Urlbar {}
        RequestEditor {}
        ResponseViewer {}
    }
}

#[component]
pub fn NewRequestSection() -> Element {
    rsx! {
        Urlbar {}
        RequestEditor {}
        ResponseViewer {}
    }
}
