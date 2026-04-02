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
    pub create_modal_type: Signal<Option<crate::state::models::CreateType>>,
    pub selected_space: Signal<Option<i32>>,
    pub selected_collection: Signal<Option<i32>>,
    pub next_environment_id: Signal<i32>,
    pub environments: Signal<Vec<crate::state::models::Environment>>,
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
        create_modal_type: Signal<Option<crate::state::models::CreateType>>,
        selected_space: Signal<Option<i32>>,
        selected_collection: Signal<Option<i32>>,
        next_environment_id: Signal<i32>,
        environments: Signal<Vec<crate::state::models::Environment>>,
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
            create_modal_type,
            selected_space,
            selected_collection,
            next_environment_id,
            environments,
        }
    }

    pub fn create_space(&mut self, name: String, icon: String, color: String) -> i32 {
        let id = *self.next_space_id.read();
        *self.next_space_id.write() += 1;
        let space = Space {
            id,
            name,
            icon: Some(icon),
            color: Some(color),
            environments: Vec::new(),
            variables: Vec::new(),
        };
        self.spaces.write().push(space);
        id
    }

    pub fn create_collection(
        &mut self,
        name: String,
        icon: String,
        color: String,
        space_id: i32,
    ) -> i32 {
        let id = *self.next_collection_id.read();
        *self.next_collection_id.write() += 1;
        let collection = Collection {
            id,
            space_id,
            name,
            icon: Some(icon),
            color: Some(color),
        };
        self.collections.write().push(collection);
        id
    }

    pub fn create_request(
        &mut self,
        name: Option<String>,
        method: String,
        url: String,
        collection_id: i32,
    ) -> i32 {
        let id = *self.next_request_id.read();
        *self.next_request_id.write() += 1;
        let request = Request {
            id,
            collection_id: Some(collection_id),
            name: name.unwrap_or_else(|| format!("{} {}", method, url)),
            method,
            url,
            headers: Vec::new(),
            params: Vec::new(),
            body: None,
            inherit_cookies_header: false,
            inherit_authorization_header: false,
        };
        self.requests.write().push(request);
        id
    }

    pub fn create_environment(&mut self, name: String, space_id: i32) -> i32 {
        let id = *self.next_environment_id.read();
        *self.next_environment_id.write() += 1;
        let environment = crate::state::models::Environment {
            id,
            name,
            variables: Vec::new(),
        };
        // Add to the space's environments list
        if let Some(space) = self.spaces.write().iter_mut().find(|s| s.id == space_id) {
            space.environments.push(environment.clone());
        }
        id
    }
}
