# httpui Kinetic UI Redesign — Design Specification

## 1. Overview

This spec covers two intertwined efforts:

1. **`kinetic_ui`** — A new shared Dioxus component library at the workspace root implementing the "Kinetic Obsidian" design system from `DESIGN.md`. Built on `dioxus-primitives` with BEM-namespaced CSS and CSS custom property tokens. Reusable across all workspace projects.

2. **httpui layout & design overhaul** — Restructure httpui from its current 6-area grid layout to the 3-column layout shown in the Stitch mockups (`httpui/docs/stitch/`). Adopt the Kinetic Obsidian design system throughout. Add placeholder UI for unimplemented features to make future work visible.

Feature implementation (persistence, auth flows, scripting, etc.) is out of scope — this spec covers the design system, layout, and component architecture only. Data model additions required by the new UI (e.g., headers, params, response metadata) are in scope.

---

## 2. Migration Approach: Foundation-Up

1. Create `kinetic_ui` with the **theme layer** (CSS tokens, typography, utilities) and core primitives (Button, Input)
2. Rebuild httpui's **layout shell** (SideNav, Explorer, TopBar, Canvas) using the theme
3. Migrate/build remaining components into `kinetic_ui` as each view section demands them

This front-loads decisions that affect everything (tokens, crate structure) while letting component details emerge from real usage.

---

## 3. `kinetic_ui` Crate Structure

```
kinetic_ui/
  Cargo.toml
  src/
    lib.rs                       # re-exports all public API
    theme/
      mod.rs                     # KineticTheme component (loads all CSS)
      kinetic-theme.css          # design tokens (CSS custom properties)
      typography.css             # font imports + type scale classes
      utilities.css              # BEM utility classes (k-surface-*, k-text-*)
    components/
      mod.rs
      button/                    # component.rs + style.css
      input/
      select/
      accordion/
      tree_view/                 # NEW — for collection hierarchy
      tabs/                      # NEW — for editor tabs (Params/Headers/Body)
      table/                     # NEW — for key-value editors
      badge/                     # NEW — for method badges, status codes
      separator/
      icon_button/
      search_input/              # NEW — for top bar search
      tooltip/                   # NEW
```

### 3.1 Dependencies

```toml
[dependencies]
dioxus = { workspace = true }
dioxus-primitives = { workspace = true }
dioxus-free-icons = { workspace = true }
strum = { workspace = true }
```

`httpui` adds `kinetic_ui` as a path dependency and drops its own component implementations.

### 3.2 CSS Architecture

**Scoping strategy:** Global CSS loaded via `asset!()` + BEM-namespaced classes to prevent collisions.

- **`--k-` prefix** for all CSS custom properties
- **`k-` prefix** for all BEM classes (e.g., `.k-button`, `.k-button--primary`, `.k-select__trigger`)
- Three theme files loaded by a `KineticTheme` wrapper component:
  - `kinetic-theme.css` — design tokens
  - `typography.css` — font imports and type scale
  - `utilities.css` — reusable utility classes

### 3.3 Component Patterns

Each component:

- Wraps a `dioxus-primitives` headless primitive where one exists
- Has its own `component.rs` (Rust) + `style.css` (BEM-scoped CSS)
- Loads its CSS via `document::Link` in its render function
- Exposes props that are `Clone + PartialEq` with owned types

---

## 4. Design Tokens

### Token Naming Convention

The spec uses shortened `--k-` prefixed names that map to the Material Design token names in `DESIGN.md`. The mapping is:

| DESIGN.md Name              | Spec Token              | Notes                           |
| --------------------------- | ----------------------- | ------------------------------- |
| `surface-container-lowest`  | `--k-surface-lowest`    | "container" dropped for brevity |
| `surface`                   | `--k-surface`           | Same                            |
| `surface-container-low`     | `--k-surface-low`       |                                 |
| `surface-container`         | `--k-surface-container` | Middle tier                     |
| `surface-container-high`    | `--k-surface-high`      |                                 |
| `surface-container-highest` | `--k-surface-highest`   |                                 |

When consulting `DESIGN.md`, use this mapping. The spec tokens are authoritative for implementation.

### 4.1 Surfaces

| Token                   | Hex                      | Usage                                    |
| ----------------------- | ------------------------ | ---------------------------------------- |
| `--k-surface-lowest`    | `#0E0E0E`                | Inset fields, deepest recesses           |
| `--k-surface`           | `#131313`                | Base canvas background                   |
| `--k-surface-low`       | `#1B1C1C`                | Sectioning, sidebar backgrounds          |
| `--k-surface-container` | `#1F2020`                | Middle tier, subtle separation           |
| `--k-surface-high`      | `#2A2A2A`                | Explorer panel, elevated sections        |
| `--k-surface-highest`   | `#353535`                | Cards, buttons, highest elevation        |
| `--k-glass-surface`     | `rgba(227,225,224,0.08)` | Glassmorphism base for floating elements |

### 4.2 Brand Colors

| Token                      | Hex       | Usage                                               |
| -------------------------- | --------- | --------------------------------------------------- |
| `--k-primary`              | `#FFB3AD` | Primary actions, active indicators, links           |
| `--k-primary-container`    | `#FF5451` | Gradient endpoints, badges                          |
| `--k-on-primary-container` | `#5C0008` | Text on primary containers/gradient buttons         |
| `--k-secondary`            | `#44E2CD` | Success states, GET method badges, tech accents     |
| `--k-secondary-container`  | `#03C6B2` | Muted secondary backgrounds                         |
| `--k-tertiary`             | `#F9BD22` | Warnings, DELETE method badges, utility accents     |
| `--k-error`                | `#FFB4AB` | Error states, 5xx status codes                      |
| `--k-on-surface`           | `#E4E2E1` | Primary text on dark surfaces                       |
| `--k-on-surface-variant`   | `#E4BEBA` | Secondary/muted text (warm tone)                    |
| `--k-outline-variant`      | `#5B403E` | Ghost borders — apply opacity contextually (10-40%) |

**Note on `--k-outline-variant`:** Stored at full opacity. Apply CSS opacity per context: 20% for default ghost borders, 10% for subtle dividers, 40% for focus rings. Example: `border: 1px solid rgba(91, 64, 62, 0.2)` or use `border-color: var(--k-outline-variant); opacity: 0.2`.

### 4.3 Typography

| Token              | Value                         | Usage                 |
| ------------------ | ----------------------------- | --------------------- |
| `--k-font-display` | `'Space Grotesk', sans-serif` | Headlines, branding   |
| `--k-font-body`    | `'Inter', sans-serif`         | UI text, body copy    |
| `--k-font-mono`    | `'JetBrains Mono', monospace` | Code, URLs, HTTP data |

**Note:** The `collections_environments` mockup uses Fira Code; JetBrains Mono is the canonical choice per `request_editor` mockup and this spec. Mockup font inconsistency is not authoritative.

**Type Scale:**

| Class            | Size      | Font           | Usage                             |
| ---------------- | --------- | -------------- | --------------------------------- |
| `.k-display-lg`  | 3.5rem    | Space Grotesk  | Hero headers (not used in httpui) |
| `.k-headline-md` | 1.75rem   | Space Grotesk  | Section titles                    |
| `.k-title-md`    | 1.125rem  | Inter          | Grouping titles                   |
| `.k-body-md`     | 0.875rem  | Inter          | Default text                      |
| `.k-label-md`    | 0.75rem   | JetBrains Mono | Technical data, monospace labels  |
| `.k-label-sm`    | 0.6875rem | Inter          | Small labels, badges              |

### 4.4 Spacing

| Token         | Value          |
| ------------- | -------------- |
| `--k-space-1` | 0.25rem (4px)  |
| `--k-space-2` | 0.5rem (8px)   |
| `--k-space-3` | 0.75rem (12px) |
| `--k-space-4` | 1rem (16px)    |
| `--k-space-6` | 1.5rem (24px)  |
| `--k-space-8` | 2rem (32px)    |

### 4.5 Rounding

| Token                | Value   | Usage                           |
| -------------------- | ------- | ------------------------------- |
| `--k-radius-sm`      | 0.25rem | Small elements, minor rounding  |
| `--k-radius-default` | 0.5rem  | Buttons, inputs (per DESIGN.md) |
| `--k-radius-lg`      | 1rem    | Cards, containers               |
| `--k-radius-xl`      | 1.5rem  | Album art (music app)           |
| `--k-radius-full`    | 9999px  | Chips, pills                    |

**Note:** The mockup Tailwind configs use `DEFAULT: 0.25rem` and `lg: 0.5rem` which differs from DESIGN.md. The spec tokens are authoritative — they match DESIGN.md Section 5 ("Buttons/Inputs: DEFAULT (0.5rem)", "Cards/Containers: lg (1rem)").

### 4.6 Shadows

Ambient shadow for floating elements:

```css
--k-shadow-float: 0 4px 40px rgba(228, 226, 225, 0.06);
```

No hard drop shadows. Shadow color sampled from `--k-on-surface`, not pure black.

---

## 5. httpui Layout Architecture

### 5.1 Grid Definition

Replace the current 6-area grid with a 3-column, 3-row CSS grid:

```css
grid-template-columns: 64px 288px 1fr;
grid-template-rows: 64px 1fr auto;
grid-template-areas:
  "sidebar  explorer  topbar"
  "sidebar  explorer  canvas"
  "sidebar  explorer  statusbar";
```

Full height viewport: `height: 100vh; overflow: hidden;`

**Note on sidebar width:** The mockups show a ~256px sidebar with text labels. This spec uses 64px (icon-only) to match the visual weight shown in the `request_editor` mockup where the sidebar is narrow and the text labels ("Collections", "History", etc.) are short enough to fit. The sidebar nav items use icon + short text label stacked vertically. The "+ New Request" button uses a compact icon-only or abbreviated form. If 64px proves too tight during implementation, widen to 80px — the grid value is the single place to change.

**Note on Explorer visibility:** The Explorer is a CSS grid area and is always visible. Collapse/toggle behavior is out of scope for this redesign but could be added later by toggling the grid column width. The 3-column grid is the correct approach — it matches both mockups where the explorer is always present.

### 5.2 Component Mapping

| Grid Area   | New Component | Replaces                                      | Surface Token  |
| ----------- | ------------- | --------------------------------------------- | -------------- |
| `sidebar`   | `SideNav`     | `Navbar`                                      | `surface-low`  |
| `explorer`  | `Explorer`    | `Library`                                     | `surface-high` |
| `topbar`    | `TopBar`      | `Tabbar` (partial)                            | `surface-low`  |
| `canvas`    | `Canvas`      | `Urlbar` + `RequestEditor` + `ResponseViewer` | `surface`      |
| `statusbar` | `StatusBar`   | —                                             | `surface-low`  |

---

## 6. View Components

### 6.1 SideNav (sidebar, 64px)

**Structure (top to bottom):**

1. **Drag region** — macOS titlebar (carried from current Navbar)
2. **Brand** — "HTTP CLIENT" text + version badge (`env!("CARGO_PKG_VERSION")`)
3. **Primary CTA** — "+ New Request" button (gradient, compact)
4. **Nav items** (vertical stack, icon + short label):
   - Collections (default active)
   - History
   - APIs
   - Mock Servers
5. **Footer items:**
   - Settings
   - Help

**Behavior:**

- Clicking a nav item sets the active SideNav item (signal state)
- Active item: left border accent (`--k-primary`) + `surface-high` background
- The active item determines what the Explorer panel shows
- Full height, spans all 3 grid rows

### 6.2 Explorer (explorer, 288px)

**Structure:**

1. **Header** — Title reflecting active SideNav item (e.g., "ACTIVE COLLECTION") + filter button
2. **Content** — varies by active SideNav item:
   - **Collections** (default): Tree view of collections → folders → requests
   - **History**: Placeholder list
   - **APIs**: Placeholder list
   - **Mock Servers**: Placeholder list

**Collections tree view:**

The `collections_environments` mockup is authoritative for the Explorer's tree hierarchy. The `request_editor` mockup shows a simplified flat list — treat it as a condensed rendering of the same data, not a different design.

- Collections are expandable root nodes
- Folders are expandable child nodes within collections
- Requests are leaf nodes displaying:
  - HTTP method badge (colored per Section 10)
  - Request name (truncated)
  - Endpoint URL (truncated, muted text)
  - Optional status badge (e.g., "200 OK")
- Selected request: left border accent + darker background
- Full height, spans all 3 grid rows

### 6.3 TopBar (topbar, 64px)

**Structure (left to right):**

1. **App name** — `env!("CARGO_PKG_NAME")` in Space Grotesk (will display "httpui" until renamed)
2. **Horizontal nav** — Collections, Environment, History (these switch the Canvas view)
3. **Search input** — with search icon, `surface-highest` background, rounded
4. **Environment selector** — dropdown with colored dot indicator
5. **Settings icon**
6. **Account icon** (placeholder)

**Surface:** `surface-low`, separated from Canvas by tonal shift (no border).

**Note on height:** The mockups use `h-16` (64px). The grid row is set to `64px` to match.

### 6.4 Canvas (canvas, flexible)

The Canvas is the main content area. It shows different views depending on navigation state.

**Request Editor mode (primary, when a request is selected):**

Vertical stack:

1. **URL Bar** — method dropdown (colored by method type) + URL input (monospace, `surface-lowest` inset) + gradient "Send →" button
2. **Editor Tabs** — Params | Authorization | Headers (count badge) | Body | Settings — active tab has `primary`-colored bottom border
3. **Tab Content** — context-dependent:
   - **Params**: Key/Value/Description table, inline editing, add/delete rows
   - **Authorization**: Placeholder (auth type selector + credential fields)
   - **Headers**: Key/Value table, same as Params
   - **Body**: Placeholder (raw text area, future: format selector)
   - **Settings**: Placeholder (request-level settings)
4. **Response Section** — collapsible:
   - Header: "Response" label + status badge (2xx=secondary, 4xx=primary, 5xx=error) + response time (ms) + response size (bytes)
   - Body: Syntax-highlighted JSON/text, line numbers, monospace, `surface-lowest` background

**Environment view mode** (when Environment nav selected in TopBar):

- Breadcrumb, title, description
- Variable table: checkbox, name, initial value, current value, type, delete
- Metrics cards (variable coverage, active references, last modified)
- Mostly placeholder/future work

**Collection view mode** (when collection selected):

- Collection details/overview — placeholder

### 6.5 StatusBar (statusbar, auto height)

Optional, low priority. Thin bar at the bottom showing connection status, branch, encoding. Render as an empty styled bar initially — future work fills it in.

---

## 7. Data Model Changes

### 7.1 New Structs

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct KeyValue {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub time_ms: u64,
    pub size_bytes: u64,
}
```

### 7.2 Modified Structs

```rust
pub struct Request {
    pub id: i32,
    pub collection_id: Option<i32>,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<KeyValue>,           // NEW
    pub params: Vec<KeyValue>,            // NEW
    pub body: Option<String>,             // NEW
    pub inherit_cookies_header: bool,
    pub inherit_authorization_header: bool,
}
```

### 7.3 Existing Model Disposition

- **`Space`**: Preserved but deprioritized. The Stitch mockups don't prominently feature spaces — collections are the primary organizational unit. The `Space` model remains in code, the space selector in the Explorer header can stay as a dropdown (as it currently is in Library), but it is not a focus of this redesign. No structural changes.
- **`Environment`** and **`Variable`**: Preserved as-is. The Environment view in the Canvas will use these models when that feature is built out. No changes needed now.
- **`Collection`**: Unchanged. Still has `space_id` foreign key.

### 7.4 New State Fields

```rust
// In AppState:
pub active_sidebar_nav: Signal<SideNavItem>,     // which sidebar icon is active
pub active_topbar_nav: Signal<TopBarNav>,        // which topbar nav is active
pub active_editor_tab: Signal<EditorTab>,        // which editor tab is active
pub response: Signal<Option<Response>>,          // replaces Signal<String>

// Enums:
pub enum SideNavItem { Collections, History, Apis, MockServers }
pub enum TopBarNav { Collections, Environment, History }
pub enum EditorTab { Params, Authorization, Headers, Body, Settings }
```

### 7.5 Open Requests & Tab Management

The current `AppState` has `open_requests: Signal<Vec<i32>>` and `selected_request: Signal<Option<i32>>` for multi-tab request editing. The Stitch mockups don't show a tabbar for multiple open requests — the Explorer serves as the request switcher.

For this redesign:

- **`selected_request`** is preserved — clicking a request in the Explorer sets it, and the Canvas shows that request's editor.
- **`open_requests`** is preserved but the visible tabbar is removed. The open requests list can be used internally to track unsaved state or restored later as a feature. This avoids data loss from the current multi-tab workflow while the UI focuses on the single-request-at-a-time pattern shown in the mockups.

---

## 8. Component Library (`kinetic_ui`) — Components

### 8.1 Migrated from httpui (restyled)

| Component     | Primitives Base               | Key Changes                                                        |
| ------------- | ----------------------------- | ------------------------------------------------------------------ |
| **Button**    | —                             | Gradient primary variant, ghost border secondary, BEM classes      |
| **Input**     | —                             | `surface-lowest` inset, ghost border focus ring, monospace variant |
| **Select**    | `dioxus-primitives` select    | Restyle trigger + dropdown to Kinetic Obsidian                     |
| **Accordion** | `dioxus-primitives` accordion | Restyle, used in Explorer for collection folders                   |
| **Separator** | —                             | Restyle to use spacing/tonal shift instead of lines where possible |

### 8.2 New Components

| Component       | Primitives Base          | Purpose                                                 |
| --------------- | ------------------------ | ------------------------------------------------------- |
| **TreeView**    | —                        | Nested expandable tree for collections/folders/requests |
| **Tabs**        | `dioxus-primitives` tabs | Horizontal tab bar with active indicator, count badges  |
| **Table**       | —                        | Inline-editable key-value table with add/delete rows    |
| **Badge**       | —                        | Colored pill for HTTP methods, status codes             |
| **IconButton**  | —                        | Icon-only button with tooltip, for SideNav and TopBar   |
| **SearchInput** | —                        | Input with search icon prefix, rounded                  |
| **Tooltip**     | —                        | Hover tooltip for icon buttons                          |

### 8.3 Theme Component

```rust
/// Wrapper component that loads all Kinetic Obsidian theme CSS.
/// Place at the root of the app.
#[component]
pub fn KineticTheme(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./theme/kinetic-theme.css") }
        document::Link { rel: "stylesheet", href: asset!("./theme/typography.css") }
        document::Link { rel: "stylesheet", href: asset!("./theme/utilities.css") }
        {children}
    }
}
```

**Note:** The existing httpui codebase uses `document::Link` for stylesheets. Dioxus 0.7 also supports `document::Stylesheet { href }`. This spec uses `document::Link` for consistency with existing code. Either works.

---

## 9. Design System Rules (from DESIGN.md)

These rules apply to all components and views:

1. **No 1px borders for sectioning** — use surface tonal shifts and spacing
2. **Ghost borders only for accessibility** — `--k-outline-variant` at contextual opacity (10-40%), for input focus and similar
3. **Gradient primary CTAs** — linear gradient from `--k-primary` to `--k-primary-container` at 135 degrees
4. **Glassmorphism for floating elements** — `--k-glass-surface` with 20-40px backdrop blur (modals, dropdowns)
5. **Ambient shadows only** — 40px blur, 6% opacity, sampled from `--k-on-surface`
6. **Rounding scale** — cards/containers: `lg` (1rem), buttons/inputs: `default` (0.5rem), chips: `full`
7. **Never use `#000000`** — deepest black is `--k-surface-lowest` (`#0E0E0E`)
8. **Color-blind safe** — amber (`--k-tertiary`) for warnings/destructive, blue-green (`--k-secondary`) for success (never red/green alone)
9. **Monospace for all technical data** — URLs, HTTP methods, response bodies, parameter keys

---

## 10. HTTP Method Color Mapping

| Method  | Color Token              | Hex       |
| ------- | ------------------------ | --------- |
| GET     | `--k-secondary`          | `#44E2CD` |
| POST    | `--k-primary`            | `#FFB3AD` |
| PUT     | `--k-tertiary`           | `#F9BD22` |
| PATCH   | `--k-tertiary`           | `#F9BD22` |
| DELETE  | `--k-tertiary`           | `#F9BD22` |
| OPTIONS | `--k-on-surface-variant` | `#E4BEBA` |
| HEAD    | `--k-on-surface-variant` | `#E4BEBA` |

**Note:** DELETE uses `--k-tertiary` (amber) rather than `--k-error` (red) per the color-blind accessibility rule (#8) and matching the Stitch mockup's `text-tertiary` treatment. This ensures destructive actions are distinguishable without relying on red alone.

---

## 11. Response Status Color Mapping

| Range | Color Token     | Example                                            |
| ----- | --------------- | -------------------------------------------------- |
| 2xx   | `--k-secondary` | 200 OK badge with `secondary-container` background |
| 3xx   | `--k-tertiary`  | 301 Redirect                                       |
| 4xx   | `--k-primary`   | 404 Not Found                                      |
| 5xx   | `--k-error`     | 500 Internal Server Error                          |

---

## 12. Out of Scope (Future Feature Work)

These are represented as placeholder UI elements but not functionally implemented:

- Request persistence (save/load from disk)
- Authentication flows (OAuth, Basic, API Key, Bearer)
- Request body editor (raw, form-data, JSON, XML, binary)
- Environment variable substitution in URLs/headers
- Request/response history tracking
- API documentation / Mock Server features
- Import/export (Postman, cURL, OpenAPI)
- Request scripting (pre-request/post-response)
- WebSocket support
- File upload
- Response timing waterfall
- Collection sharing
- Search functionality (in TopBar)
- Status bar content
- Explorer collapse/toggle
- Multiple open request tabs (data preserved internally, UI removed)
