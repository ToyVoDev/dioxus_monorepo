# dioxus_music Redesign — Kinetic Obsidian

Decomposed into sub-projects, each gets its own spec → plan → implementation cycle.

## Sub-projects

- [x] **1. Theme + Layout Shell** — Adopt `kinetic_ui` in `packages/ui`, restyle AppShell/Sidebar/PlayerBar/Navbar, responsive breakpoints (web/desktop collapse to mobile-like at narrow widths), mobile bottom nav
- [x] **2. Library + Album Detail views** — Restyle library grid, album cards, tracklist table, album header with asymmetric layout. Based on good Stitch mockups (`library_overview/`, `album_detail/`)
- [x] **3. Now Playing view** — Full-screen player with album art, controls, progress bar, technical metadata, queue panel. Design from scratch (Stitch mockup was bad). Reference `now_playing/code.html` for intended content, ignore layout
- [ ] **4. Downloads/Sync Manager** — Storage breakdown, download queue, transcoding settings, synced library grid. Design from scratch (Stitch mockup was bad). Reference `downloads_sync/code.html` for intended content, ignore layout. Mostly future feature work — placeholder UI

## Reference

- Design system: `/DESIGN.md` (repo root)
- Stitch mockups: `docs/stitch/` (library_overview, now_playing, album_detail, downloads_sync)
- Existing plans: `docs/plans/` (offline-sync-and-automotive design + impl)
- Shared component library: `/kinetic_ui/` (already built for httpui)
