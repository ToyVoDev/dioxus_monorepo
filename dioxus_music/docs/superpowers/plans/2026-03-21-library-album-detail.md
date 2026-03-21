# Library + Album Detail Views Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add album grid library view, album detail view, expanded routing, settings dropdown with rescan, and placeholder views for the dioxus_music web crate.

**Architecture:** Library derives `AlbumSummary` from track metadata client-side (no schema changes). New routes for Artists/Albums/Playlists/Downloads. Settings dropdown in shared Header replaces rescan button. Album detail shows asymmetric header + existing TrackList component.

**Tech Stack:** Rust nightly, Dioxus 0.7, kinetic_ui, dioxus_music_api (existing server functions)

**Spec:** `dioxus_music/docs/superpowers/specs/2026-03-21-library-album-detail-design.md`

**No tests.** Verification: `cargo check` and `dx serve --package dioxus_music_web`.

---

## File Structure

### New files

| File | Responsibility |
|---|---|
| `packages/web/src/views/album_detail.rs` | Album Detail view |
| `packages/web/src/views/artists.rs` | Artists placeholder view |
| `packages/web/src/views/downloads.rs` | Downloads placeholder view |
| `packages/web/src/views/playlists.rs` | Playlists full view (promotes sidebar content) |
| `packages/web/assets/library.css` | Library album grid CSS |
| `packages/web/assets/album_detail.css` | Album Detail CSS |
| `packages/ui/assets/styling/settings-dropdown.css` | Settings dropdown CSS |

### Modified files

| File | Changes |
|---|---|
| `packages/web/src/main.rs` | New routes, updated sidebar nav links |
| `packages/web/src/views/mod.rs` | New view modules |
| `packages/web/src/views/library.rs` | Rewrite: album grid + all-songs toggle |
| `packages/ui/src/header.rs` | Add settings dropdown |
| `packages/ui/src/lib.rs` | Export AlbumSummary + grouping function |

---

## Task 1: Add AlbumSummary model + grouping function

**Files:**
- Create: `packages/ui/src/album_utils.rs`
- Modify: `packages/ui/src/lib.rs`

- [ ] **Step 1: Create `packages/ui/src/album_utils.rs`**

```rust
use dioxus_music_api::models::TrackSummary;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct AlbumSummary {
    pub name: String,
    pub artist: String,
    pub genre: String,
    pub track_count: usize,
    pub total_duration_secs: i32,
}

pub fn group_tracks_into_albums(tracks: &[TrackSummary]) -> Vec<AlbumSummary> {
    let mut groups: BTreeMap<String, Vec<&TrackSummary>> = BTreeMap::new();
    for track in tracks {
        groups.entry(track.album.clone()).or_default().push(track);
    }

    groups
        .into_iter()
        .map(|(album_name, album_tracks)| {
            // Determine artist: if all same, use it; otherwise "Various Artists"
            let first_artist = &album_tracks[0].artist;
            let artist = if album_tracks.iter().all(|t| t.artist == *first_artist) {
                first_artist.clone()
            } else {
                "Various Artists".to_string()
            };
            let genre = album_tracks[0].genre.clone();
            let track_count = album_tracks.len();
            let total_duration_secs: i32 = album_tracks.iter().map(|t| t.duration_secs).sum();

            AlbumSummary {
                name: album_name,
                artist,
                genre,
                track_count,
                total_duration_secs,
            }
        })
        .collect()
}
```

- [ ] **Step 2: Update `packages/ui/src/lib.rs`**

Add:
```rust
mod album_utils;
pub use album_utils::{AlbumSummary, group_tracks_into_albums};
```

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_ui`

- [ ] **Step 4: Commit**

```bash
git add packages/ui/src/album_utils.rs packages/ui/src/lib.rs
git commit -m "feat(dioxus_music): add AlbumSummary model and track grouping utility"
```

---

## Task 2: Expand routes + add placeholder views

**Files:**
- Create: `packages/web/src/views/artists.rs`
- Create: `packages/web/src/views/downloads.rs`
- Create: `packages/web/src/views/playlists.rs`
- Modify: `packages/web/src/main.rs`
- Modify: `packages/web/src/views/mod.rs`

- [ ] **Step 1: Create placeholder views**

`artists.rs`:
```rust
use dioxus::prelude::*;

#[component]
pub fn Artists() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 40vh; color: var(--k-on-surface-variant);",
            h2 { style: "font-family: var(--k-font-display); color: var(--k-on-surface);", "Artists" }
            p { "Coming soon" }
        }
    }
}
```

`downloads.rs`: Same pattern with "Downloads" title.

`playlists.rs`: Import and render `PlaylistSidebarSection` (reused from existing code) full-width. Wrap it with a title header.

```rust
use crate::views::PlaylistSidebarSection;
use dioxus::prelude::*;

#[component]
pub fn Playlists() -> Element {
    rsx! {
        div { style: "padding: var(--k-space-4);",
            h2 {
                style: "font-family: var(--k-font-display); font-size: 1.75rem; color: var(--k-on-surface); margin-bottom: var(--k-space-4);",
                "Playlists"
            }
            PlaylistSidebarSection {}
        }
    }
}
```

- [ ] **Step 2: Update `packages/web/src/views/mod.rs`**

Add new modules:
```rust
mod library;
pub use library::Library;

mod album_detail;
pub use album_detail::AlbumDetail;

mod artists;
pub use artists::Artists;

mod downloads;
pub use downloads::Downloads;

mod playlists;
pub use playlists::Playlists;

mod playlist_sidebar;
pub use playlist_sidebar::PlaylistSidebarSection;

mod playlist_view;
pub use playlist_view::PlaylistView;
```

- [ ] **Step 3: Update Route enum in `packages/web/src/main.rs`**

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Library {},
        #[route("/album/:name")]
        AlbumDetail { name: String },
        #[route("/artists")]
        Artists {},
        #[route("/playlists")]
        Playlists {},
        #[route("/playlist/:id")]
        PlaylistView { id: Uuid },
        #[route("/downloads")]
        Downloads {},
}
```

Update imports at the top of `main.rs` to include the new view types.

- [ ] **Step 4: Update sidebar nav links in AppLayout**

Replace the current sidebar children with route-mapped nav items:
```rust
Sidebar {
    Link { class: "sidebar__nav-item", to: Route::Artists {}, "Artists" }
    Link { class: "sidebar__nav-item", to: Route::Library {}, "Albums" }
    Link { class: "sidebar__nav-item", to: Route::Playlists {}, "Playlists" }
    Link { class: "sidebar__nav-item", to: Route::Downloads {}, "Downloads" }
    PlaylistSidebarSection {}
}
```

- [ ] **Step 5: Verify**

Run: `cargo check -p dioxus_music_web`

- [ ] **Step 6: Commit**

```bash
git add -A packages/web/
git commit -m "feat(dioxus_music): expand routes with placeholder views for Artists, Playlists, Downloads"
```

---

## Task 3: Rewrite Library view as album grid

**Files:**
- Rewrite: `packages/web/src/views/library.rs`
- Create: `packages/web/assets/library.css`

- [ ] **Step 1: Create `packages/web/assets/library.css`**

```css
.library {
    padding: var(--k-space-6);
}

.library__header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin-bottom: var(--k-space-6);
}

.library__title {
    font-family: var(--k-font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--k-on-surface);
}

.library__subtitle {
    color: var(--k-on-surface-variant);
    font-size: 0.875rem;
    margin-top: var(--k-space-1);
}

.library__toggle {
    background: transparent;
    border: none;
    color: var(--k-primary);
    font-size: 0.875rem;
    cursor: pointer;
    padding: var(--k-space-1) var(--k-space-2);
    border-radius: var(--k-radius-default);
    transition: background 150ms ease;
}

.library__toggle:hover {
    background: var(--k-surface-high);
}

.album-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: var(--k-space-4);
}

.album-card {
    cursor: pointer;
    transition: transform 150ms ease;
    text-decoration: none;
    color: inherit;
    display: block;
}

.album-card:hover {
    transform: translateY(-2px);
}

.album-card__art {
    aspect-ratio: 1;
    background: var(--k-surface-highest);
    border-radius: var(--k-radius-lg);
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--k-font-display);
    font-size: 2rem;
    color: var(--k-on-surface-variant);
    margin-bottom: var(--k-space-2);
    overflow: hidden;
}

.album-card__name {
    font-size: 0.875rem;
    color: var(--k-on-surface);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.album-card__artist {
    font-size: 0.75rem;
    color: var(--k-on-surface-variant);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
```

- [ ] **Step 2: Rewrite `packages/web/src/views/library.rs`**

Remove the rescan button (moved to settings dropdown in Task 5). Add album grid + all-songs toggle.

```rust
use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::{group_tracks_into_albums, TrackList};

const LIBRARY_CSS: Asset = asset!("/assets/library.css");

#[component]
pub fn Library() -> Element {
    let tracks = use_server_future(dioxus_music_api::get_library)?;
    let mut show_all_songs = use_signal(|| false);

    let result = tracks.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: LIBRARY_CSS }

        div { class: "library",
            div { class: "library__header",
                div {
                    h1 { class: "library__title", "Library" }
                    p { class: "library__subtitle", "Your high-fidelity audio repository" }
                }
                button {
                    class: "library__toggle",
                    onclick: move |_| show_all_songs.toggle(),
                    if show_all_songs() { "Album Grid" } else { "All Songs" }
                }
            }

            match result {
                Some(Ok(track_list)) => {
                    if show_all_songs() {
                        rsx! { TrackList { tracks: track_list } }
                    } else {
                        let albums = group_tracks_into_albums(&track_list);
                        rsx! {
                            div { class: "album-grid",
                                for album in albums {
                                    {
                                        let initial = album.name.chars().next().unwrap_or('?').to_uppercase().to_string();
                                        let album_name = album.name.clone();
                                        rsx! {
                                            Link {
                                                class: "album-card",
                                                to: Route::AlbumDetail { name: album_name },
                                                div { class: "album-card__art", "{initial}" }
                                                div { class: "album-card__name", "{album.name}" }
                                                div { class: "album-card__artist", "{album.artist}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { "Error loading library: {e}" } },
                None => rsx! { p { "Loading..." } },
            }
        }
    }
}
```

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_web`

- [ ] **Step 4: Commit**

```bash
git add packages/web/src/views/library.rs packages/web/assets/library.css
git commit -m "feat(dioxus_music): rewrite Library view as album grid with all-songs toggle"
```

---

## Task 4: Create Album Detail view

**Files:**
- Create: `packages/web/src/views/album_detail.rs`
- Create: `packages/web/assets/album_detail.css`

- [ ] **Step 1: Create `packages/web/assets/album_detail.css`**

Key classes:
- `.album-detail` — padding
- `.album-detail__back` — back link styling (muted, hover primary)
- `.album-detail__header` — CSS grid: `grid-template-columns: 200px 1fr`, gap
- `.album-detail__art` — square, aspect-ratio 1, surface-highest, radius-xl, centered initial
- `.album-detail__meta` — flex column, gap
- `.album-detail__label` — mono, secondary, uppercase, small ("NOW VIEWING")
- `.album-detail__title` — display font, 2rem, bold
- `.album-detail__artist-row` — flex row, gap, items center
- `.album-detail__actions` — flex row, gap
- `.album-detail__tracklist` — margin-top spacing
- `.album-detail__footer` — mono, muted, small, margin-top

- [ ] **Step 2: Create `packages/web/src/views/album_detail.rs`**

```rust
use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::TrackList;
use kinetic_ui::{Badge, BadgeVariant, Button, ButtonVariant, IconButton};

const ALBUM_DETAIL_CSS: Asset = asset!("/assets/album_detail.css");

fn format_duration_minutes(total_secs: i32) -> String {
    let minutes = total_secs / 60;
    if minutes < 60 {
        format!("{minutes} minutes")
    } else {
        let hours = minutes / 60;
        let remaining_mins = minutes % 60;
        format!("{hours}h {remaining_mins}m")
    }
}

#[component]
pub fn AlbumDetail(name: String) -> Element {
    let tracks = use_server_future(dioxus_music_api::get_library)?;
    let result = tracks.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: ALBUM_DETAIL_CSS }

        div { class: "album-detail",
            // Back nav
            Link { class: "album-detail__back", to: Route::Library {},
                "← Back"
            }

            match result {
                Some(Ok(all_tracks)) => {
                    let album_tracks: Vec<_> = all_tracks.into_iter()
                        .filter(|t| t.album == name)
                        .collect();

                    if album_tracks.is_empty() {
                        rsx! { p { "No tracks found for this album." } }
                    } else {
                        let artist = {
                            let first = &album_tracks[0].artist;
                            if album_tracks.iter().all(|t| t.artist == *first) {
                                first.clone()
                            } else {
                                "Various Artists".to_string()
                            }
                        };
                        let genre = album_tracks[0].genre.clone();
                        let track_count = album_tracks.len();
                        let total_secs: i32 = album_tracks.iter().map(|t| t.duration_secs).sum();
                        let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();

                        rsx! {
                            // Asymmetric header
                            div { class: "album-detail__header",
                                div { class: "album-detail__art", "{initial}" }
                                div { class: "album-detail__meta",
                                    span { class: "album-detail__label", "NOW VIEWING" }
                                    h1 { class: "album-detail__title", "{name}" }
                                    div { class: "album-detail__artist-row",
                                        span { "{artist}" }
                                        Badge { variant: BadgeVariant::Muted, "{genre}" }
                                    }
                                    div { class: "album-detail__actions",
                                        Button { variant: ButtonVariant::Primary, "Download Album" }
                                        IconButton {
                                            // Heart icon
                                            svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                                path { d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" }
                                            }
                                        }
                                    }
                                }
                            }

                            // Tracklist
                            div { class: "album-detail__tracklist",
                                TrackList { tracks: album_tracks, show_download_status: true }
                            }

                            // Footer
                            div { class: "album-detail__footer",
                                "{track_count} tracks • {format_duration_minutes(total_secs)}"
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { "Error: {e}" } },
                None => rsx! { p { "Loading..." } },
            }
        }
    }
}
```

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_web`

- [ ] **Step 4: Commit**

```bash
git add packages/web/src/views/album_detail.rs packages/web/assets/album_detail.css
git commit -m "feat(dioxus_music): add Album Detail view with asymmetric header and tracklist"
```

---

## Task 5: Add settings dropdown to Header

**Files:**
- Create: `packages/ui/assets/styling/settings-dropdown.css`
- Modify: `packages/ui/src/header.rs`

- [ ] **Step 1: Create settings dropdown CSS**

Create `packages/ui/assets/styling/settings-dropdown.css`:

```css
.settings-dropdown {
    position: relative;
}

.settings-dropdown__menu {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: var(--k-space-1);
    background: var(--k-surface-highest);
    border-radius: var(--k-radius-lg);
    box-shadow: var(--k-shadow-float);
    padding: var(--k-space-1);
    min-width: 180px;
    z-index: 1000;
}

.settings-dropdown__item {
    display: flex;
    align-items: center;
    gap: var(--k-space-2);
    padding: var(--k-space-2) var(--k-space-3);
    border-radius: var(--k-radius-sm);
    color: var(--k-on-surface);
    font-size: 0.875rem;
    cursor: pointer;
    border: none;
    background: transparent;
    width: 100%;
    text-align: left;
    transition: background 150ms ease;
}

.settings-dropdown__item:hover {
    background: var(--k-surface-high);
}
```

- [ ] **Step 2: Modify `packages/ui/src/header.rs`**

Read current file first. Add a settings dropdown that wraps the settings IconButton. When clicked, toggles a dropdown with "Rescan Library" item.

The "Rescan Library" item calls `dioxus_music_api::rescan_library()`. This is a server function — it's available because `dioxus_music_ui` depends on `dioxus_music_api`.

Add `use_signal(|| false)` for dropdown open state. The settings icon toggles it. The dropdown item calls rescan and closes.

Load the new CSS file alongside the existing header CSS.

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_ui` and `cargo check -p dioxus_music_web`

- [ ] **Step 4: Commit**

```bash
git add packages/ui/src/header.rs packages/ui/assets/styling/settings-dropdown.css
git commit -m "feat(dioxus_music): add settings dropdown with Rescan Library to Header"
```

---

## Task 6: Visual polish + cleanup

**Files:**
- Various CSS files
- Possibly Rust files for clippy

- [ ] **Step 1: Verify the full flow**

Run `dx serve --package dioxus_music_web` (if DB available) or `cargo check -p dioxus_music_web`.

Check:
- `/` shows album grid
- "All Songs" toggle works
- Clicking an album card navigates to `/album/{name}`
- Album detail shows header + tracklist
- Back link returns to library
- Sidebar nav items highlight correctly
- Settings dropdown opens and "Rescan Library" is accessible
- `/artists`, `/downloads` show placeholders
- `/playlists` shows playlist list

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -p dioxus_music_ui -p dioxus_music_web`

Fix any warnings. Add `#![allow(clippy::volatile_composites)]` if needed.

- [ ] **Step 3: Commit**

```bash
git add -A dioxus_music/
git commit -m "style(dioxus_music): polish Library + Album Detail views, clippy cleanup"
```

---

## Summary

| Task | Description | Key Files |
|---|---|---|
| 1 | AlbumSummary model + grouping function | `ui/src/album_utils.rs` |
| 2 | Expand routes + placeholder views | `web/src/main.rs`, new view files |
| 3 | Library album grid + all-songs toggle | `web/src/views/library.rs`, `library.css` |
| 4 | Album Detail view | `web/src/views/album_detail.rs`, `album_detail.css` |
| 5 | Settings dropdown in Header | `ui/src/header.rs`, `settings-dropdown.css` |
| 6 | Polish + clippy | Various |

Tasks 1-2 are sequential (model first, then routes). Tasks 3-5 depend on Task 2 (routes exist) but are independent of each other. Task 6 is cleanup after everything.
