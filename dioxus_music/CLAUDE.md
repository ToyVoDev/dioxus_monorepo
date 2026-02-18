# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Serve Commands

This project uses the Dioxus CLI (`dx`). Install with `curl -sSL http://dioxus.dev/install.sh | sh`.

```bash
# Serve a platform (must cd into the package first)
cd packages/web && dx serve
cd packages/desktop && dx serve
cd packages/mobile && dx serve --platform android
cd packages/mobile && dx serve --platform ios

# Workspace-level checks
cargo check --workspace
cargo clippy --workspace
```

There are no tests currently configured.

## Architecture

This is a **Dioxus 0.7 fullstack multi-platform workspace** with 5 crates:

```
packages/
├── api/      # Shared server functions (e.g. #[post("/api/echo")])
├── ui/       # Shared UI components (Navbar, Hero, Echo) + assets
├── web/      # Web entry point, routes, and web-specific views
├── desktop/  # Desktop entry point, routes, and desktop-specific views
└── mobile/   # Mobile entry point, routes, and mobile-specific views
```

**Key architectural patterns:**

- **Platform crates** (web/desktop/mobile) each define their own `Route` enum and platform-specific layout wrapper (e.g. `WebNavbar`) that wraps the shared `Navbar` from `ui`. Views start identical but can diverge per platform.
- **`ui` crate** holds platform-agnostic components and their CSS assets. Platform crates depend on `ui`.
- **`api` crate** holds all server functions using `#[post]`/`#[get]` macros. Platform crates depend on `api` (through `ui`).
- **Fullstack compilation**: Each platform is compiled twice — once for the client and once for the server. The `server` feature flag controls this split. Server functions become API endpoints on the server and HTTP client calls on the client.
- **Assets**: Each crate has its own `assets/` directory. Use `asset!("/assets/...")` macro for compile-time asset linking.

## Dioxus 0.7 Conventions

See `AGENTS.md` for the complete Dioxus 0.7 API reference. Critical points:

- **No `cx`, `Scope`, or `use_state`** — these are removed in 0.7
- Components use `#[component]` macro; props must be `Clone + PartialEq` and owned (`String` not `&str`)
- State: `use_signal()` for local, `use_memo()` for derived, `use_context_provider()`/`use_context()` for shared
- Signal reads: `count()` or `.read()`; writes: `*count.write()` or `.with_mut()`
- Async: `use_resource()` for client-side, `use_server_future()` for SSR-compatible data fetching
- RSX: prefer `for` loops over `.map()` iterators; conditionals with `if` directly in RSX

## Clippy Configuration

`clippy.toml` prevents holding `GenerationalRef`, `GenerationalRefMut`, and `WriteLock` across await points. This avoids deadlocks in async code — never hold a signal read/write borrow across an `.await`.
