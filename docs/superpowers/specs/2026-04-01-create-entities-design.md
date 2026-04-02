# Design: Create Entities Feature

**Date:** April 1, 2026
**Package:** httpui
**Status:** Approved

## Overview

Add create functionality for Spaces, Collections, Requests, and Environments via a dropdown menu triggered by the "+" button in the side navigation. Each entity type opens a modal with contextual defaults and form fields. Upon creation, the app navigates to the appropriate view.

## Architecture

### New Components

| Component | Location | Purpose |
|-----------|----------|---------|
| `CreateDropdown` | `src/views/sidenav/` | Plus button with dropdown menu offering create options |
| `CreateModal` | `src/views/modals/` | Single modal component handling all entity types |
| `IconPicker` | `src/components/` | Icon selection popover using dioxus_free_icons |
| `ColorPicker` | `src/components/` | Color selection popover using DESIGN.md palette |
| `CollectionSettings` | `src/views/settings/` | Stub page for collection headers/auth inheritance |
| `EnvironmentSettings` | `src/views/settings/` | Stub page for environment variables |

### State Modifications

Add to `AppState` (in `src/state/store.rs`):

```rust
pub create_modal_type: Signal<Option<CreateType>>,
pub selected_space: Signal<Option<i32>>,
pub selected_collection: Signal<Option<i32>>,
```

### New Enum

Add to `src/state/models.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Serialize, Deserialize)]
pub enum CreateType {
    Space,
    Collection,
    Request,
    Environment,
}
```

### Route Additions

Add to `Route` enum in `src/main.rs`:

```rust
#[route("/settings/collection/:id")]
CollectionSettings { id: i32 },

#[route("/settings/environment/:id")]
EnvironmentSettings { id: i32 },
```

## Component Specifications

### CreateDropdown

**Location:** `src/views/sidenav/component.rs`

**Behavior:**
- Replaces the current static "+" `KButton`
- On click, shows dropdown menu with 4 options:
  - "New Space"
  - "New Collection"
  - "New Request"
  - "New Environment"
- Selecting an option sets `create_modal_type` to the corresponding `CreateType` variant
- Uses kinetic_ui dropdown/popover pattern (similar to `KSelect`)

### CreateModal

**Location:** `src/views/modals/create_modal.rs`

**Structure:**
- Rendered conditionally when `create_modal_type().is_some()`
- Contains a `match` on `CreateType` to render appropriate form fields
- Maintains local form state via `use_signal` for each field
- Form fields validated before submission
- On submit: call `AppState` creation method, navigate, close modal
- On cancel: clear local form state, close modal

**Form Fields per Type:**

#### Space
| Field | Type | Required | Default |
|-------|------|----------|---------|
| name | text | yes | (empty) |
| icon | IconPicker | no | `"MdFolder"` |
| color | ColorPicker | no | `"#FFB3AD"` |

#### Collection
| Field | Type | Required | Default |
|-------|------|----------|---------|
| name | text | yes | (empty) |
| icon | IconPicker | no | `"MdFolder"` |
| color | ColorPicker | no | `"#FFB3AD"` |
| space_id | dropdown | yes | first available space |

#### Request
| Field | Type | Required | Default |
|-------|------|----------|---------|
| name | text | no | (empty) |
| method | dropdown | yes | `"GET"` |
| url | text | yes | (empty) |
| collection_id | dropdown | yes | first available collection |

#### Environment
| Field | Type | Required | Default |
|-------|------|----------|---------|
| name | text | yes | (empty) |
| space_id | hidden | yes | first available space |

### IconPicker

**Location:** `src/components/icon_picker.rs`

**Behavior:**
- Button that opens a popover/grid of icons
- Icons sourced from `dioxus_free_icons::icons::md_file_icons` and `dioxus_free_icons::icons::md_action_icons`
- Category tabs: Folders, Actions, etc.
- On select, updates form field with icon struct name (e.g., `MdFolder`)
- Default icon stored as string in model, rendered as `<Icon icon={MdFolder} />`

### ColorPicker

**Location:** `src/components/color_picker.rs`

**Behavior:**
- Button that opens a color swatch popover
- Colors from DESIGN.md palette:
  - Primary: `#FFB3AD`
  - Secondary: `#44E2CD`
  - Tertiary: `#F9BD22`
  - Background variants: `#131313`, `#1B1C1C`, `#353535`
- On select, updates form field with hex color string

### Dropdown Components (Space/Collection Selectors)

**Behavior:**
- Use kinetic_ui `KSelect` pattern
- Display format: `"{name} (#{id})"` to help distinguish duplicate names
- Sorted alphabetically by name
- Default selection: first item in list if available
- If no entities exist, show disabled state with helper text: "Create a space first" or "Create a collection first"
- Submit button disabled if required dropdown has no selection

## Creation Flow

1. User clicks "+" button in sidenav
2. `CreateDropdown` opens showing 4 options
3. User selects "New X"
4. `create_modal_type.set(Some(CreateType::X))`
5. `CreateModal` opens with contextual defaults
6. User fills form:
   - Name fields validate for non-empty (if required)
   - Dropdowns show available entities with ghost ID text
   - Icon/Color pickers show defaults
7. User clicks "Create"
8. `AppState` method creates entity:
   - Gets next ID from `next_X_id` signal
   - Increments `next_X_id`
   - Pushes entity to appropriate `Vec`
   - Updates navigation state
9. `use_effect` in `main.rs` triggers persistence save
10. Modal closes, navigation occurs:
    - Space: `selected_space` set, stay in Collections view
    - Collection: Navigate to `/settings/collection/{id}`
    - Request: `selected_request` set, add to `open_requests`
    - Environment: Navigate to `/settings/environment/{id}`

## Defaults

### Icon Defaults
- Spaces: `MdFolder` (stored as `"folder"` string in model, mapped to `MdFolder` icon component)
- Collections: `MdFolder` (stored as `"folder"` string in model)

### Color Defaults
- Spaces: `#FFB3AD` (primary bold red from DESIGN.md)
- Collections: `#FFB3AD` (primary) - could rotate through palette later

### Dropdown Defaults
- Space dropdown (for collections): First space in `spaces()` list
- Collection dropdown (for requests): First collection in `collections()` list

## Settings Pages (Stubs)

### CollectionSettings

**Route:** `/settings/collection/:id`

**Content:**
- Header showing collection name
- "Coming soon" placeholder text
- Future: configure headers/auth inheritance for requests in this collection

### EnvironmentSettings

**Route:** `/settings/environment/:id`

**Content:**
- Header showing environment name
- "Coming soon" placeholder text
- Future: configure key-value variables for URL/header substitution

## File Structure

```
httpui/src/
├── components/
│   ├── mod.rs
│   ├── icon_picker.rs
│   └── color_picker.rs
├── views/
│   ├── modals/
│   │   ├── mod.rs
│   │   └── create_modal.rs
│   ├── sidenav/
│   │   └── component.rs (modified)
│   └── settings/
│       ├── mod.rs (modified)
│       ├── collection_settings.rs (new)
│       └── environment_settings.rs (new)
└── state/
    ├── models.rs (modified - add CreateType enum)
    ├── store.rs (modified - add new signals)
    └── persistence.rs (modified - add new fields to PersistentState)
```

## Implementation Notes

### AppState Methods

Add creation methods to `AppState`:
- `create_space(name: String, icon: String, color: String) -> i32`
- `create_collection(name: String, icon: String, color: String, space_id: i32) -> i32`
- `create_request(name: Option<String>, method: String, url: String, collection_id: i32) -> i32`
- `create_environment(name: String, space_id: i32) -> i32`

Each method:
1. Gets next ID from `next_X_id` signal
2. Increments `next_X_id`
3. Constructs entity with defaults
4. Pushes to appropriate Vec
5. Returns new entity ID for navigation

### Persistence

The existing `use_effect` in `main.rs` automatically saves state when signals change. No special handling needed for create operations beyond updating the signal Vecs.

### Validation

- Required text fields: check `trim().is_empty()`
- Required dropdowns: check `is_some()` on selected value
- Show inline validation errors below fields

### Component Reuse

- `IconPicker` and `ColorPicker` should be reusable in future edit/create forms
- `CreateModal` structure can extend to edit operations later

### Future Enhancements (Out of Scope)

- Track `selected_space` and `selected_collection` based on explorer navigation
- Collection and Environment settings pages implementation
- Request inheritance of collection headers/auth
- Variable substitution in URLs and headers