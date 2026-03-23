# PDF Margins — Dioxus/WASM Rewrite

**Date:** 2026-03-23
**Status:** Draft
**Replaces:** `pdf_margins/` (React + pdf-lib)

## Overview

Rewrite the pdf_margins utility from React/TypeScript/pdf-lib to Dioxus 0.7/Rust/lopdf. The app centers PDF content onto a target paper size (defaulting to US Letter 8.5x11") for double-sided printing alignment. All processing runs client-side in WASM — no server required.

## Goals

- Feature parity with the React version (all controls, previews, options)
- Fix the existing `nudgeBorderHeight` bug (reads from `nudgeBorderWidth` in React source)
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
| `lopdf` | PDF loading, manipulation, XObject embedding |
| `web-sys` | File API, Blob, URL.createObjectURL |
| `js-sys` | Uint8Array conversion for blob creation |

## State Management

All state lives as Dioxus signals in the root `App` component:

| Signal | Type | Purpose |
|--------|------|---------|
| `original_bytes` | `Signal<Option<Vec<u8>>>` | Raw uploaded PDF bytes |
| `original_url` | `Signal<Option<String>>` | Blob URL for original preview iframe |
| `centered_url` | `Signal<Option<String>>` | Blob URL for centered preview iframe |
| `draw_alignment` | `Signal<bool>` | Toggle alignment corner lines |
| `draw_border` | `Signal<bool>` | Toggle content boundary rectangle |
| `nudge_x` | `Signal<f64>` | Horizontal content offset (points) |
| `nudge_y` | `Signal<f64>` | Vertical content offset (points) |
| `nudge_border_x` | `Signal<f64>` | Horizontal border offset (points) |
| `nudge_border_y` | `Signal<f64>` | Vertical border offset (points) |
| `paper_size` | `Signal<PaperSize>` | Selected target paper size |

## Data Flow

1. **Upload:** User selects PDF file. `web-sys` FileReader reads bytes into `original_bytes`. A blob URL is created for `original_url`.
2. **Process:** A `use_effect` watches `original_bytes` and all option signals. On change, calls `center_pdf()` (pure, synchronous) to produce new PDF bytes. Creates a blob URL for `centered_url`. Revokes the previous centered blob URL to avoid memory leaks.
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
3. **Orientation matching:** If source is landscape (w > h), swap target dimensions so the new page also uses landscape. If both are portrait (or both landscape), keep target as-is.
4. **Calculate offsets:**
   ```
   x_offset = (target_width - source_width) / 2 + nudge_x
   y_offset = (target_height - source_height) / 2 + nudge_y
   ```
5. **Embed as Form XObject:** Extract the source page's content stream and resources. Create a Form XObject (Type: XObject, Subtype: Form, BBox: original MediaBox) in the new document.
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

Implements `Display` for the select dropdown labels and `PartialEq + Clone` for Dioxus props.

## JS Interop

Two interop surfaces, kept minimal:

### File Reading

Use `web-sys` to access the `<input type="file">` change event:
- Get `File` from `FileList`
- Use `FileReader.readAsArrayBuffer()` or the `gloo-file` crate
- Convert `ArrayBuffer` → `Uint8Array` → `Vec<u8>`

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

## Bugfix

The React version at `CenterPDF.ts:20` assigns `nudgeBorderHeight` from `options?.nudgeBorderWidth` instead of `options?.nudgeBorderHeight`. The Rust rewrite uses named struct fields, eliminating this class of bug.
