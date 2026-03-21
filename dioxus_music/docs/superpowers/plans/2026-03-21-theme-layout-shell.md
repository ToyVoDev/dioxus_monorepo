# dioxus_music Theme + Layout Shell Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Adopt the `kinetic_ui` design system in dioxus_music's shared UI crate and rebuild the layout shell (AppShell, Sidebar, Header, PlayerBar) to match the Stitch mockups.

**Architecture:** `packages/ui` gains `kinetic_ui` as a dependency. AppShell rebuilt as 2-column grid (256px sidebar + content) with floating glassmorphic PlayerBar. New Header component replaces Navbar. Domain components (TrackList, QueuePanel, PlaylistFormModal) restyled with kinetic tokens. Mobile crate gets its own layout with MobileNav bottom tabs.

**Tech Stack:** Rust nightly, Dioxus 0.7, kinetic_ui (shared component library), CSS custom properties

**Spec:** `dioxus_music/docs/superpowers/specs/2026-03-21-theme-layout-shell-design.md`

**No tests currently exist in this workspace.** Verification uses `cargo check` and `dx serve` for visual confirmation.

**Reference files:**
- Current UI source: `dioxus_music/packages/ui/src/`
- Current UI CSS: `dioxus_music/packages/ui/assets/styling/`
- Stitch mockups: `dioxus_music/docs/stitch/library_overview/code.html`, `dioxus_music/docs/stitch/album_detail/code.html`
- kinetic_ui library: `kinetic_ui/src/`
- Dioxus 0.7 conventions: `AGENTS.md`

---

## File Structure

### New files

| File | Responsibility |
|---|---|
| `packages/ui/src/header.rs` | New Header component (brand, search, action icons) |
| `packages/ui/assets/styling/app-shell.css` | New AppShell grid layout CSS |
| `packages/ui/assets/styling/sidebar.css` | New Sidebar CSS (replaces old) |
| `packages/ui/assets/styling/header.css` | Header component CSS |
| `packages/ui/assets/styling/player-bar.css` | New floating glassmorphic PlayerBar CSS |
| `packages/ui/assets/styling/track-list.css` | Restyled TrackList CSS |
| `packages/ui/assets/styling/queue-panel.css` | Restyled QueuePanel CSS |
| `packages/ui/assets/styling/playlist-form.css` | Restyled PlaylistFormModal CSS |
| `packages/mobile/src/views/mobile_nav.rs` | MobileNav bottom tab component |
| `packages/mobile/assets/mobile-nav.css` | MobileNav CSS |

### Modified files

| File | Changes |
|---|---|
| `packages/ui/Cargo.toml` | Add `kinetic_ui` dependency |
| `packages/ui/src/lib.rs` | Add header module, remove navbar module |
| `packages/ui/src/app_shell.rs` | Rebuild: 2-col grid, KineticTheme, internal Sidebar/Header/PlayerBar |
| `packages/ui/src/sidebar.rs` | Rebuild: MONOLITH brand, icon nav, route-based active state |
| `packages/ui/src/player_bar.rs` | Restyle: floating glassmorphic, compact prop, gradient play button |
| `packages/ui/src/track_list.rs` | Restyle with kinetic tokens, add `show_download_status` prop |
| `packages/ui/src/queue_panel.rs` | Restyle with kinetic tokens |
| `packages/ui/src/playlist_form.rs` | Restyle with kinetic_ui components |
| `packages/web/src/main.rs` | Update AppLayout call site (remove sidebar prop) |
| `packages/desktop/src/main.rs` | Update DesktopLayout call site |
| `packages/mobile/src/main.rs` | Rebuild MobileLayout (no sidebar, MobileNav, compact PlayerBar) |
| `packages/mobile/src/views/mod.rs` | Add mobile_nav module |
| `packages/web/assets/main.css` | Remove old resets, rely on kinetic theme |
| `packages/desktop/assets/main.css` | Remove old resets, rely on kinetic theme |
| `packages/mobile/assets/main.css` | Remove old resets, rely on kinetic theme |

### Files to delete

| File | Reason |
|---|---|
| `packages/ui/src/navbar.rs` | Replaced by header.rs |
| `packages/ui/assets/styling/app_shell.css` | Replaced by app-shell.css |
| `packages/ui/assets/styling/sidebar.css` (old) | Replaced by new sidebar.css |
| `packages/ui/assets/styling/player_bar.css` | Replaced by player-bar.css |
| `packages/ui/assets/styling/track_list.css` | Replaced by track-list.css |
| `packages/ui/assets/styling/navbar.css` | Replaced by header.css |
| `packages/ui/assets/styling/playlist_form.css` | Replaced by playlist-form.css |
| `packages/ui/assets/styling/queue_panel.css` | Replaced by queue-panel.css |

---

## Task 1: Add kinetic_ui dependency + KineticTheme wrapper

**Files:**
- Modify: `packages/ui/Cargo.toml`
- Modify: `packages/ui/src/app_shell.rs`

- [ ] **Step 1: Add kinetic_ui to packages/ui/Cargo.toml**

Add `kinetic_ui = { workspace = true }` to the `[dependencies]` section.

- [ ] **Step 2: Wrap AppShell content with KineticTheme**

In `app_shell.rs`, add `use kinetic_ui::KineticTheme;` and wrap the existing content:

```rust
rsx! {
    document::Link { rel: "stylesheet", href: APP_SHELL_CSS }
    KineticTheme {
        div { class: "app-shell",
            // ... existing content unchanged
        }
    }
}
```

This makes kinetic tokens available to all child components immediately while keeping the current layout working.

- [ ] **Step 3: Verify**

Run: `cargo check --workspace`

- [ ] **Step 4: Commit**

```bash
git add packages/ui/Cargo.toml packages/ui/src/app_shell.rs
git commit -m "feat(dioxus_music): add kinetic_ui dependency and KineticTheme wrapper"
```

---

## Task 2: Create Header component + delete Navbar

**Files:**
- Create: `packages/ui/src/header.rs`
- Create: `packages/ui/assets/styling/header.css`
- Delete: `packages/ui/src/navbar.rs`
- Delete: `packages/ui/assets/styling/navbar.css`
- Modify: `packages/ui/src/lib.rs`

- [ ] **Step 1: Create `packages/ui/assets/styling/header.css`**

```css
.header {
    grid-area: header;
    display: flex;
    align-items: center;
    gap: var(--k-space-4);
    background: var(--k-surface-low);
    padding: 0 var(--k-space-4);
    height: 56px;
}

.header__brand {
    font-family: var(--k-font-display);
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--k-primary);
    letter-spacing: -0.01em;
}

.header__spacer { flex: 1; }

.header__actions {
    display: flex;
    align-items: center;
    gap: var(--k-space-2);
}

.header__sync-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--k-secondary);
}
```

- [ ] **Step 2: Create `packages/ui/src/header.rs`**

```rust
use dioxus::prelude::*;
use kinetic_ui::{IconButton, KSearchInput};

const HEADER_CSS: Asset = asset!("/assets/styling/header.css");

#[component]
pub fn Header() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        header { class: "header",
            span { class: "header__brand", "KINETIC" }
            div { class: "header__spacer" }
            div { class: "header__actions",
                KSearchInput { placeholder: "Search...".to_string() }
                div { class: "header__sync-dot", title: "Synced" }
                IconButton {
                    // Settings icon (gear SVG)
                    svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        circle { cx: "12", cy: "12", r: "3" }
                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                    }
                }
                IconButton {
                    // Account icon (user SVG)
                    svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                        path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                        circle { cx: "12", cy: "7", r: "4" }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 3: Update `packages/ui/src/lib.rs`**

Remove `mod navbar;` and `pub use navbar::Navbar;`. Add:
```rust
mod header;
pub use header::Header;
```

- [ ] **Step 4: Delete old navbar files**

Delete `packages/ui/src/navbar.rs` and `packages/ui/assets/styling/navbar.css`.

- [ ] **Step 5: Fix any compilation errors from Navbar removal**

Search for `Navbar` usage in `packages/web/src/views/` — the Library view may use `Navbar` as a wrapper. Replace with a plain `div` or remove the wrapper.

- [ ] **Step 6: Verify**

Run: `cargo check --workspace`

- [ ] **Step 7: Commit**

```bash
git add -A packages/ui/
git commit -m "feat(dioxus_music): add Header component, remove Navbar"
```

---

## Task 3: Rebuild Sidebar

**Files:**
- Rewrite: `packages/ui/src/sidebar.rs`
- Create: `packages/ui/assets/styling/sidebar.css` (new version, replacing old)

- [ ] **Step 1: Delete old sidebar CSS and write new**

Delete `packages/ui/assets/styling/sidebar.css` and create a new one with kinetic tokens:

Key CSS classes: `.sidebar` (grid-area: sidebar, 256px width, surface-low bg, flex column), `.sidebar__brand` (MONOLITH text, primary color, Space Grotesk), `.sidebar__subtitle` (muted mono), `.sidebar__nav` (flex column, gap), `.sidebar__nav-item` (horizontal flex, icon+label, hover surface-highest, transition), `.sidebar__nav-item[aria-current="page"]` (surface-highest bg, primary text, NO left border per spec — Dioxus router's `Link` sets `aria-current="page"` on active links), `.sidebar__footer` (muted mono, margin-top auto).

- [ ] **Step 2: Rewrite `packages/ui/src/sidebar.rs`**

The Sidebar no longer accepts `children`. It renders its own nav items.

**Props:** None. Reads current route internally.

**Note:** The Sidebar needs to know about routes, but routes are defined per platform crate. The Sidebar can't import `Route` from web/desktop/mobile. Instead, use a prop-based approach: accept `nav_items: Vec<SidebarNavItem>` where `SidebarNavItem { label, icon, href, active }`. Each platform crate constructs this Vec based on its routes.

Actually, simpler: accept `children: Element` like before but style the container differently. Platform crates provide the nav links. The Sidebar just provides the chrome (brand, layout, footer).

Revised approach:
```rust
#[component]
pub fn Sidebar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SIDEBAR_CSS }
        nav { class: "sidebar",
            div { class: "sidebar__brand",
                span { class: "sidebar__brand-name", "MONOLITH" }
                span { class: "sidebar__subtitle", "Offline-First" }
            }
            div { class: "sidebar__nav", {children} }
            div { class: "sidebar__footer",
                span { "Core Engine v2.4" }
            }
        }
    }
}
```

Platform crates provide styled `Link` elements as children, using a shared `.sidebar__nav-item` CSS class.

- [ ] **Step 3: Verify**

Run: `cargo check --workspace`

Note: Platform call sites (web/desktop `main.rs`) are updated in Task 4. Sidebar compiles now with the existing children pattern.

- [ ] **Step 4: Commit**

```bash
git add -A packages/ui/
git commit -m "feat(dioxus_music): rebuild Sidebar with MONOLITH branding and kinetic tokens"
```

---

## Task 4: Rebuild AppShell with new grid layout

**Files:**
- Rewrite: `packages/ui/src/app_shell.rs`
- Create: `packages/ui/assets/styling/app-shell.css` (new)
- Delete: `packages/ui/assets/styling/app_shell.css` (old)
- Modify: `packages/web/src/main.rs` (update AppLayout)
- Modify: `packages/desktop/src/main.rs` (update DesktopLayout)

- [ ] **Step 1: Write new `packages/ui/assets/styling/app-shell.css`**

```css
.app-shell {
    display: grid;
    grid-template-columns: 256px 1fr;
    grid-template-rows: 56px 1fr;
    grid-template-areas:
        "sidebar  header"
        "sidebar  content";
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background: var(--k-surface);
}

.app-shell__content {
    grid-area: content;
    overflow-y: auto;
    padding-bottom: 88px; /* space for floating player bar */
}

@media (max-width: 768px) {
    .app-shell {
        grid-template-columns: 1fr;
        grid-template-areas:
            "header"
            "content";
    }
    .app-shell .sidebar { display: none; }
}
```

- [ ] **Step 2: Rewrite `packages/ui/src/app_shell.rs`**

Keep `sidebar: Element` prop — platform crates provide Sidebar with route-specific children. AppShell now renders Header internally and uses the new grid:

```rust
use crate::header::Header;
use crate::player_bar::PlayerBar;
use crate::queue_panel::QueuePanel;
use dioxus::prelude::*;
use kinetic_ui::KineticTheme;

const APP_SHELL_CSS: Asset = asset!("/assets/styling/app-shell.css");

#[component]
pub fn AppShell(sidebar: Element, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_SHELL_CSS }
        KineticTheme {
            div { class: "app-shell",
                {sidebar}
                Header {}
                main { class: "app-shell__content",
                    {children}
                    QueuePanel {}
                }
            }
            PlayerBar {}
        }
    }
}
```

**Note:** Keep `sidebar: Element` prop for now — platform crates still provide Sidebar with their route-specific children. This preserves the current pattern while adding the new layout.

- [ ] **Step 3: Delete old CSS**

Delete `packages/ui/assets/styling/app_shell.css`.

- [ ] **Step 4: Update web AppLayout**

The web `AppLayout` already passes `sidebar: rsx! { Sidebar { ... } }`. Update the nav links inside the Sidebar to use `.sidebar__nav-item` CSS class for consistent styling. The Sidebar children should be `Link` elements with the new class. Example:

```rust
AppShell {
    sidebar: rsx! {
        Sidebar {
            Link { class: "sidebar__nav-item", to: Route::Library {}, "All Songs" }
            PlaylistSidebarSection {}
        }
    },
    Outlet::<Route> {}
}
```

Also remove any imports of `Navbar` if still present from Task 2.

- [ ] **Step 5: Update desktop DesktopLayout**

Same pattern — update Sidebar children with `.sidebar__nav-item` class.

- [ ] **Step 6: Verify**

Run: `cargo check --workspace`

- [ ] **Step 7: Commit**

```bash
git add -A packages/ui/ packages/web/ packages/desktop/
git commit -m "feat(dioxus_music): rebuild AppShell with 2-column grid layout"
```

---

## Task 5: Restyle PlayerBar as floating glassmorphic overlay

**Files:**
- Rewrite: `packages/ui/assets/styling/player-bar.css` (new, replacing `player_bar.css`)
- Modify: `packages/ui/src/player_bar.rs`
- Delete: `packages/ui/assets/styling/player_bar.css`

- [ ] **Step 1: Write new `packages/ui/assets/styling/player-bar.css`**

Floating glassmorphic positioning. Centered pill style. Key classes:
- `.player-bar` — `position: fixed`, `bottom: 16px`, `left: 50%`, `transform: translateX(-50%)`, `max-width: 800px`, `width: calc(100% - 288px)` (256px sidebar + 32px padding), `background: var(--k-glass-surface)`, `backdrop-filter: blur(24px)`, `border-radius: var(--k-radius-lg)`, `box-shadow: var(--k-shadow-float)`, `z-index: 500`, `display: flex`, `align-items: center`, `gap: var(--k-space-3)`, `padding: var(--k-space-2) var(--k-space-4)`
- `.player-bar__art` — 40x40, rounded, surface-high bg
- `.player-bar__info` — flex column, title in on-surface, artist in on-surface-variant
- `.player-bar__controls` — flex row, gap
- `.player-bar__btn` — kinetic styling, on-surface-variant color, hover on-surface
- `.player-bar__btn--play` — gradient circle (primary to primary-container), 36x36, rounded full
- `.player-bar__btn--active` — primary color
- `.player-bar__progress` — flex row, gradient bar
- Responsive: at 768px, `width: calc(100% - 32px)` (no sidebar offset)
- `.player-bar--compact` — smaller art (32x32), fewer controls, no progress bar

- [ ] **Step 2: Modify `packages/ui/src/player_bar.rs`**

Add `compact: Option<bool>` prop (defaults to None/false). Preserve ALL existing playback logic. Changes:
- Update CSS asset path to new file
- Add album art placeholder div
- Restyle transport buttons with new CSS classes
- Add progress bar section (placeholder — actual seek functionality is future work)
- When `compact` is true, render minimal layout (art + title + play/next only)
- Keep the hidden `<audio>` element and all JS interop unchanged

- [ ] **Step 3: Delete old CSS**

Delete `packages/ui/assets/styling/player_bar.css`.

- [ ] **Step 4: Verify**

Run: `cargo check --workspace`

- [ ] **Step 5: Commit**

```bash
git add -A packages/ui/
git commit -m "feat(dioxus_music): restyle PlayerBar as floating glassmorphic overlay"
```

---

## Task 6: Restyle TrackList with kinetic tokens

**Files:**
- Create: `packages/ui/assets/styling/track-list.css` (new)
- Modify: `packages/ui/src/track_list.rs`
- Delete: `packages/ui/assets/styling/track_list.css`

- [ ] **Step 1: Write new `packages/ui/assets/styling/track-list.css`**

Replace all hardcoded colors with kinetic tokens. Key rules:
- `.track-list__header` — uppercase, `--k-on-surface-variant`, `font-size: 0.6875rem`, `letter-spacing: 0.05em`, monospace
- `.track-list__row` — hover `background: var(--k-surface-low)`, transition 150ms
- `.track-list__row--active` — `border-left: 2px solid var(--k-primary)`, title color `--k-primary`
- `.track-list__col--num` — monospace, `--k-on-surface-variant`
- `.track-list__col--duration` — monospace
- `.track-list__col--status` — small icon column (16px width)

- [ ] **Step 2: Modify `packages/ui/src/track_list.rs`**

- Update CSS asset path
- Add optional `show_download_status: Option<bool>` prop
- When true, add a status column in the header and a checkmark icon per row
- Keep all existing click-to-play logic unchanged

- [ ] **Step 3: Delete old CSS**

Delete `packages/ui/assets/styling/track_list.css`.

- [ ] **Step 4: Verify**

Run: `cargo check --workspace`

- [ ] **Step 5: Commit**

```bash
git add -A packages/ui/
git commit -m "feat(dioxus_music): restyle TrackList with kinetic tokens and download status column"
```

---

## Task 7: Restyle QueuePanel + PlaylistFormModal

**Files:**
- Create: `packages/ui/assets/styling/queue-panel.css` (new)
- Create: `packages/ui/assets/styling/playlist-form.css` (new)
- Modify: `packages/ui/src/queue_panel.rs`
- Modify: `packages/ui/src/playlist_form.rs`
- Delete: `packages/ui/assets/styling/queue_panel.css`
- Delete: `packages/ui/assets/styling/playlist_form.css`

- [ ] **Step 1: Write new `packages/ui/assets/styling/queue-panel.css`**

Restyle with kinetic tokens: `--k-surface-high` background, current track `--k-surface-highest` bg + `--k-primary` title, drag-over highlight, Space Grotesk header, monospace track count badge.

- [ ] **Step 2: Modify `packages/ui/src/queue_panel.rs`**

Update CSS asset path. Replace `IconButton` usage (if any) with kinetic_ui `IconButton`. Keep drag-to-reorder logic unchanged.

- [ ] **Step 3: Write new `packages/ui/assets/styling/playlist-form.css`**

Restyle with kinetic tokens: glassmorphic overlay, `--k-surface-high` modal card, `--k-radius-lg` rounding, `--k-shadow-float`.

- [ ] **Step 4: Modify `packages/ui/src/playlist_form.rs`**

Update CSS asset path. Replace form elements with kinetic_ui components where applicable (Button, Input, Badge for genre chips). Keep all form logic unchanged.

- [ ] **Step 5: Delete old CSS files**

Delete `packages/ui/assets/styling/queue_panel.css` and `packages/ui/assets/styling/playlist_form.css`.

- [ ] **Step 6: Verify**

Run: `cargo check --workspace`

- [ ] **Step 7: Commit**

```bash
git add -A packages/ui/
git commit -m "feat(dioxus_music): restyle QueuePanel and PlaylistFormModal with kinetic tokens"
```

---

## Task 8: Build MobileLayout + MobileNav

**Files:**
- Create: `packages/mobile/src/views/mobile_nav.rs`
- Create: `packages/mobile/assets/mobile-nav.css`
- Modify: `packages/mobile/src/main.rs`
- Modify: `packages/mobile/src/views/mod.rs`

- [ ] **Step 1: Create `packages/mobile/assets/mobile-nav.css`**

```css
.mobile-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 56px;
    background: var(--k-surface-low);
    display: flex;
    justify-content: space-around;
    align-items: center;
    z-index: 400;
}

.mobile-nav__item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    color: var(--k-on-surface-variant);
    font-size: 0.625rem;
    cursor: pointer;
    border: none;
    background: transparent;
    padding: var(--k-space-1);
    transition: color 150ms ease;
}

.mobile-nav__item[data-active="true"] {
    color: var(--k-primary);
}
```

- [ ] **Step 2: Create `packages/mobile/src/views/mobile_nav.rs`**

Bottom tab bar with 4 items: Home, Search, Library, Downloads. Each is a button with an icon and label. Active state based on current route.

- [ ] **Step 3: Update `packages/mobile/src/views/mod.rs`**

Add `pub mod mobile_nav;`.

- [ ] **Step 4: Rebuild MobileLayout in `packages/mobile/src/main.rs`**

The MobileLayout no longer uses AppShell's sidebar. It renders:
```rust
KineticTheme {
    div { class: "app-shell--mobile",
        main { class: "app-shell__content--mobile",
            Outlet::<Route> {}
        }
    }
    PlayerBar { compact: true }
    MobileNav {}
}
```

Remove the `AppShell` and `Sidebar` imports. Add `kinetic_ui::KineticTheme` and the local `PlayerBar` import.

Add mobile-specific CSS for `.app-shell--mobile` (full width, padding-bottom: 128px).

- [ ] **Step 5: Verify**

Run: `cargo check --package dioxus_music_mobile` (or `cargo check --workspace`)

- [ ] **Step 6: Commit**

```bash
git add -A packages/mobile/
git commit -m "feat(dioxus_music): add MobileLayout with MobileNav bottom tabs"
```

---

## Task 9: Update platform main.css files + cleanup

**Files:**
- Modify: `packages/web/assets/main.css`
- Modify: `packages/desktop/assets/main.css`
- Modify: `packages/mobile/assets/main.css`

- [ ] **Step 1: Simplify platform CSS files**

Each platform's `main.css` should be minimal — just a body reset that defers to kinetic theme:

```css
/* Platform-specific overrides — kinetic theme handles most styling */
html, body {
    margin: 0;
    padding: 0;
    height: 100%;
    overflow: hidden;
}
```

Remove any old color values, background colors, font declarations — kinetic theme handles all of that.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --workspace`

Fix any warnings. Add `#![allow(clippy::volatile_composites)]` to `packages/ui/src/lib.rs` if needed for `asset!()` macro.

- [ ] **Step 3: Verify full build**

Run: `cargo check --workspace`

- [ ] **Step 4: Commit**

```bash
git add -A packages/
git commit -m "style(dioxus_music): simplify platform CSS, run clippy cleanup"
```

---

## Summary

| Task | Description | Key Files |
|---|---|---|
| 1 | Add kinetic_ui dependency + KineticTheme | `packages/ui/Cargo.toml`, `app_shell.rs` |
| 2 | Create Header, delete Navbar | `header.rs`, `header.css`, delete `navbar.rs` |
| 3 | Rebuild Sidebar | `sidebar.rs`, `sidebar.css` |
| 4 | Rebuild AppShell grid layout | `app_shell.rs`, `app-shell.css`, platform `main.rs` files |
| 5 | Restyle PlayerBar (floating glassmorphic) | `player_bar.rs`, `player-bar.css` |
| 6 | Restyle TrackList | `track_list.rs`, `track-list.css` |
| 7 | Restyle QueuePanel + PlaylistFormModal | `queue_panel.rs`, `playlist_form.rs`, CSS files |
| 8 | Build MobileLayout + MobileNav | `mobile/src/main.rs`, `mobile_nav.rs` |
| 9 | Platform CSS cleanup + clippy | Platform `main.css` files |

Tasks 1-4 are sequential (each builds on the previous). Tasks 5-7 can be done in any order after Task 4. Task 8 depends on Task 5 (needs PlayerBar `compact` prop). Task 9 is cleanup after everything else.

**Active nav state pattern:** Platform crates style their Sidebar `Link` children with `.sidebar__nav-item` CSS class. Dioxus router's `Link` component renders an `<a>` tag that gets `aria-current="page"` when the route matches. The CSS targets this: `.sidebar__nav-item[aria-current="page"]` for the active style. Same pattern for MobileNav in the mobile crate.
