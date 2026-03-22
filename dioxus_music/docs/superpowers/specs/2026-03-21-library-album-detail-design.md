# dioxus_music Library + Album Detail Views — Design Specification

## 1. Overview

Sub-project 2 of 4 in the dioxus_music Kinetic Obsidian redesign. This spec covers:

1. **Expanded routing** — Artists, Albums, Playlists, Downloads routes (placeholders for most, functional for Library and Album Detail)
2. **Library view** — Album grid derived from track metadata, "All Songs" toggle for flat track list
3. **Album Detail view** — Asymmetric header with metadata, tracklist using existing TrackList component
4. **Settings dropdown** — In the Header, with "Rescan Library" option (replaces the button in Library view)
5. **Placeholder views** — Artists, Downloads show "Coming soon"
6. **Playlists view** — Promotes existing playlist sidebar content to a full view

---

## 2. Data Model

### 2.1 AlbumSummary (derived, not persisted)

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct AlbumSummary {
    pub name: String,
    pub artist: String,       // most common artist, or "Various Artists"
    pub genre: String,        // from first track
    pub track_count: usize,
    pub total_duration_secs: i32,
}
```

This struct is computed client-side from `Vec<TrackSummary>` grouped by the `album` field. No database changes, no new server functions for albums.

### 2.2 Grouping Logic

```rust
fn group_tracks_into_albums(tracks: &[TrackSummary]) -> Vec<AlbumSummary> {
    // Group by album name
    // For each group: count tracks, sum durations
    // Artist: if all tracks share the same artist, use it; otherwise "Various Artists"
    // Genre: take from first track
    // Sort alphabetically by album name
}
```

This is a pure function in the `ui` crate or in the web crate's views.

---

## 3. Routes

### 3.1 Web Crate Routes

```rust
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

**Note:** `/albums` is not a separate route — `/` (Library) IS the albums view. The sidebar "Albums" nav item links to `/`.

### 3.2 Desktop Crate Routes

Same structure. Desktop currently has no server functions, so `use_server_future()` calls won't work. The views can render empty states or be populated later when desktop gets fullstack support.

### 3.3 Sidebar Nav Mapping

| Sidebar Item | Route        | Active when                          |
| ------------ | ------------ | ------------------------------------ |
| Artists      | `/artists`   | path starts with `/artists`          |
| Albums       | `/`          | path is `/` or starts with `/album/` |
| Playlists    | `/playlists` | path starts with `/playlist`         |
| Downloads    | `/downloads` | path starts with `/downloads`        |

---

## 4. Library View

### 4.1 Layout

```
┌─────────────────────────────────────────┐
│  Library                    [All Songs] │  ← title + toggle
│  Your high-fidelity audio repository    │  ← subtitle
├─────────────────────────────────────────┤
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐  │
│  │ art  │ │ art  │ │ art  │ │ art  │  │
│  │      │ │      │ │      │ │      │  │
│  ├──────┤ ├──────┤ ├──────┤ ├──────┤  │
│  │Album │ │Album │ │Album │ │Album │  │
│  │Artist│ │Artist│ │Artist│ │Artist│  │
│  └──────┘ └──────┘ └──────┘ └──────┘  │  ← responsive grid
│  ...                                    │
└─────────────────────────────────────────┘
```

### 4.2 Components

**Page header:**

- "Library" in `--k-font-display`, headline size
- Subtitle: "Your high-fidelity audio repository" in `--k-on-surface-variant`
- "All Songs" link/button on the right — toggles between album grid and flat track list
- Uses local `use_signal(|| false)` for the toggle state

**Album grid:**

- CSS grid: `grid-template-columns: repeat(auto-fill, minmax(160px, 1fr))`
- Gap: `--k-space-4`
- Responsive: naturally adapts from 2 columns (narrow) to 6+ (wide)

**Album card:**

- Square art placeholder: `aspect-ratio: 1`, `--k-surface-highest` background, centered album initial letter (large, `--k-font-display`, `--k-on-surface-variant`), rounded `--k-radius-lg`
- Album name: below art, `--k-on-surface`, `font-size: 0.875rem`, truncated single line
- Artist name: below album, `--k-on-surface-variant`, `font-size: 0.75rem`, truncated
- Hover: `transform: translateY(-2px)`, `transition: transform 150ms ease`
- Click: navigates to `/album/{url_encoded_name}`

**"All Songs" mode:**

- When toggled, hides the album grid and shows `TrackList` with all tracks
- Toggle button text switches to "Album Grid" when in all-songs mode

### 4.3 Data Loading

```rust
// In Library component:
let library = use_server_future(get_library)?;
let albums = use_memo(move || {
    library().map(|tracks| group_tracks_into_albums(&tracks))
});
```

---

## 5. Album Detail View

### 5.1 Layout

```
┌─────────────────────────────────────────┐
│  ← Back                                 │  ← back nav
├────────────┬────────────────────────────┤
│            │  NOW VIEWING               │
│   [art]    │  ALBUM TITLE              │
│            │  Artist • Genre            │
│            │  [Download] [♥] [⋯]        │
├────────────┴────────────────────────────┤
│  # │ Title        │ Duration │ Status   │
│  1 │ Track One    │ 4:23     │ ✓        │
│  2 │ Track Two    │ 3:45     │ ✓        │
│  ...                                    │
├─────────────────────────────────────────┤
│  5 tracks • 22 minutes                  │  ← footer
└─────────────────────────────────────────┘
```

### 5.2 Components

**Back navigation:**

- "← Back" link at top, navigates to `/` (Library)
- `--k-on-surface-variant` color, hover `--k-on-surface`

**Album header — asymmetric grid:**

- CSS grid: `grid-template-columns: 200px 1fr` (or `250px 1fr` on wider screens)
- Left: Art placeholder (square, `--k-radius-xl` rounding, `--k-surface-highest`, album initial)
- Right (stacked):
  - "NOW VIEWING" label: `--k-font-mono`, `--k-secondary`, uppercase, small
  - Album title: `--k-font-display`, `2rem`, `--k-on-surface`, bold
  - Artist + genre row: artist name, bullet separator, genre `Badge` (with `BadgeVariant::Muted`)
  - Action buttons row: "Download Album" `Button` Primary (non-functional placeholder), heart `IconButton`, more-menu `IconButton`

**Tracklist:**

- `TrackList` component with `show_download_status: true`
- Tracks filtered to the current album name

**Footer:**

- Track count + total duration formatted: "5 tracks • 22 minutes"
- `--k-font-mono`, `--k-on-surface-variant`, small

### 5.3 Data Loading

```rust
// In AlbumDetail component:
let library = use_server_future(get_library)?;
let album_tracks = use_memo(move || {
    library().map(|tracks| {
        tracks.into_iter().filter(|t| t.album == name).collect::<Vec<_>>()
    })
});
```

The album name is URL-decoded from the route parameter.

---

## 6. Settings Dropdown

### 6.1 Location

Modified in `packages/ui/src/header.rs`. The existing settings `IconButton` gains dropdown behavior.

### 6.2 Implementation

- Local `use_signal(|| false)` for open/closed state
- Clicking the settings icon toggles the signal
- When open: render an absolute-positioned div below the icon
  - Background: `--k-surface-highest`
  - Rounding: `--k-radius-lg`
  - Shadow: `--k-shadow-float`
  - z-index: 1000
  - One item: "Rescan Library" button
- "Rescan Library" calls `rescan_library()` server function and closes the dropdown
- Click outside: listen for click events on the document/body to close

### 6.3 CSS

New file: `packages/ui/assets/styling/settings-dropdown.css`

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
  transition: background 150ms ease;
}
.settings-dropdown__item:hover {
  background: var(--k-surface-high);
}
```

---

## 7. Placeholder Views

### 7.1 Artists View

```rust
// Centered placeholder
div {
    h2 { "Artists" }
    p { "Coming soon" }
}
```

Styled with kinetic tokens, centered in content area.

### 7.2 Downloads View

Same pattern as Artists.

### 7.3 Playlists View

Promotes the existing `PlaylistSidebarSection` content:

- Shows playlist list (manual + smart)
- Create buttons (manual + smart)
- Click navigates to `/playlist/:id`

This reuses the existing `PlaylistSidebarSection` component from `packages/web/src/views/playlist_sidebar.rs`, but rendered full-width in the content area instead of in the sidebar. The sidebar still shows playlists as quick nav links.

---

## 8. New Files

| File                                               | Responsibility         |
| -------------------------------------------------- | ---------------------- |
| `packages/web/src/views/album_detail.rs`           | Album Detail view      |
| `packages/web/src/views/artists.rs`                | Artists placeholder    |
| `packages/web/src/views/downloads.rs`              | Downloads placeholder  |
| `packages/web/src/views/playlists.rs`              | Playlists list view    |
| `packages/web/assets/library.css`                  | Library album grid CSS |
| `packages/web/assets/album_detail.css`             | Album Detail CSS       |
| `packages/ui/assets/styling/settings-dropdown.css` | Settings dropdown CSS  |

## 9. Modified Files

| File                                | Changes                                                                           |
| ----------------------------------- | --------------------------------------------------------------------------------- |
| `packages/web/src/main.rs`          | New routes, updated sidebar nav links                                             |
| `packages/web/src/views/mod.rs`     | New view modules                                                                  |
| `packages/web/src/views/library.rs` | Rewrite: album grid + all-songs toggle (replaces flat track list + rescan button) |
| `packages/ui/src/header.rs`         | Add settings dropdown with "Rescan Library"                                       |
| `packages/ui/src/lib.rs`            | Export AlbumSummary + grouping function if placed in ui crate                     |

---

## 10. Out of Scope

- Album cover art (no art files, placeholders only)
- Album entity in database (derived from track metadata)
- Recently Played section (no play history tracking)
- Ready for Flight / offline suggestions
- Popularity dots in tracklist
- Search functionality
- Artist detail view content
- Downloads view content
