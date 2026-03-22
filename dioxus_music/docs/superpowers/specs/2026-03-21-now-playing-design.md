# dioxus_music Now Playing View — Design Specification

## 1. Overview

Sub-project 3 of 4 in the dioxus_music Kinetic Obsidian redesign. This spec covers:

1. **Now Playing view** — full-screen player with album art, controls, progress bar, technical metadata, and inline queue
2. **Audio element extraction** — move the `<audio>` element from PlayerBar into AppShell so it persists regardless of view
3. **PlayerBar modifications** — clickable track info expands to Now Playing, hidden when on `/now-playing` route
4. **Responsive layout** — two-column on desktop, single column on narrow/mobile

The Stitch mockup for Now Playing was malformed. This design is built from scratch, referencing the mockup's HTML content for intended features.

---

## 2. Route

New route `/now-playing` added to both web and desktop crates.

**Web Route enum:**
```rust
#[route("/now-playing")]
NowPlaying {},
```

**Desktop Route enum:**
```rust
#[route("/now-playing")]
NowPlaying {},
```

Both require a `NowPlaying` component in their respective `views/` modules.

**Mobile:** No Now Playing route for now. Mobile uses the compact PlayerBar only. The mobile crate is unchanged by this spec.

**Accessing:** Clicking the track title/art area in the floating PlayerBar fires an `on_expand` callback that navigates to `/now-playing`.

**Back navigation:** A back button at the top of the Now Playing view uses `navigator().go_back()` or navigates to `/`.

**No track playing:** Shows centered placeholder "No track playing — select a song from your library."

---

## 3. Audio Element Extraction

### 3.1 Problem

The hidden `<audio id="main-audio">` element currently lives inside `PlayerBar`. When PlayerBar is hidden on the Now Playing route, the audio element would be removed from the DOM, stopping playback.

### 3.2 Solution

Extract a `render_audio_element` function in `packages/ui` that renders the audio element. Call it from `AppShell` (web/desktop) and `MobileLayout` (mobile).

**The function reads `PlayerState` reactively:**

```rust
fn render_audio_element(mut player: Signal<PlayerState>) -> Element {
    let track_info = player.read().current_track.clone();
    let repeat_mode = player.read().repeat_mode;

    let audio_src = track_info
        .as_ref()
        .map(|t| format!("/stream/{}", t.id))
        .unwrap_or_default();
    let has_track = track_info.is_some();
    let is_looping = repeat_mode == RepeatMode::One;

    rsx! {
        if has_track {
            audio {
                id: "main-audio",
                src: "{audio_src}",
                autoplay: true,
                r#loop: is_looping,
                onended: move |_| {
                    player.with_mut(|p| p.next_track());
                    let _ = document::eval(r#"
                        let a = document.getElementById('main-audio');
                        if (a && a.src) { a.load(); a.play(); }
                    "#);
                },
            }
        }
    }
}
```

The `audio_src` is a reactive binding — when `PlayerState.current_track` changes (via signal write), the component re-renders and the `<audio>` element gets a new `src`, triggering reload via `autoplay: true`.

**AppShell renders it outside the grid, alongside PlayerBar:**

```rust
KineticTheme {
    div { class: "app-shell", /* grid content */ }
    PlayerBar { /* props */ }
    {render_audio_element(player)}  // persists across all routes
}
```

**PlayerBar changes:** Remove the `<audio>` element and `onended` handler. PlayerBar becomes purely visual controls that call `document::eval()`.

### 3.3 Mobile crate

`MobileLayout` renders the audio element at its layout level:

```rust
KineticTheme {
    main { /* content */ }
    PlayerBar { compact: true }
    MobileNav {}
    {render_audio_element(player)}  // audio persists here too
}
```

The `render_audio_element` function is exported from `packages/ui` (e.g., in `player_state.rs` or a new `audio.rs` module).

---

## 4. Now Playing View — Layout

### 4.1 Desktop (>768px) — Two-Column

```
┌──────────────────────────────────────────────────┐
│ ← Back                                           │
├──────────────────────┬───────────────────────────┤
│                      │  CURRENTLY STREAMING      │
│      ┌────────┐      │                           │
│      │  art   │      │  ┌──────┬──────┬──────┐  │
│      │ (300px)│      │  │ FLAC │— kbps│Local │  │
│      └────────┘      │  │Codec │Bitrate│Source│  │
│                      │  └──────┴──────┴──────┘  │
│    Track Title       │                           │
│    Artist Name       │  Queue • 12 tracks        │
│                      │  ├─ Now Playing ──────┤   │
│  ──●──────── 2:44    │  ├─ Next Track ───────┤   │
│    4:12              │  ├─ Another Track ─────┤   │
│                      │  └────────────────────┘   │
│  Sh ⏮ ▶ ⏭ R        │                           │
│                      │  🔊 ━━━━━━━━━━━━━━━━     │
│                      │                           │
└──────────────────────┴───────────────────────────┘
```

CSS grid: `grid-template-columns: 3fr 2fr` (art-dominant left). Responsive via `@media (max-width: 768px)` — same breakpoint pattern used throughout the app.

### 4.2 Mobile/Narrow (<768px) — Single Column

```
┌───────────────────┐
│ ← Back            │
│                   │
│    ┌────────┐     │
│    │  art   │     │
│    │ (200px)│     │
│    └────────┘     │
│                   │
│  Track Title      │
│  Artist Name      │
│                   │
│  ──●──── 2:44     │
│    4:12           │
│                   │
│  Sh ⏮ ▶ ⏭ R     │
│                   │
│  FLAC  — kbps  Local │ ← inline chips
└───────────────────┘
```

Queue hidden on narrow — accessible via Q button which opens the existing `QueuePanel` sliding overlay (same component used elsewhere in the app, toggled via `PlayerState.show_queue`). Volume hidden on narrow.

---

## 5. Now Playing View — Components

### 5.1 Back Navigation

- "← Back" link at top-left
- Uses `navigator().go_back()` or falls back to `Route::Library {}`
- Styled: `--k-on-surface-variant`, hover `--k-on-surface`

### 5.2 Album Art

- Large placeholder: 300px on desktop, 200px on mobile (via CSS media query)
- `--k-surface-highest` background
- `--k-radius-xl` rounding
- Centered album initial letter (`--k-font-display`, large)
- Reads from `PlayerState.current_track` for the album name initial

### 5.3 Track Info

- Title: `--k-font-display`, `2rem` on desktop, `1.5rem` on mobile, `--k-on-surface`
- Artist: `1rem`, `--k-on-surface-variant`
- Centered on left column (desktop) or centered full-width (mobile)

### 5.4 Progress Bar

- Horizontal bar: `--k-surface-highest` background track, gradient fill (`--k-primary` to `--k-primary-container`)
- Timestamps: monospace, `--k-on-surface-variant`, `0.75rem`
- **Non-functional for now** — shows `0:00` / duration from `current_track.duration_secs`. Styled with `pointer-events: none` and `opacity: 0.7` on the bar to indicate it's not interactive. Actual seek requires JS interop with audio `currentTime` (future work).

### 5.5 Transport Controls

Same controls as PlayerBar but larger:
- Shuffle button: `--k-on-surface-variant`, active `--k-primary`
- Previous: standard button
- Play/Pause: gradient circle, 48px (larger than PlayerBar's 36px)
- Next: standard button
- Repeat: shows R/RA/R1 like PlayerBar, active `--k-primary`
- All use `document::eval()` for audio control, same pattern as PlayerBar
- Queue toggle (Q button): visible on narrow screens to open QueuePanel

### 5.6 Technical Metadata (Bento Cards)

Three cards in a row (desktop) or inline chips (mobile):

| Card | Value | Color | Label |
|---|---|---|---|
| Codec | "FLAC" (placeholder) | `--k-primary` | "Codec" |
| Bitrate | "— kbps" (placeholder) | `--k-secondary` | "Bitrate" |
| Source | "Local" (placeholder) | `--k-tertiary` | "Source" |

Desktop: `--k-surface-low` background cards, `--k-radius-lg` rounding, monospace value text.
Mobile: small pill chips inline below controls.

All values are hardcoded placeholders. When real metadata is available (future), they'll read from track data.

### 5.7 Queue (Desktop Only, >768px)

- Header: "Queue • N tracks" (Space Grotesk + monospace count)
- Inline list of tracks from `PlayerState.queue`
- Current track highlighted: `--k-surface-highest` background, `--k-primary` title color
- Other tracks: `--k-on-surface` title, `--k-on-surface-variant` artist
- Click to jump: calls `player.with_mut(|p| p.jump_to(index))` + audio eval
- Scrollable if queue is long (`max-height` with `overflow-y: auto`)

### 5.8 Volume (Desktop Only, >768px)

- Volume icon + horizontal slider
- `--k-on-surface-variant` icon, `--k-surface-highest` track, `--k-on-surface` fill
- **Non-functional for now** — styled with `pointer-events: none` and `opacity: 0.7` to indicate non-interactive. Visual only. Actual volume control requires JS interop with audio `volume` property (future work).

---

## 6. PlayerBar Modifications

### 6.1 Clickable Track Info

**Problem:** PlayerBar is in the shared `packages/ui` crate and doesn't know about platform-specific `Route` enums.

**Solution:** Add an optional `on_expand: Option<EventHandler<()>>` prop to PlayerBar. When the track info area (art + title + artist) is clicked and `on_expand` is `Some`, fire the callback. Platform crates wire this to navigation:

```rust
// In web AppLayout or desktop DesktopLayout:
let nav = navigator();
// ... in AppShell:
PlayerBar {
    on_expand: move |_| { nav.push(Route::NowPlaying {}); },
}
```

The track info area gets `cursor: pointer` styling when `on_expand` is provided.

### 6.2 Hide on Now Playing Route

**Final approach:** PlayerBar accepts `hidden: bool` prop (defaults to `false`). When `true`, PlayerBar renders nothing — just returns `rsx! {}`. The audio element is in AppShell so playback continues.

Platform layout components detect the current route and pass `hidden: true` when on `/now-playing`. Since AppShell doesn't know about routes, the `hidden` prop must be passed through from the platform crate. The simplest way:

- AppShell accepts an optional `player_bar_hidden: Option<bool>` prop and passes it to PlayerBar
- Platform layout components set this based on the current route

```rust
// In web AppLayout:
let route = use_route::<Route>();
let on_now_playing = matches!(route, Route::NowPlaying {});

AppShell {
    player_bar_hidden: on_now_playing,
    sidebar: rsx! { /* ... */ },
    Outlet::<Route> {}
}
```

---

## 7. New Files

| File | Responsibility |
|---|---|
| `packages/web/src/views/now_playing.rs` | Now Playing view (web) |
| `packages/web/assets/now_playing.css` | Now Playing CSS (web) |
| `packages/desktop/src/views/now_playing.rs` | Now Playing view (desktop) |
| `packages/ui/src/audio.rs` | `render_audio_element` function |

## 8. Modified Files

| File | Changes |
|---|---|
| `packages/ui/src/app_shell.rs` | Add `player_bar_hidden` prop, render audio element, pass props to PlayerBar |
| `packages/ui/src/player_bar.rs` | Remove audio element, add `on_expand` + `hidden` props |
| `packages/ui/src/lib.rs` | Export audio module |
| `packages/web/src/main.rs` | Add `/now-playing` route, pass route-aware props to AppShell |
| `packages/web/src/views/mod.rs` | Add now_playing module |
| `packages/desktop/src/main.rs` | Add `/now-playing` route, pass route-aware props to AppShell |
| `packages/desktop/src/views/mod.rs` | Add now_playing module |
| `packages/mobile/src/main.rs` | Add audio element to MobileLayout |

---

## 9. Out of Scope

- Actual seekable progress bar (requires JS interop with audio currentTime)
- Actual volume control (requires JS interop with audio volume)
- Real codec/bitrate/source metadata (not in data model)
- Album cover art (placeholders only)
- Visualizer bars / audio waveform
- Swipe-to-dismiss gesture (mobile)
- Now Playing route for mobile crate (mobile uses compact PlayerBar only)
