use crate::state::models::*;
use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct AppState {
    pub spaces: Signal<Vec<Space>>,
    pub collections: Signal<Vec<Collection>>,
    pub requests: Signal<Vec<Request>>,
    pub next_space_id: Signal<i32>,
    pub next_collection_id: Signal<i32>,
    pub next_request_id: Signal<i32>,
    pub open_requests: Signal<Vec<i32>>,
    pub selected_request: Signal<Option<i32>>,
    pub response: Signal<String>,
}

impl AppState {
    pub fn new(
        spaces: Signal<Vec<Space>>,
        collections: Signal<Vec<Collection>>,
        requests: Signal<Vec<Request>>,
        next_space_id: Signal<i32>,
        next_collection_id: Signal<i32>,
        next_request_id: Signal<i32>,
        open_requests: Signal<Vec<i32>>,
        selected_request: Signal<Option<i32>>,
        response: Signal<String>,
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
            response,
        }
    }

    pub fn get_space(&self, id: i32) -> Option<Space> {
        self.spaces.read().iter().find(|s| s.id == id).cloned()
    }

    pub fn get_collection(&self, id: i32) -> Option<Collection> {
        self.collections.read().iter().find(|c| c.id == id).cloned()
    }

    pub fn get_request(&self, id: i32) -> Option<Request> {
        self.requests.read().iter().find(|r| r.id == id).cloned()
    }

    pub fn get_collections_for_space(&self, space_id: i32) -> Vec<Collection> {
        self.collections
            .read()
            .iter()
            .filter(|c| c.space_id == space_id)
            .cloned()
            .collect()
    }

    pub fn get_requests_for_collection(&self, collection_id: i32) -> Vec<Request> {
        self.requests
            .read()
            .iter()
            .filter(|r| r.collection_id == Some(collection_id))
            .cloned()
            .collect()
    }
}
