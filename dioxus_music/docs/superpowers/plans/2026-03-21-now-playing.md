# Now Playing View Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a full-screen Now Playing view with album art, transport controls, progress bar, technical metadata bento cards, inline queue, and volume — plus extract the audio element from PlayerBar into AppShell for persistence.

**Architecture:** Extract `render_audio_element` function from PlayerBar into a shared `audio.rs` module. AppShell and MobileLayout render it at the layout level. PlayerBar becomes purely visual controls with new `on_expand` and `hidden` props. Now Playing is a new route `/now-playing` with two-column layout (desktop) collapsing to single column (mobile).

**Tech Stack:** Rust nightly, Dioxus 0.7, kinetic_ui, dioxus_music_api (PlayerState context)

**Spec:** `dioxus_music/docs/superpowers/specs/2026-03-21-now-playing-design.md`

**No tests.** Verification: `cargo check` and `dx serve`.

---

## File Structure

### New files

| File | Responsibility |
|---|---|
| `packages/ui/src/audio.rs` | `render_audio_element` function extracted from PlayerBar |
| `packages/web/src/views/now_playing.rs` | Now Playing view (web) |
| `packages/web/assets/now_playing.css` | Now Playing CSS |
| `packages/desktop/src/views/now_playing.rs` | Now Playing view (desktop) |

### Modified files

| File | Changes |
|---|---|
| `packages/ui/src/lib.rs` | Export audio module |
| `packages/ui/src/player_bar.rs` | Remove audio element, add `on_expand` + `hidden` props |
| `packages/ui/src/app_shell.rs` | Add `player_bar_hidden` + `on_player_expand` props, render audio element |
| `packages/web/src/main.rs` | Add `/now-playing` route, pass props to AppShell |
| `packages/web/src/views/mod.rs` | Add now_playing module |
| `packages/desktop/src/main.rs` | Add `/now-playing` route, pass props to AppShell |
| `packages/desktop/src/views/mod.rs` | Add now_playing module |
| `packages/mobile/src/main.rs` | Add audio element to MobileLayout |

---

## Task 1: Extract audio element into shared module

**Files:**
- Create: `packages/ui/src/audio.rs`
- Modify: `packages/ui/src/lib.rs`
- Modify: `packages/ui/src/player_bar.rs`

- [ ] **Step 1: Create `packages/ui/src/audio.rs`**

Extract the audio element rendering from PlayerBar into a standalone function. **Note:** Spec shows this taking a `Signal<PlayerState>` parameter, but using `use_player_state()` hook is cleaner and matches existing patterns — documented deviation.

```rust
use crate::player_state::{use_player_state, RepeatMode};
use dioxus::prelude::*;

pub fn render_audio_element() -> Element {
    let mut player = use_player_state();
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

- [ ] **Step 2: Update `packages/ui/src/lib.rs`**

Add: `pub mod audio;`

- [ ] **Step 3: Remove audio element from PlayerBar**

In `packages/ui/src/player_bar.rs`, remove lines 146-162 (the `// Hidden audio element` section including the `if has_track { audio { ... } }` block). Also remove the `audio_src`, `is_looping` variables and the `RepeatMode` import if no longer needed (but RepeatMode is still used for repeat_label — keep it).

Actually, `audio_src` and `is_looping` are only used by the audio element. Remove them. Keep `has_track` (used for button disabled state).

- [ ] **Step 4: Verify**

Run: `cargo check -p dioxus_music_ui`

- [ ] **Step 5: Commit**

```bash
git add packages/ui/src/audio.rs packages/ui/src/lib.rs packages/ui/src/player_bar.rs
git commit -m "refactor(dioxus_music): extract audio element from PlayerBar into shared audio module"
```

---

## Task 2: Wire audio element into AppShell + add PlayerBar props

**Files:**
- Modify: `packages/ui/src/app_shell.rs`
- Modify: `packages/ui/src/player_bar.rs`

- [ ] **Step 1: Add props to PlayerBar**

In `player_bar.rs`, add two new optional props:

```rust
#[component]
pub fn PlayerBar(
    #[props(default)] compact: bool,
    #[props(default)] hidden: bool,
    #[props(default)] on_expand: Option<EventHandler<()>>,
) -> Element {
```

When `hidden` is true, return early: `if hidden { return rsx! {}; }`

Make the track info area (art + info divs) clickable when `on_expand` is Some:

```rust
div {
    class: "player-bar__info-area",
    style: if on_expand.is_some() { "cursor: pointer;" } else { "" },
    onclick: move |_| {
        if let Some(handler) = &on_expand {
            handler.call(());
        }
    },
    // art placeholder
    div { class: "player-bar__art" }
    // track info
    div { class: "player-bar__info", /* ... */ }
}
```

Wrap the existing art + info divs in this clickable container.

- [ ] **Step 2: Add props to AppShell**

Update AppShell to accept new props and render the audio element:

```rust
use crate::audio::render_audio_element;

#[component]
pub fn AppShell(
    sidebar: Element,
    children: Element,
    #[props(default)] player_bar_hidden: bool,
    #[props(default)] on_player_expand: Option<EventHandler<()>>,
) -> Element {
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
            PlayerBar {
                hidden: player_bar_hidden,
                on_expand: on_player_expand,
            }
            {render_audio_element()}
        }
    }
}
```

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_ui`

Existing platform crates should still compile since the new props have defaults.

Run: `cargo check -p dioxus_music_web` and `cargo check -p dioxus_music_desktop`

- [ ] **Step 4: Commit**

```bash
git add packages/ui/src/app_shell.rs packages/ui/src/player_bar.rs
git commit -m "feat(dioxus_music): add PlayerBar expand/hidden props, wire audio element in AppShell"
```

---

## Task 3: Add audio element to MobileLayout

**Files:**
- Modify: `packages/mobile/src/main.rs`

- [ ] **Step 1: Import and render audio element**

In `MobileLayout`, add:

```rust
use dioxus_music_ui::audio::render_audio_element;
```

Add `{render_audio_element()}` after `MobileNav {}` in the MobileLayout RSX.

- [ ] **Step 2: Verify**

Run: `cargo check -p dioxus_music_mobile`

- [ ] **Step 3: Commit**

```bash
git add packages/mobile/src/main.rs
git commit -m "feat(dioxus_music): add audio element to MobileLayout"
```

---

## Task 4: Create Now Playing view + route (web)

**Files:**
- Create: `packages/web/src/views/now_playing.rs`
- Create: `packages/web/assets/now_playing.css`
- Modify: `packages/web/src/views/mod.rs`
- Modify: `packages/web/src/main.rs`

- [ ] **Step 1: Create `packages/web/assets/now_playing.css`**

Key classes:
- `.now-playing` — full height, padding, overflow-y auto
- `.now-playing__back` — back link, muted color, hover primary
- `.now-playing__layout` — CSS grid: `grid-template-columns: 3fr 2fr`, gap. At `max-width: 768px`: single column
- `.now-playing__left` — flex column, align-items center, gap
- `.now-playing__art` — width 300px, aspect-ratio 1, surface-highest bg, radius-xl, centered initial. At 768px: 200px
- `.now-playing__title` — font-display, 2rem, on-surface. At 768px: 1.5rem
- `.now-playing__artist` — 1rem, on-surface-variant
- `.now-playing__progress` — flex row, gap, align center, width 100%, max-width 400px, pointer-events none, opacity 0.7
- `.now-playing__progress-bar` — flex 1, height 4px, surface-highest bg, radius
- `.now-playing__progress-fill` — gradient primary to primary-container, height 100%, width 0%
- `.now-playing__progress-time` — mono, on-surface-variant, 0.75rem
- `.now-playing__controls` — flex row, gap, align center, justify center
- `.now-playing__btn` — same pattern as player-bar__btn but larger
- `.now-playing__btn--play` — 48px circle, gradient fill
- `.now-playing__right` — flex column, gap
- `.now-playing__section-label` — mono, secondary, uppercase, small, letter-spacing
- `.now-playing__bento` — grid, 3 columns, gap
- `.now-playing__bento-card` — surface-low bg, radius-lg, padding, flex column
- `.now-playing__bento-value` — mono, 1.25rem, font-weight 600
- `.now-playing__bento-label` — on-surface-variant, 0.6875rem
- `.now-playing__queue` — flex column, max-height 300px, overflow-y auto
- `.now-playing__queue-item` — flex row, gap, padding, radius-sm, cursor pointer, hover surface-high
- `.now-playing__queue-item--active` — surface-highest bg, primary title color
- `.now-playing__volume` — flex row, gap, align center, pointer-events none, opacity 0.7
- `.now-playing__volume-bar` — flex 1, height 3px, surface-highest bg, radius
- `.now-playing__chips` — flex row, gap (mobile: inline chips for metadata instead of bento)
- `.now-playing__chip` — surface-low bg, radius-full, padding, mono, small

Responsive: at 768px, `.now-playing__layout` becomes single column. Right column elements (queue, volume) hidden. Bento cards become chips. Q button shown in controls.

- [ ] **Step 2: Create `packages/web/src/views/now_playing.rs`**

Read `PlayerState` via `use_player_state()`. Read queue, current track, shuffle/repeat/playing state.

Structure:
- Back link → `navigator().go_back()` or `Route::Library {}`
- If no current_track: show placeholder
- If current_track: render the two-column layout

Left column:
- Art placeholder with album initial
- Title + artist
- Progress bar (non-functional: shows 0:00 / formatted duration)
- Transport controls (shuffle, prev, play/pause 48px, next, repeat, Q button on mobile)
- All use `document::eval()` same as PlayerBar

Right column (hidden on mobile via CSS):
- "CURRENTLY STREAMING" label
- Bento cards: Codec "FLAC" (primary), Bitrate "— kbps" (secondary), Source "Local" (tertiary)
- Queue header + inline track list from `player.read().queue`
- Volume slider (visual only, pointer-events none)

Mobile fallback (shown when right column hidden):
- Tech metadata as inline chips below controls

- [ ] **Step 3: Add route to web crate**

In `packages/web/src/views/mod.rs`: add `mod now_playing; pub use now_playing::NowPlaying;`

In `packages/web/src/main.rs`:
- Add `NowPlaying` to imports
- Add route: `#[route("/now-playing")] NowPlaying {},`
- In `AppLayout`, detect route and pass props to AppShell:

```rust
#[component]
fn AppLayout() -> Element {
    use_player_state_provider();
    let nav = navigator();
    let route = use_route::<Route>();
    let on_now_playing = matches!(route, Route::NowPlaying {});

    rsx! {
        AppShell {
            player_bar_hidden: on_now_playing,
            on_player_expand: move |_| { nav.push(Route::NowPlaying {}); },
            sidebar: rsx! { /* existing sidebar */ },
            Outlet::<Route> {}
        }
    }
}
```

- [ ] **Step 4: Verify**

Run: `cargo check -p dioxus_music_web`

- [ ] **Step 5: Commit**

```bash
git add -A packages/web/
git commit -m "feat(dioxus_music): add Now Playing view with two-column layout (web)"
```

---

## Task 5: Create Now Playing view + route (desktop)

**Files:**
- Create: `packages/desktop/src/views/now_playing.rs`
- Modify: `packages/desktop/src/views/mod.rs`
- Modify: `packages/desktop/src/main.rs`

- [ ] **Step 1: Create desktop Now Playing view**

Create `packages/desktop/src/views/now_playing.rs` — same structure as web version. Can share the CSS by loading it from web assets, or duplicate the styles inline. Simplest: use inline styles (since the desktop crate doesn't have the web crate's CSS files) or create a desktop-specific CSS file.

**Decision:** Create `packages/desktop/assets/now_playing.css` with the same content as the web version. This duplicates CSS but keeps the pattern consistent (each platform has its own assets). The desktop views already use inline styles but this view is complex enough to warrant a CSS file.

- [ ] **Step 2: Add route to desktop crate**

In `packages/desktop/src/views/mod.rs`: add `mod now_playing; pub use now_playing::NowPlaying;`

In `packages/desktop/src/main.rs`:
- Add `NowPlaying` to imports
- Add route: `#[route("/now-playing")] NowPlaying {},`
- In `DesktopLayout`, detect route and pass props to AppShell (same pattern as web)

- [ ] **Step 3: Verify**

Run: `cargo check -p dioxus_music_desktop`

- [ ] **Step 4: Commit**

```bash
git add -A packages/desktop/
git commit -m "feat(dioxus_music): add Now Playing view with two-column layout (desktop)"
```

---

## Task 6: Polish + verify

**Files:**
- Various CSS/RS files

- [ ] **Step 1: Visual verification**

Run `dx serve --package dioxus_music_web` or `dx serve --package dioxus_music_desktop`.

Check:
- Click a track in Library → track appears in PlayerBar
- Click track info in PlayerBar → navigates to `/now-playing`
- Now Playing shows art, title, controls, metadata, queue
- Transport controls work (play/pause, next, prev, shuffle, repeat)
- Back button returns to previous view
- PlayerBar is hidden on Now Playing route
- Audio continues playing across route changes
- Responsive: narrow window shows single column, no queue/volume

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -p dioxus_music_ui -p dioxus_music_web -p dioxus_music_desktop`

Fix any warnings.

- [ ] **Step 3: Commit**

```bash
git add -A dioxus_music/
git commit -m "style(dioxus_music): polish Now Playing view, clippy cleanup"
```

---

## Summary

| Task | Description | Key Files |
|---|---|---|
| 1 | Extract audio element into shared module | `ui/src/audio.rs`, `player_bar.rs` |
| 2 | Wire audio into AppShell + PlayerBar props | `app_shell.rs`, `player_bar.rs` |
| 3 | Add audio to MobileLayout | `mobile/src/main.rs` |
| 4 | Now Playing view + route (web) | `web/src/views/now_playing.rs`, CSS, routes |
| 5 | Now Playing view + route (desktop) | `desktop/src/views/now_playing.rs`, routes |
| 6 | Polish + verify | Various |

Tasks 1-3 are sequential (audio extraction first). Tasks 4-5 are independent (web and desktop). Task 6 is cleanup after everything.

**Note:** Mobile does NOT get a `/now-playing` route (spec §2). Only Task 3 (audio element) touches the mobile crate.
