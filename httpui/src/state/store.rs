use crate::state::models::{
    Collection, EditorTab, HttpResponse, Request, SideNavItem, Space, TopBarNav,
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct AppState {
    pub spaces: Signal<Vec<Space>>,
    pub collections: Signal<Vec<Collection>>,
    pub requests: Signal<Vec<Request>>,
    pub next_space_id: Signal<i32>,
    pub next_collection_id: Signal<i32>,
    pub next_request_id: Signal<i32>,
    pub open_requests: Signal<Vec<i32>>,
    pub selected_request: Signal<Option<i32>>,
    pub active_sidebar_nav: Signal<SideNavItem>,
    pub active_topbar_nav: Signal<TopBarNav>,
    pub active_editor_tab: Signal<EditorTab>,
    pub http_response: Signal<Option<HttpResponse>>,
}

impl AppState {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        spaces: Signal<Vec<Space>>,
        collections: Signal<Vec<Collection>>,
        requests: Signal<Vec<Request>>,
        next_space_id: Signal<i32>,
        next_collection_id: Signal<i32>,
        next_request_id: Signal<i32>,
        open_requests: Signal<Vec<i32>>,
        selected_request: Signal<Option<i32>>,
        active_sidebar_nav: Signal<SideNavItem>,
        active_topbar_nav: Signal<TopBarNav>,
        active_editor_tab: Signal<EditorTab>,
        http_response: Signal<Option<HttpResponse>>,
    ) -> Self {
        Self {
            spaces,
            collections,
            requests,
            next_space_id,
            next_collection_id,
            next_request_id,
            open_requests,
            selected_request,
            active_sidebar_nav,
            active_topbar_nav,
            active_editor_tab,
            http_response,
        }
    }
}
