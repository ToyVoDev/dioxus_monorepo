# dioxus_music Theme + Layout Shell Redesign — Design Specification

## 1. Overview

Sub-project 1 of 4 in the dioxus_music Kinetic Obsidian redesign. This spec covers:

1. **Adopt `kinetic_ui`** in `packages/ui` — replace the current custom CSS with Kinetic Obsidian design tokens and components
2. **Rebuild the layout shell** — new AppShell, Sidebar, Header, and PlayerBar matching the Stitch mockups
3. **Restyle domain components** — TrackList, QueuePanel, PlaylistFormModal get token swaps; APIs preserved except where explicitly noted
4. **Mobile layout** — bottom tab nav for the mobile crate, no sidebar

This spec does NOT cover view content (Library grid, Album detail, Now Playing, Downloads/Sync). Those are sub-projects 2-4.

---

## 2. Approach

**Rebuild layout shell, preserve domain components (Approach B):**
- AppShell and Sidebar rebuilt from scratch to match the new layout
- New Header component replaces Navbar
- PlayerBar restyled as floating glassmorphic overlay
- New MobileNav for mobile crate
- Domain components (TrackList, QueuePanel, PlaylistFormModal) keep their existing Rust APIs with CSS restyled using kinetic tokens. **Exception:** TrackList gains one new optional prop (`show_download_status`) — see §6.1.
- `packages/ui` gains `kinetic_ui` as a dependency

**Platform strategy (Approach C):**
- Web and desktop use the sidebar layout; CSS media query collapses sidebar at narrow widths
- Mobile crate uses its own MobileLayout with bottom tab nav, no sidebar
- Shared components in `packages/ui` are platform-agnostic

---

## 3. Dependencies

`packages/ui/Cargo.toml` adds:
```toml
kinetic_ui = { workspace = true }
```

The workspace root `Cargo.toml` already has `kinetic_ui = { path = "kinetic_ui" }` in `[workspace.dependencies]`.

---

## 4. Layout Architecture

### 4.1 Desktop/Web Layout

2-column CSS grid with a floating player bar. Sidebar width is 256px, matching the Stitch mockups (`w-64`).

```css
grid-template-columns: 256px 1fr;
grid-template-rows: 56px 1fr;
grid-template-areas:
  "sidebar  header"
  "sidebar  content";
height: 100vh;
overflow: hidden;
```

The PlayerBar is **not** a grid area. It is `position: fixed` with glassmorphism, floating above the content at the bottom.

Content area has `padding-bottom: 88px` to avoid content being hidden behind the player bar.

### 4.2 Mobile Layout

No grid — vertical flex column:

```
┌──────────────────┐
│  Content (flex:1) │
│                   │
├──────────────────┤
│  PlayerBar       │ ← floating, above bottom nav
├──────────────────┤
│  MobileNav       │ ← fixed bottom tab bar
└──────────────────┘
```

Content area has `padding-bottom: 128px` (player bar + bottom nav height).

### 4.3 Responsive Behavior (Web/Desktop only)

At viewport widths below `768px`, CSS hides the sidebar:
```css
@media (max-width: 768px) {
  .app-shell { grid-template-columns: 1fr; }
  .sidebar { display: none; }
  .player-bar { left: 16px; }  /* no sidebar offset */
}
```

The header remains visible and provides navigation when the sidebar is hidden. This is a simplified narrow-desktop mode, not the full mobile experience (which has bottom tab nav and is only in the mobile crate).

---

## 5. Component Specifications

### 5.1 AppShell (rebuild)

**Location:** `packages/ui/src/app_shell.rs`

**Props:** `children: Element` (the routed content)

**Note:** The current AppShell accepts `sidebar: Element` and other props from platform callers. The rebuilt version takes only `children`. All three platform crates (web, desktop, mobile) must update their `AppLayout`/`DesktopLayout`/`MobileLayout` call sites to pass only children content. The Sidebar, Header, and PlayerBar are now rendered internally by AppShell (desktop) or by the platform layout wrapper (mobile).

**Renders:**
- `KineticTheme` wrapper (loads all token CSS)
- Grid container with Sidebar + Header + content area
- QueuePanel (rendered inside the content area, toggled by PlayerState)
- PlayerBar (floating, outside the grid)

**Desktop structure:**
```
KineticTheme {
  div.app-shell {
    Sidebar {}
    Header {}
    main.app-shell__content {
      {children}
      QueuePanel {}
    }
  }
  PlayerBar {}
}
```

**Mobile structure (in mobile crate's layout wrapper):**
```
KineticTheme {
  div.app-shell--mobile {
    main.app-shell__content { {children} }
  }
  PlayerBar { compact: true }
  MobileNav {}
}
```

**Surface:** Grid container background `--k-surface`.

### 5.2 Sidebar (rebuild)

**Location:** `packages/ui/src/sidebar.rs`

**Props:** None. Reads the current route internally via Dioxus router hooks to determine the active nav item.

**Structure (top to bottom):**
1. **Brand** — "MONOLITH" in Space Grotesk, `--k-primary` color. Subtitle: "Offline-First" badge in `--k-on-surface-variant`, small monospace. This matches the Stitch mockup where "MONOLITH" is the sidebar brand.
2. **Nav items** (icon + label, horizontal per item):
   - Artists
   - Albums
   - Playlists
   - Downloads
3. **Spacer** (flex: 1)
4. **Footer** — "Core Engine v2.4" in muted monospace text

**Active state:** `--k-surface-highest` background + `--k-primary` text color. No left border (per Stitch mockup and no-line rule). Background fill only.

**Navigation:** Clicking a nav item uses Dioxus router navigation. The active item is determined by matching the current route.

**Surface:** `--k-surface-low` background. Width: 256px.

### 5.3 Header (new)

**Location:** `packages/ui/src/header.rs`

**Props:** None

**Structure (left to right):**
1. "KINETIC" brand text in Space Grotesk, `--k-primary` — this is the app brand, distinct from the "MONOLITH" engine brand in the sidebar
2. Spacer (flex: 1)
3. `KSearchInput` — placeholder "Search...", non-functional
4. Sync status dot — small colored circle (green = synced, amber = pending). Non-functional placeholder, defaults to green.
5. Settings `IconButton`
6. Account `IconButton` (placeholder)

**Note:** The header does NOT contain horizontal nav links. Navigation is in the sidebar only, matching both Stitch mockups where the header contains only brand + search + action icons. When the sidebar is hidden at narrow widths, the header brand "KINETIC" remains as the sole top-level branding.

**Surface:** `--k-surface-low`. Height: 56px.

### 5.4 PlayerBar (restyle)

**Location:** `packages/ui/src/player_bar.rs`

**Preserves:** All existing playback logic — hidden `<audio>` element, `document::eval()` JS interop, `onended` handling, shuffle/repeat state, queue navigation. The `PlayerState` context and its methods are untouched.

**New prop:** `compact: bool` (defaults to false). When true, renders the mobile/compact layout. The mobile crate passes `compact: true`; web/desktop use the default full layout.

**Visual changes only:**

**Full layout (default, desktop/web):**
- Album art thumbnail (40x40, `--k-radius-default` rounding) — placeholder colored div if no track
- Track title (`--k-on-surface`) + artist (`--k-on-surface-variant`), stacked
- Spacer
- Transport controls: shuffle icon, previous, play/pause (gradient circle — `--k-primary` to `--k-primary-container`), next, repeat icon
- Spacer
- Progress bar: gradient fill (`--k-primary` to `--k-primary-container`), time stamps in monospace
- Volume slider (hidden below 1024px width via CSS media query)

**Compact layout (`compact: true`, mobile crate):**
- Album art thumbnail (32x32)
- Track title (single line, truncated)
- Play/pause + next buttons only
- No progress bar, no volume

**Positioning — centered pill style matching album_detail mockup:**
- Desktop: `position: fixed; bottom: 16px; left: 50%; transform: translateX(-50%); max-width: 800px; width: calc(100% - 272px - 32px);` (centered, respecting sidebar width)
- Narrow desktop (below 768px): `width: calc(100% - 32px);` (no sidebar offset)
- Mobile: `position: fixed; bottom: 72px; left: 8px; right: 8px;` (above MobileNav)
- Background: `var(--k-glass-surface); backdrop-filter: blur(24px);`
- Rounding: `--k-radius-lg`
- Shadow: `--k-shadow-float`
- z-index: 500

### 5.5 MobileNav (new, mobile crate only)

**Location:** `packages/mobile/src/views/mobile_nav.rs`

Since this is only for the mobile crate, it lives in the mobile package.

**Structure:** Fixed bottom bar with 4 tab icons:
- Home (house icon)
- Search (search icon)
- Library (music note / book icon)
- Downloads (download icon)

**Active state:** `--k-primary` color on active tab icon + label.

**Surface:** `--k-surface-low`. Height: 56px. `position: fixed; bottom: 0;`. z-index: 400 (below PlayerBar's 500).

### 5.6 Navbar (removed)

The existing `packages/ui/src/navbar.rs` is deleted. Its functionality is replaced by the Header component.

---

## 6. Domain Component Restyle

These components keep their existing props and behavior. Only CSS changes, except where explicitly noted.

### 6.1 TrackList

**Current:** Div-based CSS grid layout (NOT an HTML `<table>`) with `.track-list__*` classes and blue accent (#6d85c6).

**Changes:**
- Replace all color values with kinetic tokens (`--k-on-surface`, `--k-on-surface-variant`, `--k-primary`)
- Row hover: `background: var(--k-surface-low)`
- Active track row: `border-left: 2px solid var(--k-primary)` + primary text color for title
- Duration: `font-family: var(--k-font-mono)`
- Track number: monospace, `--k-on-surface-variant`
- Header row: uppercase, `--k-on-surface-variant`, `font-size: 0.6875rem`, `letter-spacing: 0.05em`

**API change (exception to "preserve APIs"):** Add optional prop `show_download_status: Option<bool>` (defaults to None/false). When true, renders a rightmost column showing a download status icon per track. Non-functional for now — always shows a checkmark icon. This prepares the UI for the Downloads feature in sub-project 4.

### 6.2 QueuePanel

**Current:** Right-side sliding panel, dark background, drag-to-reorder.

**Changes:**
- Background: `--k-surface-high`
- Current track: primary left border removed (no-line rule) — instead use `--k-surface-highest` background + `--k-primary` text color for title
- Track title: `--k-on-surface`
- Track artist: `--k-on-surface-variant`
- Drag-over highlight: `--k-surface-highest`
- Panel header: Space Grotesk, track count in monospace badge
- Close button: `IconButton` from kinetic_ui

**QueuePanel placement:** Rendered inside `AppShell`'s content area (see §5.1 desktop structure). Its visibility is still toggled via `PlayerState.show_queue`.

### 6.3 PlaylistFormModal

**Current:** Modal overlay with form fields and genre chip UI.

**Changes:**
- Overlay: `var(--k-glass-surface)` + `backdrop-filter: blur(24px)`
- Modal card: `--k-surface-high`, `--k-radius-lg` rounding, `--k-shadow-float`
- Form inputs: kinetic_ui `Input` component
- Genre chips: kinetic_ui `Badge` component with `--k-radius-full`
- Save button: kinetic_ui `Button` Primary variant
- Cancel button: kinetic_ui `Button` Ghost variant
- Smart playlist chip colors: include genres get `BadgeVariant::Secondary`, exclude genres get `BadgeVariant::Error`

---

## 7. CSS Architecture

### 7.1 Remove old CSS files

Delete all files in `packages/ui/assets/styling/`:
- `app_shell.css`
- `sidebar.css`
- `player_bar.css`
- `track_list.css`
- `navbar.css`
- `playlist_form.css`
- `queue_panel.css`

### 7.2 New CSS files

Each restyled component gets a new CSS file using kinetic tokens and `k-` BEM naming:
- `packages/ui/assets/styling/app-shell.css`
- `packages/ui/assets/styling/sidebar.css`
- `packages/ui/assets/styling/header.css`
- `packages/ui/assets/styling/player-bar.css`
- `packages/ui/assets/styling/track-list.css`
- `packages/ui/assets/styling/queue-panel.css`
- `packages/ui/assets/styling/playlist-form.css`

Platform-specific CSS:
- `packages/mobile/assets/mobile-nav.css`
- `packages/web/assets/main.css` (updated)
- `packages/desktop/assets/main.css` (updated)

### 7.3 Theme loading

`AppShell` renders `KineticTheme` at the root, which loads the global token CSS. Individual components load their own CSS via `document::Link { rel: "stylesheet", href: asset!("...") }`.

---

## 8. Platform-Specific Changes

### 8.1 Web (`packages/web`)

- `AppLayout` uses the new `AppShell` — update call site to pass only `children` (remove `sidebar` prop)
- Routes remain: `/` (Library), `/playlist/:id` (PlaylistView)
- Future sub-projects will add: `/artists`, `/albums`, `/album/:id`, `/now-playing`, `/downloads`
- `main.css` updated: remove old resets/backgrounds, rely on kinetic theme

### 8.2 Desktop (`packages/desktop`)

- `DesktopLayout` uses the new `AppShell` — update call site to pass only `children`
- Currently placeholder — Home view says "coming soon"
- macOS transparent titlebar config preserved from current code (if present)

### 8.3 Mobile (`packages/mobile`)

- `MobileLayout` does NOT use `AppShell` — it builds its own layout: `KineticTheme` + content + `PlayerBar { compact: true }` + `MobileNav`
- Routes: same as web but rendered full-width
- Currently placeholder — will be built out in later sub-projects

---

## 9. Out of Scope

- View content (Library grid, Album detail, Now Playing, Downloads/Sync) — sub-projects 2-4
- "Go Online" toggle — automatic sync, no manual toggle
- Storage meter in sidebar
- "Downloaded Only" filter toggle
- "Ready for Flight" suggestion cards
- Actual search functionality (SearchInput is visual placeholder)
- Sync status backend (dot is visual placeholder)
- New route definitions (Artists, Albums views) — added in sub-project 2
- Audio playback logic changes — this is CSS/layout only
