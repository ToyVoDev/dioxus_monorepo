# Kinetic UI Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create the `kinetic_ui` shared component library with the Kinetic Obsidian design system, then rebuild httpui's layout to match the Stitch mockups.

**Architecture:** Foundation-up — theme tokens and crate scaffold first, then layout shell, then components as needed. `kinetic_ui` is a workspace crate wrapping `dioxus-primitives` with BEM-namespaced CSS. httpui consumes it as a path dependency.

**Tech Stack:** Rust nightly, Dioxus 0.7, dioxus-primitives, CSS custom properties, BEM naming (`k-` prefix, `--k-` tokens)

**Spec:** `httpui/docs/superpowers/specs/2026-03-20-kinetic-ui-redesign-design.md`

**No tests currently exist in this workspace.** The CLAUDE.md explicitly states "There are no tests currently configured in any project." Steps that would normally be TDD will instead use `cargo check --package` and `dx serve --package httpui` for visual verification. `cargo clippy` is configured with `pedantic` + `nursery` deny.

**Reference files:**

- Design system: `DESIGN.md` (repo root)
- Stitch mockups: `httpui/docs/stitch/request_editor/code.html`, `httpui/docs/stitch/collections_environments/code.html`
- Current httpui source: `httpui/src/`
- Dioxus 0.7 conventions: `AGENTS.md` (repo root)

**Note on `dioxus-primitives`:** This is a git dependency (`DioxusLabs/components`, commit `8d65778`). All primitives (accordion, select, tabs, tooltip, separator, collapsible, dialog, dropdown_menu, etc.) are available with `default-features = false`. The exact type names and APIs should be verified against the source at `~/.cargo/git/checkouts/components-*/8d65778/primitives/src/`. The httpui crate currently uses `accordion`, `select`, and `separator` primitives successfully with this config.

---

## File Structure

### New files (kinetic_ui crate)

| File                                                  | Responsibility                          |
| ----------------------------------------------------- | --------------------------------------- |
| `kinetic_ui/Cargo.toml`                               | Crate manifest, workspace dependencies  |
| `kinetic_ui/src/lib.rs`                               | Re-exports theme + all components       |
| `kinetic_ui/src/theme/mod.rs`                         | `KineticTheme` component that loads CSS |
| `kinetic_ui/src/theme/kinetic-theme.css`              | Design tokens (CSS custom properties)   |
| `kinetic_ui/src/theme/typography.css`                 | Font imports + type scale classes       |
| `kinetic_ui/src/theme/utilities.css`                  | BEM utility classes                     |
| `kinetic_ui/src/components/mod.rs`                    | Re-exports all components               |
| `kinetic_ui/src/components/button/mod.rs`             | Module declaration                      |
| `kinetic_ui/src/components/button/component.rs`       | Button + LinkButton components          |
| `kinetic_ui/src/components/button/style.css`          | Button variants CSS                     |
| `kinetic_ui/src/components/input/mod.rs`              | Module declaration                      |
| `kinetic_ui/src/components/input/component.rs`        | Input component                         |
| `kinetic_ui/src/components/input/style.css`           | Input CSS                               |
| `kinetic_ui/src/components/badge/mod.rs`              | Module declaration                      |
| `kinetic_ui/src/components/badge/component.rs`        | Badge component (method, status)        |
| `kinetic_ui/src/components/badge/style.css`           | Badge CSS                               |
| `kinetic_ui/src/components/icon_button/mod.rs`        | Module declaration                      |
| `kinetic_ui/src/components/icon_button/component.rs`  | IconButton component                    |
| `kinetic_ui/src/components/icon_button/style.css`     | IconButton CSS                          |
| `kinetic_ui/src/components/tabs/mod.rs`               | Module declaration                      |
| `kinetic_ui/src/components/tabs/component.rs`         | Tabs wrapping dioxus-primitives tabs    |
| `kinetic_ui/src/components/tabs/style.css`            | Tabs CSS                                |
| `kinetic_ui/src/components/select/mod.rs`             | Module declaration                      |
| `kinetic_ui/src/components/select/component.rs`       | Select wrapping dioxus-primitives       |
| `kinetic_ui/src/components/select/style.css`          | Select CSS                              |
| `kinetic_ui/src/components/separator/mod.rs`          | Module declaration                      |
| `kinetic_ui/src/components/separator/component.rs`    | Separator component                     |
| `kinetic_ui/src/components/separator/style.css`       | Separator CSS                           |
| `kinetic_ui/src/components/table/mod.rs`              | Module declaration                      |
| `kinetic_ui/src/components/table/component.rs`        | Key-value editable table                |
| `kinetic_ui/src/components/table/style.css`           | Table CSS                               |
| `kinetic_ui/src/components/tooltip/mod.rs`            | Module declaration                      |
| `kinetic_ui/src/components/tooltip/component.rs`      | Tooltip wrapping dioxus-primitives      |
| `kinetic_ui/src/components/tooltip/style.css`         | Tooltip CSS                             |
| `kinetic_ui/src/components/search_input/mod.rs`       | Module declaration                      |
| `kinetic_ui/src/components/search_input/component.rs` | Search input with icon                  |
| `kinetic_ui/src/components/search_input/style.css`    | Search input CSS                        |
| `kinetic_ui/src/components/tree_view/mod.rs`          | Module declaration                      |
| `kinetic_ui/src/components/tree_view/component.rs`    | Expandable tree view                    |
| `kinetic_ui/src/components/tree_view/style.css`       | Tree view CSS                           |
| `kinetic_ui/src/components/accordion/mod.rs`          | Module declaration                      |
| `kinetic_ui/src/components/accordion/component.rs`    | Accordion wrapping dioxus-primitives    |
| `kinetic_ui/src/components/accordion/style.css`       | Accordion CSS                           |

### New files (httpui views)

| File                                      | Responsibility                          |
| ----------------------------------------- | --------------------------------------- |
| `httpui/src/views/sidenav/mod.rs`         | Module declaration                      |
| `httpui/src/views/sidenav/component.rs`   | SideNav component                       |
| `httpui/src/views/sidenav/style.css`      | SideNav layout CSS                      |
| `httpui/src/views/explorer/mod.rs`        | Module declaration                      |
| `httpui/src/views/explorer/component.rs`  | Explorer panel component                |
| `httpui/src/views/explorer/style.css`     | Explorer layout CSS                     |
| `httpui/src/views/topbar/mod.rs`          | Module declaration                      |
| `httpui/src/views/topbar/component.rs`    | TopBar component                        |
| `httpui/src/views/topbar/style.css`       | TopBar layout CSS                       |
| `httpui/src/views/canvas/mod.rs`          | Module declaration                      |
| `httpui/src/views/canvas/component.rs`    | Canvas (request editor, env view, etc.) |
| `httpui/src/views/canvas/style.css`       | Canvas layout CSS                       |
| `httpui/src/views/statusbar/mod.rs`       | Module declaration                      |
| `httpui/src/views/statusbar/component.rs` | StatusBar stub component                |
| `httpui/src/views/statusbar/style.css`    | StatusBar CSS                           |

### Modified files

| File                                        | Changes                                                                      |
| ------------------------------------------- | ---------------------------------------------------------------------------- |
| `Cargo.toml` (workspace root)               | Add `kinetic_ui` to members + workspace deps                                 |
| `httpui/Cargo.toml`                         | Add `kinetic_ui` dependency, remove `dioxus-primitives` direct dep           |
| `httpui/src/main.rs`                        | Swap LayoutGrid + Navbar for new layout, use KineticTheme, update Route enum |
| `httpui/src/state/models.rs`                | Add `KeyValue`, `Response` structs, extend `Request`, add nav/tab enums      |
| `httpui/src/state/store.rs`                 | Add new signal fields to AppState                                            |
| `httpui/src/views/mod.rs`                   | Replace old view modules with new ones                                       |
| `httpui/src/views/layout_grid/component.rs` | Rewrite grid to 3-column layout                                              |
| `httpui/src/views/layout_grid/style.css`    | New grid CSS                                                                 |

### Files to remove (after migration)

Old views and components that are fully replaced:

- `httpui/src/views/navbar/` — replaced by `sidenav/`
- `httpui/src/views/library/` — replaced by `explorer/`
- `httpui/src/views/tabbar/` — replaced by `topbar/`
- `httpui/src/views/urlbar/` — merged into `canvas/`
- `httpui/src/views/request_editor/` — merged into `canvas/`
- `httpui/src/views/response_viewer/` — merged into `canvas/`
- `httpui/src/components/` — all moved to `kinetic_ui`
- `httpui/assets/dx-components-theme.css` — replaced by kinetic theme

---

## Task 1: Create `kinetic_ui` crate scaffold + theme CSS

**Files:**

- Create: `kinetic_ui/Cargo.toml`
- Create: `kinetic_ui/src/lib.rs`
- Create: `kinetic_ui/src/theme/mod.rs`
- Create: `kinetic_ui/src/theme/kinetic-theme.css`
- Create: `kinetic_ui/src/theme/typography.css`
- Create: `kinetic_ui/src/theme/utilities.css`
- Create: `kinetic_ui/src/components/mod.rs`
- Modify: `Cargo.toml` (workspace root)

- [ ] **Step 1: Create `kinetic_ui/Cargo.toml`**

```toml
[package]
name = "kinetic_ui"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { workspace = true, features = ["router"] }
dioxus-free-icons = { workspace = true, features = [
    "codicons",
    "material-design-icons-navigation",
    "material-design-icons-file",
    "material-design-icons-action",
    "material-design-icons-content",
] }
dioxus-primitives = { workspace = true, default-features = false }
strum = { workspace = true, features = ["derive"] }

[lints.clippy]
pedantic = "deny"
nursery = "deny"
```

- [ ] **Step 2: Add `kinetic_ui` to workspace `Cargo.toml`**

Add `"kinetic_ui"` to the `[workspace].members` array. Add `kinetic_ui = { path = "kinetic_ui" }` to `[workspace.dependencies]`.

- [ ] **Step 3: Create `kinetic_ui/src/theme/kinetic-theme.css`**

Write the full design token CSS file. All tokens from spec Section 4: surfaces (`--k-surface-lowest` through `--k-surface-highest`, `--k-surface-container`, `--k-glass-surface`), brand colors (`--k-primary`, `--k-primary-container`, `--k-on-primary-container`, `--k-secondary`, `--k-secondary-container`, `--k-tertiary`, `--k-error`, `--k-on-surface`, `--k-on-surface-variant`, `--k-outline-variant`), spacing (`--k-space-1` through `--k-space-8`), rounding (`--k-radius-sm` through `--k-radius-full`), shadow (`--k-shadow-float`), and typography tokens (`--k-font-display`, `--k-font-body`, `--k-font-mono`).

Apply as `:root` variables. Include a base reset: `*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }` and `body { background: var(--k-surface); color: var(--k-on-surface); font-family: var(--k-font-body); }`.

Reference spec Section 4 for all exact hex values.

- [ ] **Step 4: Create `kinetic_ui/src/theme/typography.css`**

Import fonts from Google Fonts: Space Grotesk (300,400,500,600,700), Inter (300,400,500,600), JetBrains Mono (400,500,600). Define type scale classes per spec Section 4.3: `.k-display-lg`, `.k-headline-md`, `.k-title-md`, `.k-body-md`, `.k-label-md`, `.k-label-sm`.

- [ ] **Step 5: Create `kinetic_ui/src/theme/utilities.css`**

Define BEM utility classes for common token applications:

- Surface utilities: `.k-surface-lowest`, `.k-surface`, `.k-surface-low`, `.k-surface-container`, `.k-surface-high`, `.k-surface-highest` (set `background-color`)
- Text utilities: `.k-text-primary`, `.k-text-secondary`, `.k-text-tertiary`, `.k-text-error`, `.k-text-on-surface`, `.k-text-on-surface-variant` (set `color`)
- A `.k-mono` class that sets `font-family: var(--k-font-mono)`
- A `.k-glass` class for glassmorphism: `background: var(--k-glass-surface); backdrop-filter: blur(24px);`

- [ ] **Step 6: Create `kinetic_ui/src/theme/mod.rs`**

```rust
use dioxus::prelude::*;

#[component]
pub fn KineticTheme(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./kinetic-theme.css") }
        document::Link { rel: "stylesheet", href: asset!("./typography.css") }
        document::Link { rel: "stylesheet", href: asset!("./utilities.css") }
        {children}
    }
}
```

- [ ] **Step 7: Create `kinetic_ui/src/components/mod.rs`**

Empty for now — components added in later tasks:

```rust
// Components will be added as they are built
```

- [ ] **Step 8: Create `kinetic_ui/src/lib.rs`**

```rust
pub mod theme;
pub mod components;

pub use theme::KineticTheme;
```

- [ ] **Step 9: Verify it compiles**

Run: `cargo check --package kinetic_ui`
Expected: Compiles with no errors.

- [ ] **Step 10: Commit**

```bash
git add kinetic_ui/ Cargo.toml
git commit -m "feat: scaffold kinetic_ui crate with Kinetic Obsidian theme tokens"
```

---

## Task 2: Create core components — Button, Input, Badge, IconButton

**Files:**

- Create: `kinetic_ui/src/components/button/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/input/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/badge/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/icon_button/{mod.rs,component.rs,style.css}`
- Modify: `kinetic_ui/src/components/mod.rs`

- [ ] **Step 1: Create Button component**

Port from `httpui/src/components/button/component.rs` but restyle with BEM classes. Variants: Primary (gradient from `--k-primary` to `--k-primary-container` at 135deg), Secondary (`surface-highest` fill + ghost border), Ghost (text only, hover to `surface-high`), Destructive (same as primary but for dangerous actions).

`style.css`: All classes use `.k-button` prefix. `.k-button--primary` gets the gradient. Hover state: `filter: brightness(1.1)`. Active: `filter: brightness(0.95)`. Use `--k-radius-default` for border-radius.

`component.rs`: `#[component]` fn with `variant: ButtonVariant`, `onclick`, `children: Element`. The existing httpui Button has `onmousedown`/`onmouseup` — preserve those as optional props. Use `#[derive(Clone, PartialEq, strum::Display)]` for `ButtonVariant` enum to generate data-attribute values.

`mod.rs`: `mod component; pub use component::*;`

- [ ] **Step 2: Create Input component**

Port from `httpui/src/components/input/component.rs`. Restyle with BEM.

`style.css`: `.k-input` base — `background: var(--k-surface-lowest)`, `color: var(--k-on-surface)`, `border-radius: var(--k-radius-default)`, `border: 1px solid transparent`. Focus: `border-color: rgba(91, 64, 62, 0.4)` (outline-variant at 40%). Add `.k-input--mono` modifier that sets `font-family: var(--k-font-mono)`.

`component.rs`: Same prop surface as current (oninput, onchange, onfocus, onblur, etc.) plus `monospace: bool` prop that adds the `--mono` modifier class.

- [ ] **Step 3: Create Badge component**

New component for method and status badges.

`style.css`: `.k-badge` base — `display: inline-flex`, `padding: 2px 8px`, `border-radius: var(--k-radius-full)`, `font-family: var(--k-font-mono)`, `font-size: 0.6875rem`, `font-weight: 600`, `text-transform: uppercase`. Data-attribute variants for colors: `[data-variant="primary"]` uses `--k-primary`, `[data-variant="secondary"]` uses `--k-secondary`, etc.

`component.rs`: Props: `variant: BadgeVariant`, `children: Element`. `BadgeVariant` enum: `Primary, Secondary, Tertiary, Error, Muted`.

- [ ] **Step 4: Create IconButton component**

New component for SideNav and TopBar icon buttons.

`style.css`: `.k-icon-button` — `display: flex`, `align-items: center`, `justify-content: center`, `width: 36px`, `height: 36px`, `border-radius: var(--k-radius-default)`, `border: none`, `background: transparent`, `color: var(--k-on-surface-variant)`, `cursor: pointer`. Hover: `background: var(--k-surface-high)`. Active variant: `color: var(--k-primary)`, `background: var(--k-surface-high)`.

`component.rs`: Props: `onclick`, `active: Option<bool>`, `children: Element` (the icon element).

- [ ] **Step 5: Update `kinetic_ui/src/components/mod.rs`**

```rust
pub mod button;
pub mod input;
pub mod badge;
pub mod icon_button;
```

- [ ] **Step 6: Update `kinetic_ui/src/lib.rs`**

Add re-exports for the new components.

- [ ] **Step 7: Verify it compiles**

Run: `cargo check --package kinetic_ui`
Expected: Compiles with no errors.

- [ ] **Step 8: Commit**

```bash
git add kinetic_ui/
git commit -m "feat(kinetic_ui): add Button, Input, Badge, IconButton components"
```

---

## Task 3: Create Tabs, Select, Separator, Tooltip components

**Files:**

- Create: `kinetic_ui/src/components/tabs/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/select/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/separator/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/tooltip/{mod.rs,component.rs,style.css}`
- Modify: `kinetic_ui/src/components/mod.rs`

- [ ] **Step 1: Create Tabs component**

Wrap `dioxus_primitives::tabs` (which provides `TabList`, `TabTrigger`, `TabContent`).

`style.css`: `.k-tabs` container. `.k-tabs__list` — horizontal flex, `border-bottom: 1px solid rgba(91,64,62,0.1)`, gap `--k-space-4`. `.k-tabs__trigger` — padding, `color: var(--k-on-surface-variant)`, no border, transparent background. Active trigger: `color: var(--k-primary)`, `border-bottom: 2px solid var(--k-primary)`. `.k-tabs__content` — padding `--k-space-4`.

`component.rs`: Thin wrappers around primitives that add the BEM classes. Export: `KTabs`, `KTabList`, `KTabTrigger`, `KTabContent`. `KTabTrigger` accepts optional `badge: Option<u32>` prop to show a count badge next to the label.

- [ ] **Step 2: Create Select component**

Port from `httpui/src/components/select/component.rs`. Restyle with Kinetic Obsidian tokens.

`style.css`: `.k-select__trigger` — similar to button secondary. `.k-select__list` — `background: var(--k-surface-highest)`, `border-radius: var(--k-radius-lg)`, `box-shadow: var(--k-shadow-float)`, `backdrop-filter: blur(24px)` (glassmorphism for the dropdown). `.k-select__option` — padding, hover `background: var(--k-surface-high)`.

`component.rs`: Wrap `dioxus_primitives::select` components. Keep the generic `<T: Clone + PartialEq + Display + 'static>` pattern from current httpui. Export: `KSelect`, `KSelectTrigger`, `KSelectValue`, `KSelectList`, `KSelectOption`, `KSelectGroup`, `KSelectGroupLabel`, `KSelectItemIndicator`.

- [ ] **Step 3: Create Separator component**

Port from `httpui/src/components/separator/component.rs`. Minimal — uses spacing/tonal shift per design rules.

`style.css`: `.k-separator` — `background: rgba(91,64,62,0.1)`. Horizontal: `height: 1px; width: 100%`. Vertical: `width: 1px; height: 100%`.

- [ ] **Step 4: Create Tooltip component**

Wrap `dioxus_primitives::tooltip` (`TooltipTrigger`, `TooltipContent`).

`style.css`: `.k-tooltip__content` — `background: var(--k-surface-highest)`, `color: var(--k-on-surface)`, `padding: 4px 8px`, `border-radius: var(--k-radius-sm)`, `font-size: 0.75rem`, `box-shadow: var(--k-shadow-float)`.

- [ ] **Step 5: Update `components/mod.rs` and `lib.rs`**

Add the new modules and re-exports.

- [ ] **Step 6: Verify**

Run: `cargo check --package kinetic_ui`

- [ ] **Step 7: Commit**

```bash
git add kinetic_ui/
git commit -m "feat(kinetic_ui): add Tabs, Select, Separator, Tooltip components"
```

---

## Task 4: Create Table, SearchInput, TreeView, Accordion components

**Files:**

- Create: `kinetic_ui/src/components/table/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/search_input/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/tree_view/{mod.rs,component.rs,style.css}`
- Create: `kinetic_ui/src/components/accordion/{mod.rs,component.rs,style.css}`
- Modify: `kinetic_ui/src/components/mod.rs`

- [ ] **Step 1: Create Table component**

For key-value editing (Params, Headers tables in the request editor).

`style.css`: `.k-table` — `width: 100%`, `border-collapse: collapse`. `.k-table__header` — column labels in `--k-on-surface-variant`, `font-size: 0.6875rem`, `text-transform: uppercase`, `letter-spacing: 0.05em`, `padding: 8px`, `border-bottom: 1px solid rgba(91,64,62,0.1)`. `.k-table__row` — `border-bottom: 1px solid rgba(91,64,62,0.05)`, hover `background: var(--k-surface-low)`. `.k-table__cell` — `padding: 6px 8px`. `.k-table__input` — transparent background, no border, full width, `color: var(--k-on-surface)`, `font-family: var(--k-font-mono)`. `.k-table__add-row` — `color: var(--k-primary)`, `cursor: pointer`, `padding: 8px`.

`component.rs`: Generic table component. Props: `columns: Vec<String>`, `children: Element` (rows rendered by consumer). Also export `KTableRow` and `KTableCell` helper components. The actual data binding is done by the consumer (httpui views), not the table itself.

- [ ] **Step 2: Create SearchInput component**

`style.css`: `.k-search-input` wrapper — `display: flex`, `align-items: center`, `gap: 8px`, `background: var(--k-surface-highest)`, `border-radius: var(--k-radius-full)`, `padding: 6px 12px`. `.k-search-input__icon` — `color: var(--k-on-surface-variant)`. `.k-search-input__field` — transparent background, no border, `color: var(--k-on-surface)`, `font-size: 0.875rem`.

`component.rs`: Wraps a search icon + an Input. Props: `placeholder: String`, `value: String`, `oninput: EventHandler<FormEvent>`.

- [ ] **Step 3: Create TreeView component**

Custom component (no primitives equivalent).

`style.css`: `.k-tree` container. `.k-tree__node` — `padding-left` based on depth (CSS variable `--depth`). `.k-tree__branch` — expandable node with chevron icon. `.k-tree__branch-trigger` — `display: flex`, `align-items: center`, `gap: 8px`, `padding: 6px 8px`, `cursor: pointer`, `border-radius: var(--k-radius-sm)`, hover `background: var(--k-surface-highest)`. `.k-tree__branch-trigger[data-expanded]` rotates the chevron. `.k-tree__leaf` — terminal node, `display: flex`, `align-items: center`, `gap: 8px`, `padding: 6px 8px 6px 28px` (indented past chevron). `.k-tree__leaf[data-selected]` — `background: var(--k-surface-highest)`, `border-left: 2px solid var(--k-primary)`.

`component.rs`: Two components:

- `KTreeBranch` — expandable node. Props: `label: Element`, `initially_expanded: Option<bool>`, `children: Element`. Manages an `expanded` signal internally.
- `KTreeLeaf` — terminal node. Props: `selected: Option<bool>`, `onclick: Option<EventHandler<MouseEvent>>`, `children: Element`.

The tree structure is composed by nesting `KTreeBranch` and `KTreeLeaf` in RSX — no data model imposed by the component.

- [ ] **Step 4: Create Accordion component**

Port from `httpui/src/components/accordion/component.rs`, wrapping `dioxus_primitives::accordion`.

`style.css`: `.k-accordion`, `.k-accordion__item`, `.k-accordion__trigger`, `.k-accordion__content`. Style with Kinetic tokens — trigger hover uses `surface-high`, content has slide animation.

- [ ] **Step 5: Update `components/mod.rs` and `lib.rs`**

- [ ] **Step 6: Verify**

Run: `cargo check --package kinetic_ui`

- [ ] **Step 7: Commit**

```bash
git add kinetic_ui/
git commit -m "feat(kinetic_ui): add Table, SearchInput, TreeView, Accordion components"
```

---

## Task 5: Update httpui data models and state

**Files:**

- Modify: `httpui/src/state/models.rs`
- Modify: `httpui/src/state/store.rs`

- [ ] **Step 1: Add new types to `models.rs`**

Add at the top of the file:

```rust
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum SideNavItem {
    Collections,
    History,
    Apis,
    MockServers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum TopBarNav {
    Collections,
    Environment,
    History,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum EditorTab {
    Params,
    Authorization,
    Headers,
    Body,
    Settings,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub time_ms: u64,
    pub size_bytes: u64,
}
```

Note: Named `HttpResponse` to avoid collision with any `Response` in scope.

- [ ] **Step 2: Update `Request` struct in `models.rs`**

Add three new fields:

```rust
pub struct Request {
    pub id: i32,
    pub collection_id: Option<i32>,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<KeyValue>,
    pub params: Vec<KeyValue>,
    pub body: Option<String>,
    pub inherit_cookies_header: bool,
    pub inherit_authorization_header: bool,
}
```

- [ ] **Step 3: Update `AppState` in `store.rs`**

Add new signal fields:

```rust
pub active_sidebar_nav: Signal<SideNavItem>,
pub active_topbar_nav: Signal<TopBarNav>,
pub active_editor_tab: Signal<EditorTab>,
pub http_response: Signal<Option<HttpResponse>>,  // NEW — used by Canvas
```

**Keep the existing `response: Signal<String>` field for now.** The old `response_viewer` and `urlbar` views still reference it and won't be deleted until Task 11. Removing it here would break compilation. The new `http_response` field is used by the Canvas view (Task 10). The old `response` field is removed in Task 11 when old views are deleted.

Update the `new()` constructor to accept and initialize the new fields alongside the existing ones.

- [ ] **Step 4: Update all call sites that create `Request` structs**

Search for `Request {` in httpui views (Library, Tabbar, etc.) and add the new fields with defaults: `headers: Vec::new(), params: Vec::new(), body: None`.

- [ ] **Step 5: Update `App` component in `main.rs`**

Add signal initialization for new state fields:

```rust
let active_sidebar_nav = use_signal(|| SideNavItem::Collections);
let active_topbar_nav = use_signal(|| TopBarNav::Collections);
let active_editor_tab = use_signal(|| EditorTab::Params);
let http_response = use_signal(|| None);
```

Pass these to `AppState::new()`. Keep the existing `response` signal — it's still used by old views until Task 11.

- [ ] **Step 6: Verify**

Run: `cargo check --package httpui`
Expected: Compiles (existing views may have warnings about unused fields but no errors).

- [ ] **Step 7: Commit**

```bash
git add httpui/src/state/ httpui/src/main.rs httpui/src/views/
git commit -m "feat(httpui): update data models with KeyValue, HttpResponse, nav enums"
```

---

## Task 6: Wire `kinetic_ui` into httpui + new layout grid

**Files:**

- Modify: `httpui/Cargo.toml`
- Modify: `httpui/src/main.rs`
- Modify: `httpui/src/views/layout_grid/component.rs`
- Modify: `httpui/src/views/layout_grid/style.css`

- [ ] **Step 1: Add `kinetic_ui` dependency to `httpui/Cargo.toml`**

Add: `kinetic_ui = { workspace = true }`

You can keep `dioxus-primitives` for now if any existing code still imports it directly, but the goal is to use it only through `kinetic_ui`.

- [ ] **Step 2: Update `httpui/src/views/layout_grid/style.css`**

Replace the current 6-area grid with:

```css
#layout-grid {
  display: grid;
  grid-template-columns: 64px 288px 1fr;
  grid-template-rows: 64px 1fr auto;
  grid-template-areas:
    "sidebar  explorer  topbar"
    "sidebar  explorer  canvas"
    "sidebar  explorer  statusbar";
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--k-surface);
}
```

- [ ] **Step 3: Update `httpui/src/views/layout_grid/component.rs`**

Replace the current `LayoutGrid` component to render the 5 grid areas. For now, use placeholder divs for new areas and keep `{children}` in the canvas area:

```rust
use dioxus::prelude::*;

#[component]
pub fn LayoutGrid(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { id: "layout-grid",
            div { style: "grid-area: sidebar; background: var(--k-surface-low);",
                "SideNav placeholder"
            }
            div { style: "grid-area: explorer; background: var(--k-surface-high);",
                "Explorer placeholder"
            }
            div { style: "grid-area: topbar; background: var(--k-surface-low);",
                "TopBar placeholder"
            }
            div { style: "grid-area: canvas;",
                {children}
            }
            div { style: "grid-area: statusbar; background: var(--k-surface-low); min-height: 24px;",
            }
        }
    }
}
```

- [ ] **Step 4: Update `httpui/src/main.rs`**

Add `use kinetic_ui::KineticTheme;` and wrap the app content:

```rust
rsx! {
    document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
    KineticTheme {
        LayoutGrid {
            Router::<Route> {}
        }
    }
}
```

Remove the old `dx-components-theme.css` link.

- [ ] **Step 5: Update the Route enum and clean up Navbar references**

Remove the `#[layout(Navbar)]` wrapper from the Route enum since Navbar is being replaced. The layout is now handled by `LayoutGrid`. Routes render into the canvas area. Keep all existing route variants — the old view components still exist and removing routes would orphan them.

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/request/:id")]
    RequestSection { id: i32 },
    #[route("/space/:id")]
    SpaceSection { id: i32 },
    #[route("/collection/:id")]
    CollectionSection { id: i32 },
    #[route("/settings")]
    SettingsSection {},
    #[route("/")]
    NewRequestSection {},
}
```

**Critical:** Also remove the `use views::navbar::Navbar;` import from `main.rs`. The `pedantic` + `nursery` clippy deny config will fail on unused imports. The `navbar` module declaration in `views/mod.rs` can stay until Task 11 cleanup (it won't error as long as it's not imported into scope where it's unused).

- [ ] **Step 6: Verify**

Run: `cargo check --package httpui`

Then: `dx serve --package httpui` — should show the new 3-column grid with placeholder text in each area and the router content in the canvas.

- [ ] **Step 7: Commit**

```bash
git add httpui/ Cargo.toml
git commit -m "feat(httpui): wire kinetic_ui + new 3-column layout grid"
```

---

## Task 7: Build SideNav view

**Files:**

- Create: `httpui/src/views/sidenav/{mod.rs,component.rs,style.css}`
- Modify: `httpui/src/views/mod.rs`
- Modify: `httpui/src/views/layout_grid/component.rs`

- [ ] **Step 1: Create `sidenav/style.css`**

```css
.sidenav {
  grid-area: sidebar;
  display: flex;
  flex-direction: column;
  background: var(--k-surface-low);
  padding: var(--k-space-2);
  height: 100%;
  overflow: hidden;
}

.sidenav__drag-region {
  -webkit-app-region: drag;
  height: 28px;
  flex-shrink: 0;
}

.sidenav__brand {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--k-space-1);
  padding: var(--k-space-2) 0;
  font-family: var(--k-font-display);
  font-size: 0.625rem;
  font-weight: 700;
  color: var(--k-on-surface);
  text-transform: uppercase;
  letter-spacing: 0.1em;
  text-align: center;
}

.sidenav__version {
  font-family: var(--k-font-mono);
  font-size: 0.5rem;
  color: var(--k-on-surface-variant);
  font-weight: 400;
}

.sidenav__nav {
  display: flex;
  flex-direction: column;
  gap: var(--k-space-1);
  flex: 1;
  padding: var(--k-space-2) 0;
}

.sidenav__nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: var(--k-space-2) var(--k-space-1);
  border-radius: var(--k-radius-default);
  cursor: pointer;
  color: var(--k-on-surface-variant);
  font-size: 0.5625rem;
  text-align: center;
  border: none;
  background: transparent;
  position: relative;
}

.sidenav__nav-item:hover {
  background: var(--k-surface-high);
}

.sidenav__nav-item[data-active="true"] {
  color: var(--k-primary);
  background: var(--k-surface-high);
}

.sidenav__nav-item[data-active="true"]::before {
  content: "";
  position: absolute;
  left: 0;
  top: 25%;
  bottom: 25%;
  width: 2px;
  background: var(--k-primary);
  border-radius: 1px;
}

.sidenav__footer {
  display: flex;
  flex-direction: column;
  gap: var(--k-space-1);
  padding-bottom: var(--k-space-2);
}
```

- [ ] **Step 2: Create `sidenav/component.rs`**

Use `use_context::<AppState>()` to read/write `active_sidebar_nav`. Render the brand with `env!("CARGO_PKG_NAME")` and `env!("CARGO_PKG_VERSION")`. Use `dioxus_free_icons` for nav icons (Material Design icons: folder for Collections, history for History, api for APIs, dns for Mock Servers, settings for Settings, help for Help). The "+ New Request" button uses the kinetic_ui Button with Primary variant.

Each nav item calls `active_sidebar_nav.set(SideNavItem::Variant)` on click. Data-active attribute bound to `active_sidebar_nav() == SideNavItem::Variant`.

- [ ] **Step 3: Create `sidenav/mod.rs`**

```rust
mod component;
pub use component::*;
```

- [ ] **Step 4: Update `views/mod.rs`**

Add `pub mod sidenav;`

- [ ] **Step 5: Replace placeholder in `layout_grid/component.rs`**

Import `SideNav` and replace the sidebar placeholder div with the actual component.

- [ ] **Step 6: Verify**

Run: `dx serve --package httpui` — SideNav should render with brand, nav icons, and clicking should highlight the active item.

- [ ] **Step 7: Commit**

```bash
git add httpui/src/views/sidenav/ httpui/src/views/mod.rs httpui/src/views/layout_grid/
git commit -m "feat(httpui): implement SideNav with brand, nav items, and active state"
```

---

## Task 8: Build Explorer view

**Files:**

- Create: `httpui/src/views/explorer/{mod.rs,component.rs,style.css}`
- Modify: `httpui/src/views/mod.rs`
- Modify: `httpui/src/views/layout_grid/component.rs`

- [ ] **Step 1: Create `explorer/style.css`**

Style the explorer panel: header with title + filter icon, scrollable content area. Background `--k-surface-high`. Header is uppercase, small font, `--k-on-surface-variant` color. Content area `overflow-y: auto`.

Request items in the tree: flex row with method badge, name, and truncated URL. Selected state via `data-selected` attribute with left border accent and `--k-surface-highest` background.

- [ ] **Step 2: Create `explorer/component.rs`**

Read `active_sidebar_nav` from context to determine what to show.

For `SideNavItem::Collections`: Read `collections` and `requests` from `AppState`. Use `KTreeBranch` for each collection and `KTreeLeaf` for each request inside it. Each request leaf shows a `Badge` with the HTTP method, the request name, and the truncated URL. Clicking a request sets `selected_request` and navigates to `/request/{id}`.

For other SideNavItem variants: Render a placeholder with "Coming soon" text.

The header title changes based on the active nav item (e.g., "COLLECTIONS", "HISTORY").

- [ ] **Step 3: Wire into layout_grid**

Import `Explorer` and replace the explorer placeholder.

- [ ] **Step 4: Verify**

Run: `dx serve --package httpui` — Explorer should show the collections tree (empty initially, but the structure renders). Creating collections/requests via existing mechanisms should populate the tree.

- [ ] **Step 5: Commit**

```bash
git add httpui/src/views/explorer/ httpui/src/views/mod.rs httpui/src/views/layout_grid/
git commit -m "feat(httpui): implement Explorer panel with collections tree view"
```

---

## Task 9: Build TopBar view

**Files:**

- Create: `httpui/src/views/topbar/{mod.rs,component.rs,style.css}`
- Modify: `httpui/src/views/mod.rs`
- Modify: `httpui/src/views/layout_grid/component.rs`

- [ ] **Step 1: Create `topbar/style.css`**

Horizontal flex layout. Background `--k-surface-low`. App name on left in Space Grotesk. Horizontal nav links in center. Search input + env selector + icons on right. Nav links: `--k-on-surface-variant` default, `--k-primary` when active with bottom border indicator. Height 64px, `align-items: center`, `padding: 0 --k-space-4`.

- [ ] **Step 2: Create `topbar/component.rs`**

Read `active_topbar_nav` from context. Render:

- App name from `env!("CARGO_PKG_NAME")` with `.k-headline-md` styling (but smaller, ~1rem)
- Three nav links (Collections, Environment, History) — clicking sets `active_topbar_nav`
- `SearchInput` component (non-functional placeholder — `oninput` is a no-op)
- Environment selector — use `KSelect` with hardcoded options (Dev, Staging, Production) as in the current Tabbar
- Settings icon button
- Account icon button (placeholder)

- [ ] **Step 3: Wire into layout_grid**

Replace the topbar placeholder.

- [ ] **Step 4: Verify**

Run: `dx serve --package httpui`

- [ ] **Step 5: Commit**

```bash
git add httpui/src/views/topbar/ httpui/src/views/mod.rs httpui/src/views/layout_grid/
git commit -m "feat(httpui): implement TopBar with nav, search, and environment selector"
```

---

## Task 10: Build Canvas view (URL bar + editor tabs + response)

**Files:**

- Create: `httpui/src/views/canvas/{mod.rs,component.rs,style.css}`
- Modify: `httpui/src/views/mod.rs`
- Modify: `httpui/src/views/layout_grid/component.rs`
- Modify: `httpui/src/main.rs` (Route adjustments)

- [ ] **Step 1: Create `canvas/style.css`**

Vertical flex layout filling the canvas grid area. URL bar section: flex row with method select, URL input, send button. Editor tabs below. Tab content area with `overflow-y: auto`. Response section at bottom with collapsible header.

Response status badge colors: use `data-status` attribute — `[data-status="2xx"]` green/secondary, `[data-status="4xx"]` primary, `[data-status="5xx"]` error. Response body: `background: var(--k-surface-lowest)`, monospace font, `overflow: auto`, `white-space: pre`.

- [ ] **Step 2: Create `canvas/component.rs`**

This is the largest component. It reads `selected_request`, `active_editor_tab`, and `response` from context.

**URL Bar section:**

- `KSelect` for HTTP method (GET, POST, PUT, PATCH, DELETE, OPTIONS, HEAD) — use method color for the trigger text
- `Input` (monospace) for URL
- `Button` (Primary variant) for "Send →" — onclick dispatches the HTTP request using `reqwest` (port the logic from current `urlbar/component.rs`)

**When sending a request:**

- Capture start time with `web_time::Instant` or `std::time::Instant`
- Build the `reqwest` request including params from the `params` Vec (append as query string)
- Include custom headers from the `headers` Vec
- Include body if present
- On response, populate `HttpResponse` with status, status_text, body, headers, time_ms, size_bytes
- Set `app_state.response` signal

**Editor Tabs:**

- Use `KTabs` with `KTabList` + `KTabTrigger` for each `EditorTab` variant
- The Headers tab trigger shows a badge with `headers.len()` count
- `KTabContent` renders based on `active_editor_tab`:
  - **Params**: `KTable` with Key/Value/Description columns, rows from `request.params`, inline editing updates the signal, "+ Add" row at bottom
  - **Headers**: Same table structure using `request.headers`
  - **Authorization**: Placeholder div with "Authorization configuration coming soon"
  - **Body**: Placeholder div with "Request body editor coming soon"
  - **Settings**: Placeholder div with "Request settings coming soon"

**Response Section:**

- Collapsible via a local signal `response_expanded: Signal<bool>`
- Header row shows: collapse chevron, "Response" text, status badge (`Badge` component), response time, response size
- Body area: `<pre>` with monospace, line numbers (enumerate lines), `--k-surface-lowest` background
- If no response yet, show muted "No response yet — send a request" placeholder

**When no request is selected** (e.g., at `/`):

- Show a centered placeholder: "Select or create a request to get started"

- [ ] **Step 3: Move Canvas outside the router**

The Canvas component is rendered directly in the layout grid — it reads `selected_request` from state, not from the route. It is **not** a routed component.

**Do NOT change the Route enum yet.** The existing routes (`RequestSection`, `SpaceSection`, `CollectionSection`, `SettingsSection`, `NewRequestSection`) and their view modules still exist. They will be cleaned up in Task 11. For now, the Router still renders in the layout grid, but the Canvas is placed alongside it (the Canvas shows when a request is selected, the Router handles non-request routes like `/settings`).

In `layout_grid/component.rs`, render both:

```rust
div { style: "grid-area: canvas;",
    Canvas {}
}
```

The Canvas component checks `selected_request` — if `Some`, it renders the request editor. If `None`, it shows the placeholder. The Router content (settings, space, collection stubs) can be rendered conditionally or kept as-is until Task 11.

- [ ] **Step 4: Wire Canvas into layout_grid**

Replace the canvas area placeholder with the Canvas component. The Canvas reads all state from context and renders accordingly.

- [ ] **Step 5: Verify**

Run: `dx serve --package httpui` — The full layout should now be visible. Creating a request, selecting it in the Explorer, editing URL/method, and clicking Send should work. Response should display with status, timing, and body.

- [ ] **Step 6: Commit**

```bash
git add httpui/src/views/canvas/ httpui/src/views/mod.rs httpui/src/views/layout_grid/ httpui/src/main.rs
git commit -m "feat(httpui): implement Canvas with URL bar, editor tabs, and response viewer"
```

---

## Task 11: Build StatusBar + clean up old views

**Files:**

- Create: `httpui/src/views/statusbar/{mod.rs,component.rs,style.css}`
- Delete: `httpui/src/views/navbar/`
- Delete: `httpui/src/views/library/`
- Delete: `httpui/src/views/tabbar/`
- Delete: `httpui/src/views/urlbar/`
- Delete: `httpui/src/views/request_editor/`
- Delete: `httpui/src/views/response_viewer/`
- Delete: `httpui/src/views/request/`
- Delete: `httpui/src/components/` (entire directory)
- Delete: `httpui/assets/dx-components-theme.css`
- Modify: `httpui/src/views/mod.rs`
- Modify: `httpui/src/main.rs`

- [ ] **Step 1: Create StatusBar stub**

`style.css`: `.statusbar` — `grid-area: statusbar`, `background: var(--k-surface-low)`, `height: 24px`, `display: flex`, `align-items: center`, `padding: 0 --k-space-3`, `font-size: 0.6875rem`, `color: var(--k-on-surface-variant)`.

`component.rs`: Renders a thin bar with placeholder text "Ready" (or empty). This is intentionally minimal — future work fills it in.

- [ ] **Step 2: Wire StatusBar into layout_grid**

- [ ] **Step 3: Delete old views**

Remove all old view directories that are now replaced:

- `httpui/src/views/navbar/` → replaced by `sidenav/`
- `httpui/src/views/library/` → replaced by `explorer/`
- `httpui/src/views/tabbar/` → replaced by `topbar/`
- `httpui/src/views/urlbar/` → merged into `canvas/`
- `httpui/src/views/request_editor/` → merged into `canvas/`
- `httpui/src/views/response_viewer/` → merged into `canvas/`
- `httpui/src/views/request/` → replaced by canvas signal-driven approach

- [ ] **Step 4: Delete old components directory**

Remove `httpui/src/components/` entirely — all components now live in `kinetic_ui`. Remove `mod components;` from `main.rs`.

**Note:** Check if any remaining view uses `icon_select_trigger` (from old components). The Tabbar used it for the HTTP method selector — since Tabbar is being deleted, this should be safe. But grep for `icon_select_trigger` imports first.

- [ ] **Step 5: Delete old theme CSS**

Remove `httpui/assets/dx-components-theme.css`. Remove its `document::Link` from `main.rs` if not already done.

- [ ] **Step 6: Remove old `response: Signal<String>` from AppState**

Now that all old views referencing the string-based `response` are deleted, remove the `response: Signal<String>` field from `AppState` in `store.rs` and its initialization in `main.rs`. The `http_response: Signal<Option<HttpResponse>>` field (added in Task 5) is the replacement.

- [ ] **Step 7: Simplify Route enum**

Now that old view modules are deleted, simplify the Route enum to only include active routes:

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/settings")]
    SettingsSection {},
    #[route("/")]
    Home {},
}
```

The Canvas is signal-driven (not routed). Space and Collection views were stubs — delete `httpui/src/views/space/` and `httpui/src/views/collection/` along with the routes.

- [ ] **Step 8: Update `views/mod.rs`**

Should now only contain:

```rust
pub mod canvas;
pub mod explorer;
pub mod layout_grid;
pub mod settings;
pub mod sidenav;
pub mod statusbar;
pub mod topbar;
```

- [ ] **Step 9: Update `main.rs` imports**

Remove all imports of deleted views. Ensure only the new views are imported.

- [ ] **Step 10: Verify**

Run: `cargo check --package httpui` — should compile with no errors.
Run: `cargo clippy --package httpui` — should pass with no warnings.
Run: `dx serve --package httpui` — full visual verification of the new layout.

- [ ] **Step 11: Commit**

```bash
git add -A httpui/
git commit -m "feat(httpui): remove old views/components, add StatusBar, complete layout migration"
```

---

## Task 12: Visual polish pass

**Files:**

- Various CSS files in `kinetic_ui/src/` and `httpui/src/views/`

- [ ] **Step 1: Run `dx serve --package httpui` and compare against Stitch mockups**

Open both mockup screenshots side-by-side:

- `httpui/docs/stitch/request_editor/screen.png`
- `httpui/docs/stitch/collections_environments/screen.png`

Check each area against the mockup for: spacing, colors, font sizes, alignment, hover states, active states.

- [ ] **Step 2: Fix spacing and alignment issues**

Adjust padding, gaps, and margins to match the mockup's visual density. The mockups use generous whitespace — ensure the Explorer items have enough padding, the TopBar elements are well-spaced, and the Canvas sections breathe.

- [ ] **Step 3: Fix color accuracy**

Verify all surface tokens are applied correctly — no areas using wrong surface tier. Check that text colors match (primary text vs muted text). Verify gradient button renders correctly.

- [ ] **Step 4: Fix typography**

Ensure Space Grotesk loads for headlines/brand. Ensure JetBrains Mono loads for all technical data (URLs, methods, response body). Ensure Inter is the default body font.

- [ ] **Step 5: Verify transitions and hover states**

All interactive elements should have smooth transitions (100-200ms). Buttons, nav items, tree nodes should respond to hover. Selected states should be visually distinct.

- [ ] **Step 6: Run clippy**

Run: `cargo clippy --package httpui --package kinetic_ui`
Fix any warnings.

- [ ] **Step 7: Commit**

```bash
git add kinetic_ui/ httpui/
git commit -m "style: visual polish pass — spacing, colors, typography, hover states"
```

---

## Task 13: Seed data + final verification

**Files:**

- Modify: `httpui/src/main.rs`

- [ ] **Step 1: Add seed data for visual testing**

In the `App` component, initialize the signals with sample data so the app looks populated on launch. This helps verify the full UI:

```rust
let spaces = use_signal(|| vec![
    Space { id: 1, name: "Default".into(), icon: None, color: None, environments: vec![], variables: vec![] },
]);
let collections = use_signal(|| vec![
    Collection { id: 1, space_id: 1, name: "User Auth API".into(), icon: None, color: None },
    Collection { id: 2, space_id: 1, name: "Billing Service".into(), icon: None, color: None },
]);
let requests = use_signal(|| vec![
    Request { id: 1, collection_id: Some(1), name: "Login User".into(), method: "POST".into(), url: "https://api.example.com/v1/auth/login".into(), headers: vec![], params: vec![], body: None, inherit_cookies_header: false, inherit_authorization_header: false },
    Request { id: 2, collection_id: Some(1), name: "Get User Profile".into(), method: "GET".into(), url: "https://api.example.com/v1/users/me".into(), headers: vec![], params: vec![KeyValue { id: 1, key: "api_version".into(), value: "2024-09-01".into(), description: "API version".into(), enabled: true }], body: None, inherit_cookies_header: false, inherit_authorization_header: false },
    Request { id: 3, collection_id: Some(2), name: "List Invoices".into(), method: "GET".into(), url: "https://api.example.com/v1/billing/invoices".into(), headers: vec![], params: vec![], body: None, inherit_cookies_header: false, inherit_authorization_header: false },
]);
let next_request_id = use_signal(|| 4);
let next_collection_id = use_signal(|| 3);
```

- [ ] **Step 2: Full visual verification**

Run: `dx serve --package httpui`

Verify:

- SideNav shows brand + version + nav items
- Explorer shows "User Auth API" and "Billing Service" collections with requests inside
- Clicking a request highlights it in Explorer and shows the editor in Canvas
- URL bar, editor tabs, and response section all render correctly
- TopBar shows app name, nav links, search, and env selector
- Colors, fonts, and spacing match the Kinetic Obsidian design system
- macOS titlebar drag region works

- [ ] **Step 3: Commit**

```bash
git add httpui/src/main.rs
git commit -m "feat(httpui): add seed data for visual testing of new layout"
```

---

## Summary

| Task | Description                                       | Key Files                                                             |
| ---- | ------------------------------------------------- | --------------------------------------------------------------------- |
| 1    | kinetic_ui crate scaffold + theme CSS             | `kinetic_ui/` scaffold, theme files                                   |
| 2    | Core components: Button, Input, Badge, IconButton | `kinetic_ui/src/components/{button,input,badge,icon_button}/`         |
| 3    | Tabs, Select, Separator, Tooltip                  | `kinetic_ui/src/components/{tabs,select,separator,tooltip}/`          |
| 4    | Table, SearchInput, TreeView, Accordion           | `kinetic_ui/src/components/{table,search_input,tree_view,accordion}/` |
| 5    | Data model + state updates                        | `httpui/src/state/{models,store}.rs`                                  |
| 6    | Wire kinetic_ui + new layout grid                 | `httpui/Cargo.toml`, layout_grid                                      |
| 7    | SideNav view                                      | `httpui/src/views/sidenav/`                                           |
| 8    | Explorer view                                     | `httpui/src/views/explorer/`                                          |
| 9    | TopBar view                                       | `httpui/src/views/topbar/`                                            |
| 10   | Canvas view (request editor + response)           | `httpui/src/views/canvas/`                                            |
| 11   | StatusBar + delete old views                      | Cleanup pass                                                          |
| 12   | Visual polish                                     | CSS adjustments                                                       |
| 13   | Seed data + final verification                    | `main.rs` seed data                                                   |

Tasks 1-4 are `kinetic_ui` library work (can be parallelized somewhat). Tasks 5-6 are the bridge. Tasks 7-11 are httpui view rebuilding (sequential). Tasks 12-13 are polish.
