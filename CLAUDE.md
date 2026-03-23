# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Environment

This repo uses a Nix flake for development dependencies. Enter the devshell with:

```bash
nix develop
```

The devshell provides: Rust nightly toolchain, Dioxus CLI (`dx`), Android SDK/NDK, wasm-bindgen, diesel CLI, and other build dependencies. Without Nix, install these manually (see README.md).

## Build & Serve Commands

```bash
# Serve with hot-reload (from repo root)
dx serve --package discord_bot
dx serve --package game_manager
dx serve --package httpui
dx serve --package dioxus_music_web
dx serve --package dioxus_music_desktop
dx serve --package dioxus_music_mobile --platform android
dx serve --package dioxus_music_mobile --platform ios

# Workspace-level checks
cargo check --workspace
cargo clippy --workspace

# Format
nix fmt                     # Runs nixfmt, rustfmt, prettier via treefmt
```

There are no tests currently configured in any project.

## Dioxus 0.7 Conventions

See `AGENTS.md` at the repo root for the complete Dioxus 0.7 API reference. Critical rules:

- **No `cx`, `Scope`, or `use_state`** — these are removed in 0.7
- Components use `#[component]` macro; props must be `Clone + PartialEq` and owned (`String` not `&str`)
- State: `use_signal()` for local, `use_memo()` for derived, `use_context_provider()`/`use_context()` for shared
- Signal reads: `count()` or `.read()`; writes: `*count.write()` or `.with_mut()`
- Async: `use_resource()` for client-side, `use_server_future()` for SSR-compatible data fetching
- RSX: prefer `for` loops over `.map()` iterators; conditionals with `if` directly in RSX
- Assets: `asset!("/assets/...")` macro; CSS via `document::Link { rel: "stylesheet", href: ... }`

## Clippy Configuration

`dioxus_music/clippy.toml` prevents holding `GenerationalRef`, `GenerationalRefMut`, and `WriteLock` across await points. This avoids deadlocks — never hold a signal `.read()` or `.write()` borrow across an `.await`. Pattern: read into a local variable, drop the borrow, then await.

## Workspace Architecture

This is a Cargo workspace monorepo containing multiple independent Dioxus projects. All projects target Dioxus 0.7 on Rust nightly.

### Projects

| Package          | Type            | Description                                                                  |
| ---------------- | --------------- | ---------------------------------------------------------------------------- |
| `discord_bot`    | fullstack (web) | Discord bot with web UI — Poise/Serenity bot + Axum server + Dioxus frontend |
| `game_manager`   | fullstack (web) | Game server manager — Minecraft/Terraria status APIs + JWT auth              |
| `httpui`         | desktop         | HTTP client app (Postman-like) — the actively developed version              |
| `dioxus_music_*` | multi-platform  | Music player — web (fullstack), desktop, and mobile (Android/iOS)            |

### Fullstack Compilation Pattern

Every fullstack crate uses the same feature-flag split:

```toml
[features]
web    = ["dioxus/web"]
server = ["dioxus/server", "dep:axum", "dep:tokio", ...]
```

The `dx` CLI compiles the crate twice: once with `server` for the backend binary, once with `web`/`desktop`/`mobile` for the client. Server-only code is gated with `#[cfg(feature = "server")]`. Server functions (`#[server]`, `#[get]`, `#[post]`) become API endpoints on the server and HTTP client calls on the client.

### dioxus_music — Multi-Platform Architecture

The most architecturally complete project, following a 5-crate pattern:

```
dioxus_music/packages/
├── api/      # Server functions + Diesel models + DB pool + audio scanner + streaming handler
├── ui/       # Shared components (AppShell, Sidebar, PlayerBar, TrackList, PlaylistFormModal)
├── web/      # Web entry point — fullstack with routes, server startup, playlist views
├── desktop/  # Desktop entry point — minimal shell wrapping shared UI
└── mobile/   # Mobile entry point — mirrors desktop, targets Android/iOS
```

- **Platform crates** each define their own `Route` enum and layout wrapper. Views can diverge per platform.
- **`ui` crate** holds platform-agnostic components and CSS assets. All platform crates depend on it.
- **`api` crate** holds server functions, Diesel schema/models, the music file scanner (`walkdir` + `lofty`), and an Axum audio streaming handler. Platform crates depend on `api` through `ui`.
- **PlayerState** is shared global state via `use_context_provider`/`use_context`. It manages playback queue, shuffle (Fisher-Yates), repeat modes, and controls the `<audio>` element via `document::eval` JS interop.

### discord_bot — Server Architecture

The server main runs three concurrent tasks via `tokio::select!`:

1. **Axum** — serves the Dioxus fullstack app
2. **Poise/Serenity bot** — Discord slash commands for game server management
3. **Interval task** — periodic polling

Uses `OnceLock<AppState>` and `OnceLock<Pool>` for server-side global state accessible from both Axum handlers and Dioxus server functions.

### httpui — Component Architecture

Desktop-only app with a custom component library: generic `Select<T>`, `Accordion`, `Button` variants, `Input`, `Separator`. All state is in-memory `Signal<Vec<T>>` (no persistence). Data model: Spaces > Collections > Requests. macOS-specific: transparent titlebar with fullsize content view.

## Database Projects

`discord_bot` and `dioxus_music` use PostgreSQL via Diesel (async with `diesel-async` + `bb8` or `deadpool`). Migrations are embedded and run at startup. The `dioxus_music_web` server also registers an Axum route at `/stream/{track_id}` for audio file streaming.

To run database projects, you need a running PostgreSQL instance and a `DATABASE_URL` environment variable (typically set in `.env`, which is gitignored and loaded by `.envrc`).
