use crate::state::models::{Collection, EditorTab, Request, SideNavItem, Space, TopBarNav};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentState {
    pub spaces: Vec<Space>,
    pub collections: Vec<Collection>,
    pub requests: Vec<Request>,
    pub next_space_id: i32,
    pub next_collection_id: i32,
    pub next_request_id: i32,
    pub open_requests: Vec<i32>,
    pub selected_request: Option<i32>,
    pub active_sidebar_nav: SideNavItem,
    pub active_topbar_nav: TopBarNav,
    pub active_editor_tab: EditorTab,
    pub create_modal_type: Option<crate::state::models::CreateType>,
    pub selected_space: Option<i32>,
    pub selected_collection: Option<i32>,
    pub next_environment_id: i32,
    pub environments: Vec<crate::state::models::Environment>,
}

impl Default for PersistentState {
    fn default() -> Self {
        Self {
            spaces: vec![],
            collections: vec![],
            requests: vec![],
            next_space_id: 1,
            next_collection_id: 1,
            next_request_id: 1,
            open_requests: Vec::new(),
            selected_request: None,
            active_sidebar_nav: SideNavItem::Collections,
            active_topbar_nav: TopBarNav::Collections,
            active_editor_tab: EditorTab::Params,
            create_modal_type: None,
            selected_space: None,
            selected_collection: None,
            next_environment_id: 1,
            environments: Vec::new(),
        }
    }
}

pub fn get_storage_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("httpui");
        path.push("state.json");
        path
    })
}

pub fn save_state(state: &PersistentState) {
    if let Some(path) = get_storage_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(state) {
            let _ = fs::write(path, json);
        }
    }
}

pub fn load_state() -> PersistentState {
    if let Some(path) = get_storage_path() {
        tracing::info!("Loading state from {}", path.display());
        match fs::read_to_string(&path) {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(state) => {
                    tracing::info!("Successfully restored state from {}", path.display());
                    return state;
                }
                Err(e) => {
                    tracing::warn!("Failed to parse state from {}: {}", path.display(), e);
                }
            },
            Err(e) => {
                tracing::info!(
                    "No existing state file at {} ({}), using defaults",
                    path.display(),
                    e
                );
            }
        }
    }
    PersistentState::default()
}
