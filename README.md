# dx_monorepo

Monorepo of my various Dioxus projects.

## Projects

| Package | Platform | Description |
|---------|----------|-------------|
| `discord_bot` | web (fullstack) | Discord bot with Dioxus web UI |
| `game_manager` | web (fullstack) | Game server manager |
| `httpui` | desktop | HTTP client (Postman-like) |
| `dioxus_music_web` | web (fullstack) | Music player web app |
| `dioxus_music_desktop` | desktop | Music player desktop app |
| `dioxus_music_mobile` | android/ios | Music player mobile app |

## Prerequisites

### Nix (recommended)

This project uses a Nix flake for development dependencies. Enter the devshell:

```bash
nix develop --impure
```

The `--impure` flag is required because the Android SDK has an unfree license. You may also need:

```bash
export NIXPKGS_ALLOW_UNFREE=1
export NIXPKGS_ACCEPT_ANDROID_SDK_LICENSE=1
```

The devshell provides: Rust toolchain, Dioxus CLI (`dx`), Android SDK/NDK, wasm-bindgen, diesel CLI, and other build dependencies.

### Without Nix

Install the following manually:

- [Rust](https://rustup.rs/) (see `rust-toolchain.toml` for the required toolchain)
- [Dioxus CLI](https://dioxus.dev): `curl -sSL http://dioxus.dev/install.sh | sh`
- For Android: Android SDK + NDK, set `ANDROID_HOME` and `ANDROID_NDK_HOME`
- For database projects: `diesel_cli` with postgres support, a running PostgreSQL instance

## iOS Builds (macOS only)

iOS builds require Xcode installed on the host machine. The Nix devshell handles SDK discovery automatically, but you must complete these one-time setup steps:

1. **Install Xcode** from the Mac App Store or [developer.apple.com](https://developer.apple.com/xcode/)

2. **Accept the Xcode license:**
   ```bash
   sudo xcodebuild -license
   ```

3. **Verify iOS SDKs are available** (from within the devshell):
   ```bash
   xcrun --show-sdk-path --sdk iphonesimulator
   xcrun --show-sdk-path --sdk iphoneos
   ```

The devshell unsets `SDKROOT` and sets `DEVELOPER_DIR` to Xcode.app so that `xcrun` can discover the iOS SDKs even if `xcode-select` points at CommandLineTools.

## Serving / Building

```bash
# Serve with hot-reload (from repo root)
dx serve --package <package_name> --platform <platform>

# Build
dx build --package <package_name> --platform <platform>
```

Examples:

```bash
# Web
dx serve --package dioxus_music_web --platform web

# Desktop
dx serve --package dioxus_music_desktop --platform desktop
dx serve --package httpui --platform desktop

# Mobile
dx build --package dioxus_music_mobile --platform android
dx build --package dioxus_music_mobile --platform ios
```
