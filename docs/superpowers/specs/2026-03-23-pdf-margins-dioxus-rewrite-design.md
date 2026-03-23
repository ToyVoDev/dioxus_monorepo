# PDF Margins — Dioxus/WASM Rewrite

**Date:** 2026-03-23
**Status:** Draft
**Replaces:** `pdf_margins/` (React + pdf-lib)

## Overview

Rewrite the pdf_margins utility from React/TypeScript/pdf-lib to Dioxus 0.7/Rust/lopdf. The app centers PDF content onto a target paper size (defaulting to US Letter 8.5x11") for double-sided printing alignment. All processing runs client-side in WASM — no server required.

## Goals

- Feature parity with the React version (all controls, previews, options)
- Fix two existing bugs (see Bugfixes section)
- Pure Rust PDF processing via `lopdf` instead of JavaScript pdf-lib
- Deploy as a static site (same as current)

## Non-Goals

- Server-side processing
- Desktop or mobile targets
- New features beyond 1:1 parity + bugfix

## Project Structure

```
pdf_margins_dioxus/
├── Cargo.toml          # dioxus (web), lopdf, web-sys, js-sys
├── Dioxus.toml         # dx serve config
├── src/
│   ├── main.rs         # App entry, component, state, JS interop
│   └── center_pdf.rs   # Pure Rust centering logic
└── assets/
    └── style.css       # Layout styling
```

Single crate, web-only. Added as a workspace member in the root `Cargo.toml`.

### Dependencies

| Crate | Purpose |
|-------|---------|
| `dioxus` (feature: `web`) | UI framework, WASM target |
| `lopdf` (`default-features = false`, feature: `wasm_js`) | PDF loading, manipulation, XObject embedding. Default features disabled to avoid `rayon` (no WASM threads) and `chrono`/`jiff` clock features. `wasm_js` enables `getrandom` WASM support. |
| `web-sys` | File API, Blob, URL.createObjectURL |
| `js-sys` | Uint8Array conversion for blob creation |
| `gloo-file` | Ergonomic file reading from `<input type="file">` |

## State Management

All state lives as Dioxus signals in the root `App` component:

| Signal | Type | Purpose |
|--------|------|---------|
| `original_bytes` | `Signal<Option<Vec<u8>>>` | Raw uploaded PDF bytes |
| `original_url` | `Signal<Option<String>>` | Blob URL for original preview iframe |
| `centered_url` | `Signal<Option<String>>` | Blob URL for centered preview iframe |
| `draw_alignment` | `Signal<bool>` | Toggle alignment corner lines |
| `draw_border` | `Signal<bool>` | Toggle content boundary rectangle |
| `nudge_x` | `Signal<f64>` | Horizontal content offset in points (default: 0.0) |
| `nudge_y` | `Signal<f64>` | Vertical content offset in points (default: 0.0) |
| `nudge_border_x` | `Signal<f64>` | Horizontal border offset in points (default: 7.0) |
| `nudge_border_y` | `Signal<f64>` | Vertical border offset in points (default: 7.0) |
| `paper_size` | `Signal<PaperSize>` | Selected target paper size |

## Data Flow

1. **Upload:** User selects PDF file. `web-sys` FileReader reads bytes into `original_bytes`. A blob URL is created for `original_url`.
2. **Process:** A `use_effect` watches `original_bytes` and all option signals. On change, calls `center_pdf()` (pure, synchronous) to produce new PDF bytes. Creates a blob URL for `centered_url`. Revokes the previous centered blob URL to avoid memory leaks. Note: processing is synchronous on the main thread — large PDFs may briefly freeze the UI. This is an acceptable tradeoff for v1; web worker offloading is future work.
3. **Preview:** Two `<iframe>` elements display the original and centered PDFs via their blob URLs.

## PDF Centering Algorithm (`center_pdf.rs`)

### Interface

```rust
pub struct CenterOptions {
    pub paper_size: (f64, f64),      // target (width, height) in PDF points
    pub draw_alignment: bool,
    pub draw_border: bool,
    pub nudge_x: f64,
    pub nudge_y: f64,
    pub nudge_border_x: f64,
    pub nudge_border_y: f64,
}

pub fn center_pdf(pdf_bytes: &[u8], options: &CenterOptions) -> Result<Vec<u8>, String>
```

### Per-Page Algorithm

1. **Load** source PDF via `lopdf::Document::load_mem(pdf_bytes)`
2. **Read MediaBox** of source page to get original `(width, height)`
3. **Orientation matching:** Compare source and target orientations. If both are landscape or both are portrait, keep target as-is. If they differ (one landscape, one portrait), swap the target dimensions to match the source orientation. This fixes a bug in the React version (see Bugfixes section).
4. **Calculate offsets:**
   ```
   x_offset = (target_width - source_width) / 2 + nudge_x
   y_offset = (target_height - source_height) / 2 + nudge_y
   ```
5. **Embed as Form XObject:** This is the most complex step — lopdf has no high-level `embedPage` API like pdf-lib, so we build it manually:
   - **Copy content streams:** Read the page's `/Contents` (may be a single stream or an array of streams). If multiple, concatenate them into one stream.
   - **Copy resources recursively:** Clone the page's `/Resources` dictionary, including all referenced fonts, images, color spaces, and nested XObjects. Remap all indirect object references from the source document to the new document using `lopdf::Document::renumber_objects()` or manual ID mapping.
   - **Handle encoded streams:** lopdf handles FlateDecode and other filters transparently when reading content streams.
   - **Build Form XObject:** Create an XObject stream with dictionary entries: `/Type /XObject`, `/Subtype /Form`, `/BBox [0 0 width height]` (from source MediaBox), `/Resources` (the copied resource dict). The stream data is the concatenated content stream bytes.
   - **Register in new page:** Add the Form XObject to the new page's `/Resources/XObject` dictionary under a name like `/SourcePage`.
6. **Draw on new page:** Create a new page with target MediaBox. Write content stream: `q {x_offset} {y_offset} cm /SourcePage Do Q` to place the embedded page at the calculated position.
7. **Optional alignment corner:** If `draw_alignment` is true, draw two perpendicular lines (L-shape) at the appropriate corner. Position depends on page orientation (matches React version behavior).
8. **Optional border:** If `draw_border` is true, draw a rectangle at the original content boundary, accounting for nudge offsets. 2pt stroke, no fill.
9. **Save** document to bytes and return.

### PaperSize Enum

Rust enum covering standard sizes, each mapping to `(width_pts, height_pts)`:

- Letter (612 x 792), Legal (612 x 1008), Tabloid (792 x 1224)
- A0 through A6
- B4, B5
- Additional sizes as present in pdf-lib's PageSizes

Implements `Display` for the select dropdown labels and `Copy + Clone + PartialEq` for Dioxus props compatibility.

## JS Interop

Two interop surfaces, kept minimal:

### File Reading

Use `gloo-file` for ergonomic file reading:
- Get `File` from the `<input type="file">` change event via `web-sys`
- Use `gloo_file::futures::read_as_bytes()` to asynchronously read into `Vec<u8>`

### Blob URL Management

Create and revoke blob URLs via `web-sys`:
- `web_sys::Blob::new_with_u8_array_sequence()` to create a blob from PDF bytes
- `web_sys::Url::create_object_url_with_blob()` to get the URL string
- `web_sys::Url::revoke_object_url()` to clean up old URLs

This avoids `document::eval` entirely — pure Rust bindings through `web-sys`.

## UI Layout

Single-page layout with plain HTML/CSS (no component library):

```
+------------------------------------------+
| [Upload PDF]                             |
| ☐ Draw Alignment  ☐ Draw Border         |
| Nudge X: [___]  Nudge Y: [___]          |
| Border X: [___]  Border Y: [___]        |
| Paper Size: [Letter ▼]                   |
+------------------+-----------------------+
| Original         | Centered              |
| [iframe]         | [iframe]              |
|                  |                       |
+------------------+-----------------------+
```

- Controls in a top/sidebar area
- Two iframes side by side filling remaining space
- Flexbox layout, minimal CSS

## Dioxus.toml

Minimal configuration:
- `name = "pdf_margins"`, `default_platform = "web"`
- Title: "PDF Margins"
- Web app dir and asset dir pointing to standard locations

## Error Handling

`center_pdf()` returns `Result<Vec<u8>, String>`. On error (malformed PDF, encrypted PDF, etc.), the UI displays the error message in the preview area instead of the iframe. The upload/original preview is unaffected.

## Bugfixes

Two bugs in the React version are fixed:

1. **`nudgeBorderHeight` assignment bug:** `CenterPDF.ts:20` assigns `nudgeBorderHeight` from `options?.nudgeBorderWidth` instead of `options?.nudgeBorderHeight`. The Rust rewrite uses named struct fields, eliminating this class of bug.

2. **Orientation matching bug:** `CenterPDF.ts:27-28` checks `bothLandscapeOrPortrait` but the condition (`oldPage.getWidth() > oldPage.getHeight() && paperSize[0] > paperSize[1]`) is only true when both are landscape. When both source and target are portrait, the variable is false and the target dimensions get swapped to landscape — incorrect behavior. The Rust rewrite properly checks whether orientations match (both landscape or both portrait) and only swaps when they differ.
