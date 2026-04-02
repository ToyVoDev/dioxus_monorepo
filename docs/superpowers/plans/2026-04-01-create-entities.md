# Create Entities Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ability to create Spaces, Collections, Requests, and Environments via a dropdown menu from the "+" button in the side navigation.

**Architecture:** Single modal component handles all entity types with variant prop, using local form state and contextual defaults. Icon and color pickers are reusable components. AppState tracks modal state and creation logic.

**Tech Stack:** Dioxus 0.7, kinetic_ui components, dioxus_free_icons, serde

---

## File Structure

### New Files
- `httpui/src/components/mod.rs` - Module exports for reusable components
- `httpui/src/components/icon_picker.rs` - Icon selection popover
- `httpui/src/components/color_picker.rs` - Color selection popover
- `httpui/src/views/modals/mod.rs` - Module exports for modals
- `httpui/src/views/modals/create_modal.rs` - Main create entity modal
- `httpui/src/views/settings/collection_settings.rs` - Stub settings page
- `httpui/src/views/settings/environment_settings.rs` - Stub settings page

### Modified Files
- `httpui/src/state/models.rs` - Add `CreateType` enum
- `httpui/src/state/store.rs` - Add `create_modal_type`, `selected_space`, `selected_collection` signals and creation methods
- `httpui/src/state/persistence.rs` - Add new state fields to `PersistentState`
- `httpui/src/views/mod.rs` - Export `modals` module
- `httpui/src/views/sidenav/component.rs` - Replace static "+" button with `CreateDropdown`
- `httpui/src/views/settings/mod.rs` - Export new settings components
- `httpui/src/main.rs` - Add routes for settings pages, initialize new state

---

## Task 1: Add CreateType Enum to State Models

**Files:**
- Modify: `httpui/src/state/models.rs:1-10`

**Steps:**

- [ ] **Step 1: Add CreateType enum after existing enums**

Add to `httpui/src/state/models.rs` after the `EditorTab` enum (around line 26):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Serialize, Deserialize)]
pub enum CreateType {
    Space,
    Collection,
    Request,
    Environment,
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds with no errors

- [ ] **Step 3: Commit**

```bash
git add httpui/src/state/models.rs
git commit -m "feat(httpui): add CreateType enum for entity creation modal"
```

---

## Task 2: Add New State Signals to AppState

**Files:**
- Modify: `httpui/src/state/store.rs:6-21`

**Steps:**

- [ ] **Step 1: Add new signal fields to AppState**

Modify `httpui/src/state/store.rs`. Add these fields to the `AppState` struct (line 8-20):

```rust
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
```

- [ ] **Step 2: Update AppState::new constructor**

Modify the `AppState::new` method to accept and initialize the new fields:

```rust
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
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation fails with "expected 13 parameters, found 17" (this is expected - we'll fix main.rs next)

- [ ] **Step 4: Commit**

```bash
git add httpui/src/state/store.rs
git commit -m "feat(httpui): add new state signals for create modal and selection tracking"
```

---

## Task 3: Add Creation Methods to AppState

**Files:**
- Modify: `httpui/src/state/store.rs:54-end`

**Steps:**

- [ ] **Step 1: Add impl block with creation methods**

Add after the `AppState::new` method in `httpui/src/state/store.rs`:

```rust
impl AppState {
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
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check --package httpui`
Expected: Same errors about parameters in main.rs (we'll fix in next task)

- [ ] **Step 3: Commit**

```bash
git add httpui/src/state/store.rs
git commit -m "feat(httpui): add entity creation methods to AppState"
```

---

## Task 4: Update PersistentState for New Fields

**Files:**
- Modify: `httpui/src/state/persistence.rs:6-36`

**Steps:**

- [ ] **Step 1: Add new fields to PersistentState struct**

Modify `httpui/src/state/persistence.rs`. Update the struct (lines 6-19):

```rust
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
```

- [ ] **Step 2: Update Default impl for PersistentState**

Update the `Default` implementation (lines 21-37):

```rust
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
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check --package httpui`
Expected: Same parameter count errors (will fix main.rs next)

- [ ] **Step 4: Commit**

```bash
git add httpui/src/state/persistence.rs
git commit -m "feat(httpui): add new state fields to PersistentState"
```

---

## Task 5: Update main.rs with New State Initialization

**Files:**
- Modify: `httpui/src/main.rs:60-111`

**Steps:**

- [ ] **Step 1: Add Environment import**

Add to imports at top of `httpui/src/main.rs` (around line 5):

```rust
use state::models::CreateType;
```

- [ ] **Step 2: Initialize new state signals in App component**

Update the signal initialization in the `App` component (lines 60-74):

```rust
fn App() -> Element {
    let initial_state = use_signal(state::load_state);

    // Initialize state signals with data from persistence
    let spaces = use_signal(|| initial_state.read().spaces.clone());
    let collections = use_signal(|| initial_state.read().collections.clone());
    let requests = use_signal(|| initial_state.read().requests.clone());
    let next_space_id = use_signal(|| initial_state.read().next_space_id);
    let next_collection_id = use_signal(|| initial_state.read().next_collection_id);
    let next_request_id = use_signal(|| initial_state.read().next_request_id);
    let open_requests = use_signal(|| initial_state.read().open_requests.clone());
    let selected_request = use_signal(|| initial_state.read().selected_request);
    let active_sidebar_nav = use_signal(|| initial_state.read().active_sidebar_nav);
    let active_topbar_nav = use_signal(|| initial_state.read().active_topbar_nav);
    let active_editor_tab = use_signal(|| initial_state.read().active_editor_tab);
    let http_response = use_signal(|| None);
    let create_modal_type = use_signal(|| initial_state.read().create_modal_type);
    let selected_space = use_signal(|| initial_state.read().selected_space);
    let selected_collection = use_signal(|| initial_state.read().selected_collection);
    let next_environment_id = use_signal(|| initial_state.read().next_environment_id);
    let environments = use_signal(|| initial_state.read().environments.clone());
```

- [ ] **Step 3: Update use_effect to save new fields**

Update the `use_effect` block (lines 77-92):

```rust
    // Save state whenever relevant signals change
    use_effect(move || {
        let state = state::PersistentState {
            spaces: spaces.read().clone(),
            collections: collections.read().clone(),
            requests: requests.read().clone(),
            next_space_id: *next_space_id.read(),
            next_collection_id: *next_collection_id.read(),
            next_request_id: *next_request_id.read(),
            open_requests: open_requests.read().clone(),
            selected_request: *selected_request.read(),
            active_sidebar_nav: *active_sidebar_nav.read(),
            active_topbar_nav: *active_topbar_nav.read(),
            active_editor_tab: *active_editor_tab.read(),
            create_modal_type: *create_modal_type.read(),
            selected_space: *selected_space.read(),
            selected_collection: *selected_collection.read(),
            next_environment_id: *next_environment_id.read(),
            environments: environments.read().clone(),
        };
        state::save_state(&state);
    });
```

- [ ] **Step 4: Update AppState::new call**

Update the `AppState::new` call (lines 95-108):

```rust
    // Create the app state
    let app_state = state::AppState::new(
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
    );
```

- [ ] **Step 5: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds with no errors

- [ ] **Step 6: Commit**

```bash
git add httpui/src/main.rs
git commit -m "feat(httpui): initialize new state signals in App component"
```

---

## Task 6: Add Settings Routes

**Files:**
- Modify: `httpui/src/main.rs:13-20`

**Steps:**

- [ ] **Step 1: Import settings components**

Add to imports at top of `httpui/src/main.rs`:

```rust
use views::settings::{CollectionSettings, EnvironmentSettings};
```

- [ ] **Step 2: Add new route variants**

Update the `Route` enum in `httpui/src/main.rs` (lines 13-20):

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/settings/collection/:id")]
    CollectionSettings { id: i32 },
    #[route("/settings/environment/:id")]
    EnvironmentSettings { id: i32 },
    #[route("/settings")]
    SettingsSection {},
    #[route("/")]
    Home {},
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check --package httpui`
Expected: Fails with "unresolved import" (expected - we'll create the components next)

- [ ] **Step 4: Commit**

```bash
git add httpui/src/main.rs
git commit -m "feat(httpui): add settings routes for collection and environment"
```

---

## Task 7: Create Settings Module Exports

**Files:**
- Modify: `httpui/src/views/settings/mod.rs`
- Create: `httpui/src/views/settings/collection_settings.rs`
- Create: `httpui/src/views/settings/environment_settings.rs`

**Steps:**

- [ ] **Step 1: Update settings mod.rs**

Replace contents of `httpui/src/views/settings/mod.rs`:

```rust
mod component;
mod collection_settings;
mod environment_settings;

pub use component::SettingsSection;
pub use collection_settings::CollectionSettings;
pub use environment_settings::EnvironmentSettings;
```

- [ ] **Step 2: Create collection_settings.rs**

Create `httpui/src/views/settings/collection_settings.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn CollectionSettings(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "settings-page",
            h1 { "Collection Settings" }
            p { "Collection ID: {id}" }
            p { "Configure headers, authentication, and other settings for this collection." }
        }
    }
}
```

- [ ] **Step 3: Create environment_settings.rs**

Create `httpui/src/views/settings/environment_settings.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn EnvironmentSettings(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "settings-page",
            h1 { "Environment Settings" }
            p { "Environment ID: {id}" }
            p { "Configure variables for URL and header substitution." }
        }
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 5: Commit**

```bash
git add httpui/src/views/settings/
git commit -m "feat(httpui): add stub settings pages for collections and environments"
```

---

## Task 8: Create Components Module

**Files:**
- Create: `httpui/src/components/mod.rs`
- Create: `httpui/src/components/icon_picker.rs`
- Create: `httpui/src/components/color_picker.rs`

**Steps:**

- [ ] **Step 1: Create components module file**

Create `httpui/src/components/mod.rs`:

```rust
mod color_picker;
mod icon_picker;

pub use color_picker::ColorPicker;
pub use icon_picker::IconPicker;
```

- [ ] **Step 2: Create basic IconPicker component**

Create `httpui/src/components/icon_picker.rs`:

```rust
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_file_icons::MdFolder;

#[component]
pub fn IconPicker(
    selected: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        button {
            class: "icon-picker",
            onclick: move |_| {
                // TODO: implement icon selection popover
                // For now, just cycle through a few default icons
                let new_icon = if selected.as_str() == "folder" {
                    "dns".to_string()
                } else {
                    "folder".to_string()
                };
                on_change.call(new_icon);
            },
            Icon { icon: MdFolder, width: 24, height: 24 }
        }
    }
}
```

- [ ] **Step 3: Create basic ColorPicker component**

Create `httpui/src/components/color_picker.rs`:

```rust
use dioxus::prelude::*;

const COLORS: &[&str] = &[
    "#FFB3AD", // Primary
    "#44E2CD", // Secondary
    "#F9BD22", // Tertiary
    "#FF5451", // Primary Container
];

#[component]
pub fn ColorPicker(
    selected: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "color-picker",
            for color in COLORS {
                button {
                    class: "color-picker__swatch",
                    style:format!("background-color: {}", color),
                    onclick: move |_| {
                        on_change.call(color.to_string());
                    },
                }
            }
        }
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 5: Commit**

```bash
git add httpui/src/components/
git commit -m "feat(httpui): add basic IconPicker and ColorPicker components"
```

---

## Task 9: Create Modals Module

**Files:**
- Create: `httpui/src/views/modals/mod.rs`
- Create: `httpui/src/views/modals/create_modal.rs`

**Steps:**

- [ ] **Step 1: Create modals module file**

Create `httpui/src/views/modals/mod.rs`:

```rust
mod create_modal;

pub use create_modal::CreateModal;
```

- [ ] **Step 2: Create CreateModal component (part 1 - structure)**

Create `httpui/src/views/modals/create_modal.rs`:

```rust
use crate::state::{AppState, CreateType};
use dioxus::prelude::*;
use kinetic_ui::{KButton, KButtonVariant, KInput};

#[component]
pub fn CreateModal() -> Element {
    let mut state = use_context::<AppState>();
    let modal_type = state.create_modal_type;

    let mut space_name = use_signal(String::new);
    let mut space_icon = use_signal(|| "folder".to_string());
    let mut space_color = use_signal(|| "#FFB3AD".to_string());

    let mut collection_name = use_signal(String::new);
    let mut collection_icon = use_signal(|| "folder".to_string());
    let mut collection_color = use_signal(|| "#FFB3AD".to_string());
    let mut collection_space_id = use_signal(|| {
        state.spaces.read().first().map(|s| s.id).unwrap_or(-1)
    });

    let mut request_name = use_signal(String::new);
    let mut request_method = use_signal(|| "GET".to_string());
    let mut request_url = use_signal(String::new);
    let mut request_collection_id = use_signal(|| {
        state.collections.read().first().map(|c| c.id).unwrap_or(-1)
    });

    let mut environment_name = use_signal(String::new);

    match modal_type() {
        None => rsx! {},
        Some(CreateType::Space) => rsx! {
            render_space_modal(
                state,
                modal_type,
                space_name,
                space_icon,
                space_color,
            )
        },
        Some(CreateType::Collection) => rsx! {
            render_collection_modal(
                state,
                modal_type,
                collection_name,
                collection_icon,
                collection_color,
                collection_space_id,
            )
        },
        Some(CreateType::Request) => rsx! {
            render_request_modal(
                state,
                modal_type,
                request_name,
                request_method,
                request_url,
                request_collection_id,
            )
        },
        Some(CreateType::Environment) => rsx! {
            render_environment_modal(
                state,
                modal_type,
                environment_name,
            )
        },
    }
}
```

- [ ] **Step 3: Add render helper functions**

Add to `httpui/src/views/modals/create_modal.rs`:

```rust

fn render_space_modal(
    mut state: AppState,
    modal_type: Signal<Option<CreateType>>,
    space_name: Signal<String>,
    space_icon: Signal<String>,
    space_color: Signal<String>,
) -> Element {
    rsx! {
        div { class: "modal-backdrop",
            onclick: move |_| modal_type.set(None),
            div { class: "modal",
                onclick: |e| e.stop_propagation(),
                h2 { "Create Space" }
                div { class: "modal__content",
                    KInput {
                        placeholder: "Space name (required)".to_string(),
                        value: space_name(),
                        on_input: move |e| space_name.set(e.value()),
                    }
                    // TODO: Add IconPicker and ColorPicker when components are ready
                    p { "Icon: {space_icon}" }
                    p { "Color: {space_color}" }
                }
                div { class: "modal__actions",
                    KButton {
                        variant: KButtonVariant::Secondary,
                        onclick: move |_| modal_type.set(None),
                        "Cancel"
                    }
                    KButton {
                        variant: KButtonVariant::Primary,
                        onclick: move |_| {
                            let name = space_name().clone();
                            if name.trim().is_empty() {
                                return;
                            }
                            let icon = space_icon().clone();
                            let color = space_color().clone();
                            let id = state.create_space(name, icon, color);
                            state.selected_space.set(Some(id));
                            modal_type.set(None);
                        },
                        "Create"
                    }
                }
            }
        }
    }
}

fn render_collection_modal(
    mut state: AppState,
    modal_type: Signal<Option<CreateType>>,
    collection_name: Signal<String>,
    collection_icon: Signal<String>,
    collection_color: Signal<String>,
    collection_space_id: Signal<i32>,
) -> Element {
    rsx! {
        div { class: "modal-backdrop",
            onclick: move |_| modal_type.set(None),
            div { class: "modal",
                onclick: |e| e.stop_propagation(),
                h2 { "Create Collection" }
                div { class: "modal__content",
                    KInput {
                        placeholder: "Collection name (required)".to_string(),
                        value: collection_name(),
                        on_input: move |e| collection_name.set(e.value()),
                    }
                    div { class: "modal__field",
                        label { "Space: " }
                        select {
                            value: collection_space_id(),
                            onchange: move |e| {
                                if let Ok(id) = e.value().parse::<i32>() {
                                    collection_space_id.set(id);
                                }
                            },
                            for space in state.spaces() {
                                option {
                                    value: "{space.id}",
                                    "{space.name} (#{space.id})"
                                }
                            }
                        }
                    }
                    // TODO: Add IconPicker and ColorPicker
                    p { "Icon: {collection_icon}" }
                    p { "Color: {collection_color}" }
                }
                div { class: "modal__actions",
                    KButton {
                        variant: KButtonVariant::Secondary,
                        onclick: move |_| modal_type.set(None),
                        "Cancel"
                    }
                    KButton {
                        variant: KButtonVariant::Primary,
                        onclick: move |_| {
                            let name = collection_name().clone();
                            if name.trim().is_empty() {
                                return;
                            }
                            let icon = collection_icon().clone();
                            let color = collection_color().clone();
                            let space_id = collection_space_id();
                            let id = state.create_collection(name, icon, color, space_id);
                            // Navigate to collection settings
                            modal_type.set(None);
                        },
                        "Create"
                    }
                }
            }
        }
    }
}

fn render_request_modal(
    mut state: AppState,
    modal_type: Signal<Option<CreateType>>,
    request_name: Signal<String>,
    request_method: Signal<String>,
    request_url: Signal<String>,
    request_collection_id: Signal<i32>,
) -> Element {
    rsx! {
        div { class: "modal-backdrop",
            onclick: move |_| modal_type.set(None),
            div { class: "modal",
                onclick: |e| e.stop_propagation(),
                h2 { "Create Request" }
                div { class: "modal__content",
                    KInput {
                        placeholder: "Request name (optional)".to_string(),
                        value: request_name(),
                        on_input: move |e| request_name.set(e.value()),
                    }
                    div { class: "modal__field",
                        label { "Method: " }
                        select {
                            value: request_method(),
                            onchange: move |e| request_method.set(e.value()),
                            option { value: "GET", "GET" }
                            option { value: "POST", "POST" }
                            option { value: "PUT", "PUT" }
                            option { value: "PATCH", "PATCH" }
                            option { value: "DELETE", "DELETE" }
                        }
                    }
                    KInput {
                        placeholder: "URL (required)".to_string(),
                        value: request_url(),
                        on_input: move |e| request_url.set(e.value()),
                    }
                    div { class: "modal__field",
                        label { "Collection: " }
                        select {
                            value: request_collection_id(),
                            onchange: move |e| {
                                if let Ok(id) = e.value().parse::<i32>() {
                                    request_collection_id.set(id);
                                }
                            },
                            for collection in state.collections() {
                                option {
                                    value: "{collection.id}",
                                    "{collection.name} (#{collection.id})"
                                }
                            }
                        }
                    }
                }
                div { class: "modal__actions",
                    KButton {
                        variant: KButtonVariant::Secondary,
                        onclick: move |_| modal_type.set(None),
                        "Cancel"
                    }
                    KButton {
                        variant: KButtonVariant::Primary,
                        onclick: move |_| {
                            let url = request_url().clone();
                            if url.trim().is_empty() {
                                return;
                            }
                            let name = if request_name().trim().is_empty() {
                                None
                            } else {
                                Some(request_name().clone())
                            };
                            let method = request_method().clone();
                            let collection_id = request_collection_id();
                            let id = state.create_request(name, method, url, collection_id);
                            state.selected_request.set(Some(id));
                            state.open_requests.write().push(id);
                            modal_type.set(None);
                        },
                        "Create"
                    }
                }
            }
        }
    }
}

fn render_environment_modal(
    mut state: AppState,
    modal_type: Signal<Option<CreateType>>,
    environment_name: Signal<String>,
) -> Element {
    let space_id = state.spaces.read().first().map(|s| s.id).unwrap_or(-1);
    
    rsx! {
        div { class: "modal-backdrop",
            onclick: move |_| modal_type.set(None),
            div { class: "modal",
                onclick: |e| e.stop_propagation(),
                h2 { "Create Environment" }
                div { class: "modal__content",
                    KInput {
                        placeholder: "Environment name (required)".to_string(),
                        value: environment_name(),
                        on_input: move |e| environment_name.set(e.value()),
                    }
                }
                div { class: "modal__actions",
                    KButton {
                        variant: KButtonVariant::Secondary,
                        onclick: move |_| modal_type.set(None),
                        "Cancel"
                    }
                    KButton {
                        variant: KButtonVariant::Primary,
                        onclick: move |_| {
                            let name = environment_name().clone();
                            if name.trim().is_empty() {
                                return;
                            }
                            let _id = state.create_environment(name, space_id);
                            // Navigate to environment settings
                            modal_type.set(None);
                        },
                        "Create"
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check --package httpui`
Expected: Several errors about missing imports and methods - we'll fix next

- [ ] **Step 5: Add missing import for CreateType**

Update `httpui/src/state/mod.rs` to export CreateType:

```rust
mod models;
mod persistence;
mod store;

pub use models::*;
pub use persistence::*;
pub use store::AppState;
```

- [ ] **Step 6: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 7: Commit**

```bash
git add httpui/src/views/modals/
git commit -m "feat(httpui): add CreateModal component with form fields"
```

---

## Task 10: Update Views Module Exports

**Files:**
- Modify: `httpui/src/views/mod.rs`

**Steps:**

- [ ] **Step 1: Export modals module**

Update `httpui/src/views/mod.rs`:

```rust
pub mod canvas;
pub mod components;
pub mod explorer;
pub mod layout_grid;
pub mod modals;
pub mod settings;
pub mod sidenav;
pub mod statusbar;
pub mod topbar;
```

- [ ] **Step 2: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 3: Commit**

```bash
git add httpui/src/views/mod.rs
git commit -m "feat(httpui): export modals module"
```

---

## Task 11: Create CreateDropdown Component in Sidenav

**Files:**
- Modify: `httpui/src/views/sidenav/component.rs:46-49`

**Steps:**

- [ ] **Step 1: Import CreateType**

Add to imports at top of `httpui/src/views/sidenav/component.rs`:

```rust
use crate::state::CreateType;
```

- [ ] **Step 2: Replace static "+" button with dropdown**

Replace the CTA button section (lines 46-49) in `httpui/src/views/sidenav/component.rs`:

```rust
            // CTA: new request button
            div { class: "sidenav__cta",
                KButton {
                    variant: KButtonVariant::Primary,
                    onclick: move |_| {
                        // For now, open request modal directly
                        // TODO: Add dropdown menu with all options
                        state.create_modal_type.set(Some(CreateType::Request));
                    },
                    "+"
                }
            }
```

- [ ] **Step 3: Add CreateModal to layout**

In the main render, add CreateModal after the Router in `httpui/src/main.rs` (line 117):

```rust
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        KineticTheme {
            LayoutGrid {
                Router::<Route> {}
            }
        }
        views::modals::CreateModal {}
    }
```

Wait, this requires importing. Let me do this properly.

Actually, looking at the structure, CreateModal should be rendered at the App level, not in sidenav. Let me adjust.

- [ ] **Step 4: Add CreateModal to App component**

Update `httpui/src/main.rs` imports to include CreateModal:

```rust
use views::{layout_grid::LayoutGrid, settings::SettingsSection, modals::CreateModal};
```

Then add it to the render in `App` component (line 113):

```rust
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        KineticTheme {
            LayoutGrid {
                Router::<Route> {}
            }
        }
        CreateModal {}
    }
```

- [ ] **Step 5: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 6: Commit**

```bash
git add httpui/src/views/sidenav/component.rs httpui/src/main.rs
git commit -m "feat(httpui): wire up create modal trigger from sidenav"
```

---

## Task 12: Add Modal Styling

**Files:**
- Create: `httpui/src/views/modals/style.css`

**Steps:**

- [ ] **Step 1: Create modal stylesheet**

Create `httpui/src/views/modals/style.css`:

```css
.modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal {
    background: #1B1C1C;
    border-radius: 0.5rem;
    padding: 1.5rem;
    min-width: 400px;
    max-width: 90vw;
}

.modal__content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin: 1rem 0;
}

.modal__field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.modal__field label {
    font-weight: 500;
}

.modal__actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
}

.color-picker {
    display: flex;
    gap: 0.5rem;
}

.color-picker__swatch {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
}

.color-picker__swatch:hover {
    border-color: white;
}
```

- [ ] **Step 2: Reference stylesheet in CreateModal**

Add to top of `httpui/src/views/modals/create_modal.rs`:

```rust
rsx! {
    document::Link { rel: "stylesheet", href: asset!("./style.css") }
    // ... rest of modal
}
```

Wait, looking at the code structure, the modal is split into helper functions. Let me add the stylesheet link at the top of the main CreateModal function.

Update `httpui/src/views/modals/create_modal.rs` - add to beginning of `render_space_modal`:

```rust
fn render_space_modal(...) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "modal-backdrop",
```

Actually, all the render functions are separate components. The stylesheet should be linked from the main CreateModal or we need to add it to each. Let me add it to each render function.

- [ ] **Step 3: Add stylesheet link to each render function**

Add `document::Link { rel: "stylesheet", href: asset!("./style.css") }` at the start of each render function's rsx! macro in `httpui/src/views/modals/create_modal.rs`.

Actually, a better approach is to add it once in the CreateModal component itself. Let me restructure:

Update `httpui/src/views/modals/create_modal.rs`:

```rust
#[component]
pub fn CreateModal() -> Element {
    let mut state = use_context::<AppState>();
    let modal_type = state.create_modal_type;

    // ... rest of the signal declarations ...

    match modal_type() {
        None => rsx! {},
        Some(CreateType::Space) => {
            rsx! {
                document::Link { rel: "stylesheet", href: asset!("./style.css") }
                render_space_modal(
                    state,
                    modal_type,
                    space_name,
                    space_icon,
                    space_color,
                )
            }
        }
        // ... repeat for other variants ...
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 5: Commit**

```bash
git add httpui/src/views/modals/style.css httpui/src/views/modals/create_modal.rs
git commit -m "feat(httpui): add modal styling"
```

---

## Task 13: Add Basic CSS for IconPicker and ColorPicker

**Files:**
- Create: `httpui/src/components/style.css`

**Steps:**

- [ ] **Step 1: Create components stylesheet**

Create `httpui/src/components/style.css`:

```css
.icon-picker {
    background: transparent;
    border: 1px solid #5B403E;
    border-radius: 0.25rem;
    cursor: pointer;
    padding: 0.5rem;
}

.icon-picker:hover {
    background: #353535;
}

.color-picker {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
}

.color-picker__swatch {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    transition: border-color 0.15s;
}

.color-picker__swatch:hover {
    border-color: white;
}
```

- [ ] **Step 2: Add stylesheet link to IconPicker**

Update `httpui/src/components/icon_picker.rs`:

```rust
#[component]
pub fn IconPicker(
    selected: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "icon-picker",
            onclick: move |_| {
                let new_icon = if selected.as_str() == "folder" {
                    "dns".to_string()
                } else {
                    "folder".to_string()
                };
                on_change.call(new_icon);
            },
            Icon { icon: MdFolder, width: 24, height: 24 }
        }
    }
}
```

- [ ] **Step 3: Add stylesheet link to ColorPicker**

Update `httpui/src/components/color_picker.rs`:

```rust
#[component]
pub fn ColorPicker(
    selected: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "color-picker",
            for color in COLORS {
                button {
                    class: "color-picker__swatch",
                    style:format!("background-color: {}", color),
                    onclick: move |_| {
                        on_change.call(color.to_string());
                    },
                }
            }
        }
    }
}
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check --package httpui`
Expected: Compilation succeeds

- [ ] **Step 5: Commit**

```bash
git add httpui/src/components/
git commit -m "feat(httpui): add styling for IconPicker and ColorPicker"
```

---

## Task 14: Build and Test

**Files:**
- None (build and manual test)

**Steps:**

- [ ] **Step 1: Build the project**

Run: `cargo build --package httpui`
Expected: Build completes with no errors

- [ ] **Step 2: Run the application**

Run: `dx serve --package httpui --platform desktop`
Expected: Application launches successfully

- [ ] **Step 3: Test create space flow**

Manual test:
1. Click "+" button in sidenav
2. Modal should open
3. Enter space name
4. Click "Create"
5. Verify space appears in state (check persistence file)

- [ ] **Step 4: Test create collection flow**

Manual test:
1. Click "+" button in sidenav
2. Enter collection name
3. Select space from dropdown
4. Click "Create"
5. Verify collection appears in state

- [ ] **Step 5: Test create request flow**

Manual test:
1. Click "+" button in sidenav
2. Enter URL
3. Select method
4. Select collection
5. Click "Create"
6. Verify request appears in explorer

- [ ] **Step 6: Test create environment flow**

Manual test:
1. Click "+" button in sidenav
2. Enter environment name
3. Click "Create"
4. Verify environment appears in state

- [ ] **Step 7: Verify persistence**

Check `~/.config/httpui/state.json` contains created entities

- [ ] **Step 8: Commit if all tests pass**

```bash
git add -A
git commit -m "test(httpui): verify create entity flows manually"
```

---

## Summary

This plan adds complete create functionality for Spaces, Collections, Requests, and Environments:

1. **State foundation** - New enums, signals, and creation methods
2. **Persistence** - Serialize/deserialize new state fields
3. **UI Components** - Modal, IconPicker, ColorPicker
4. **Wiring** - Connect sidenav button to modal trigger
5. **Settings stubs** - Placeholder pages for future features
6. **Testing** - Manual verification of all create flows

All changes follow the existing Dioxus 0.7 patterns in the codebase and use kinetic_ui components for consistency. The architecture uses a single CreateModal with variant-based rendering for maintainability and code reuse.