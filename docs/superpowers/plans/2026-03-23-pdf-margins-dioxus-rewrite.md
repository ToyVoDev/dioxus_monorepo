# PDF Margins Dioxus Rewrite — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite the pdf_margins React app as a Dioxus 0.7 web (WASM) app using lopdf for PDF centering.

**Architecture:** Single-crate, client-side-only Dioxus web app. Pure Rust `center_pdf` module handles PDF manipulation via lopdf (Form XObject embedding). UI is a single `App` component with signals for state, `use_effect` for reactive processing, and `web-sys` for blob URL management.

**Tech Stack:** Dioxus 0.7 (web), lopdf (WASM), web-sys, js-sys, gloo-file

**Spec:** `docs/superpowers/specs/2026-03-23-pdf-margins-dioxus-rewrite-design.md`

---

## File Map

| File | Responsibility |
|------|---------------|
| `Cargo.toml` (workspace root) | Add `pdf_margins_dioxus` to workspace members |
| `pdf_margins_dioxus/Cargo.toml` | Package manifest with dependencies |
| `pdf_margins_dioxus/Dioxus.toml` | dx CLI config (name, title, web settings) |
| `pdf_margins_dioxus/src/main.rs` | App entry, component, signals, file upload, blob URL interop, UI |
| `pdf_margins_dioxus/src/center_pdf.rs` | `PaperSize` enum, `CenterOptions` struct, `center_pdf()` function |
| `pdf_margins_dioxus/assets/style.css` | Flexbox layout, controls styling, iframe sizing |

---

### Task 1: Project Scaffolding

**Files:**
- Modify: `Cargo.toml` (workspace root, line 15)
- Create: `pdf_margins_dioxus/Cargo.toml`
- Create: `pdf_margins_dioxus/Dioxus.toml`
- Create: `pdf_margins_dioxus/src/main.rs`
- Create: `pdf_margins_dioxus/assets/style.css`

- [ ] **Step 1: Create the package directory structure**

```bash
mkdir -p pdf_margins_dioxus/src pdf_margins_dioxus/assets
```

- [ ] **Step 2: Create `pdf_margins_dioxus/Cargo.toml`**

```toml
[package]
name = "pdf_margins_dioxus"
version = "0.1.0"
authors = ["Collin Diekvoss <Collin@Diekvoss.com>"]
edition = "2024"

[dependencies]
dioxus = { workspace = true }
lopdf = { version = "0.40", default-features = false, features = ["wasm_js"] }
web-sys = { workspace = true, features = [
    "Blob",
    "BlobPropertyBag",
    "File",
    "FileList",
    "FileReader",
    "HtmlInputElement",
    "Url",
] }
js-sys = "0.3"
gloo-file = { version = "0.3", features = ["futures"] }
wasm-bindgen-futures = "0.4"
tracing = { workspace = true }
dioxus-logger = { workspace = true }

[features]
default = ["web"]
web = ["dioxus/web"]

[profile]

[profile.wasm-dev]
inherits = "dev"
opt-level = 1

[profile.server-dev]
inherits = "dev"

[profile.android-dev]
inherits = "dev"
```

- [ ] **Step 3: Create `pdf_margins_dioxus/Dioxus.toml`**

```toml
[application]
name = "pdf_margins_dioxus"

[web.app]
title = "PDF Margins"

[web.resource]
style = []
script = []

[web.resource.dev]
script = []

[web.watcher]
index_on_404 = true
```

- [ ] **Step 4: Create minimal `pdf_margins_dioxus/src/main.rs`**

```rust
mod center_pdf;

use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "PDF Margins" }
        p { "Coming soon..." }
    }
}
```

- [ ] **Step 5: Create empty `pdf_margins_dioxus/src/center_pdf.rs`**

```rust
// PDF centering logic using lopdf
```

- [ ] **Step 6: Create `pdf_margins_dioxus/assets/style.css`**

```css
html, body {
    margin: 0;
    padding: 0;
    height: 100%;
    font-family: system-ui, -apple-system, sans-serif;
}

#main {
    display: flex;
    flex-direction: column;
    height: 100vh;
}
```

- [ ] **Step 7: Add to workspace members**

In the root `Cargo.toml`, add `"pdf_margins_dioxus"` to the `members` array.

- [ ] **Step 8: Verify it compiles**

Run: `dx serve --package pdf_margins_dioxus`
Expected: App builds and serves, shows "PDF Margins" heading in browser.

- [ ] **Step 9: Commit**

```bash
git add pdf_margins_dioxus/ Cargo.toml
git commit -m "feat: scaffold pdf_margins_dioxus project"
```

---

### Task 2: PaperSize Enum and CenterOptions Struct

**Files:**
- Modify: `pdf_margins_dioxus/src/center_pdf.rs`

- [ ] **Step 1: Implement `PaperSize` enum**

Write the `PaperSize` enum with all standard sizes matching pdf-lib's `PageSizes`. Each variant maps to `(f64, f64)` in PDF points. Reference values (pdf-lib uses the same PDF standard points):

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaperSize {
    Letter,
    Legal,
    Tabloid,
    Ledger,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    B4,
    B5,
    FourA0,
    TwoA0,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Executive,
    Folio,
    GovernmentLetter,
    JuniorLegal,
}

impl PaperSize {
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            Self::Letter => (612.0, 792.0),
            Self::Legal => (612.0, 1008.0),
            Self::Tabloid => (792.0, 1224.0),
            Self::Ledger => (1224.0, 792.0),
            Self::A0 => (2383.94, 3370.39),
            Self::A1 => (1683.78, 2383.94),
            Self::A2 => (1190.55, 1683.78),
            Self::A3 => (841.89, 1190.55),
            Self::A4 => (595.28, 841.89),
            Self::A5 => (419.53, 595.28),
            Self::A6 => (297.64, 419.53),
            Self::B4 => (708.66, 1000.63),
            Self::B5 => (498.90, 708.66),
            Self::FourA0 => (4767.87, 6740.79),
            Self::TwoA0 => (3370.39, 4767.87),
            Self::C0 => (2599.37, 3676.54),
            Self::C1 => (1836.85, 2599.37),
            Self::C2 => (1298.27, 1836.85),
            Self::C3 => (918.43, 1298.27),
            Self::C4 => (649.13, 918.43),
            Self::C5 => (459.21, 649.13),
            Self::C6 => (323.15, 459.21),
            Self::C7 => (229.61, 323.15),
            Self::C8 => (161.57, 229.61),
            Self::C9 => (113.39, 161.57),
            Self::C10 => (79.37, 113.39),
            Self::Executive => (521.86, 756.0),
            Self::Folio => (612.0, 936.0),
            Self::GovernmentLetter => (576.0, 756.0),
            Self::JuniorLegal => (360.0, 576.0),
        }
    }

    pub const ALL: &[PaperSize] = &[
        Self::Letter, Self::Legal, Self::Tabloid, Self::Ledger,
        Self::A0, Self::A1, Self::A2, Self::A3, Self::A4, Self::A5, Self::A6,
        Self::B4, Self::B5,
        Self::FourA0, Self::TwoA0,
        Self::C0, Self::C1, Self::C2, Self::C3, Self::C4, Self::C5,
        Self::C6, Self::C7, Self::C8, Self::C9, Self::C10,
        Self::Executive, Self::Folio, Self::GovernmentLetter, Self::JuniorLegal,
    ];
}

impl fmt::Display for PaperSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FourA0 => write!(f, "4A0"),
            Self::TwoA0 => write!(f, "2A0"),
            Self::GovernmentLetter => write!(f, "Government Letter"),
            Self::JuniorLegal => write!(f, "Junior Legal"),
            other => write!(f, "{:?}", other),
        }
    }
}
```

- [ ] **Step 2: Add `CenterOptions` struct**

Append to `center_pdf.rs`:

```rust
pub struct CenterOptions {
    pub paper_size: (f64, f64),
    pub draw_alignment: bool,
    pub draw_border: bool,
    pub nudge_x: f64,
    pub nudge_y: f64,
    pub nudge_border_x: f64,
    pub nudge_border_y: f64,
}
```

- [ ] **Step 3: Add stub `center_pdf` function**

```rust
pub fn center_pdf(pdf_bytes: &[u8], options: &CenterOptions) -> Result<Vec<u8>, String> {
    // TODO: implement in Task 3
    Err("not yet implemented".to_string())
}
```

- [ ] **Step 4: Verify it compiles**

Run: `cargo check --package pdf_margins_dioxus`
Expected: Compiles with no errors (warning about unused is fine).

- [ ] **Step 5: Commit**

```bash
git add pdf_margins_dioxus/src/center_pdf.rs
git commit -m "feat: add PaperSize enum and CenterOptions struct"
```

---

### Task 3: Core PDF Centering Logic (lopdf)

**Files:**
- Modify: `pdf_margins_dioxus/src/center_pdf.rs`

This is the most complex task. The `center_pdf` function must load a PDF, embed each page as a Form XObject on a new larger page, and optionally draw alignment/border lines.

- [ ] **Step 1: Implement page embedding helper**

Replace the stub `center_pdf` function with the full implementation. Key steps per page:

```rust
use lopdf::{Document, Object, ObjectId, Stream, Dictionary};
use std::collections::BTreeMap;

pub fn center_pdf(pdf_bytes: &[u8], options: &CenterOptions) -> Result<Vec<u8>, String> {
    let source_doc = Document::load_mem(pdf_bytes).map_err(|e| format!("Failed to load PDF: {e}"))?;
    let mut new_doc = Document::with_version("1.7");

    // Initialize page tree — Document::with_version creates an empty doc with no catalog
    let mut pages_tree = Dictionary::new();
    pages_tree.set("Type", Object::Name(b"Pages".to_vec()));
    pages_tree.set("Kids", Object::Array(vec![]));
    pages_tree.set("Count", Object::Integer(0));
    let pages_id = new_doc.add_object(pages_tree);

    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_id));
    let catalog_id = new_doc.add_object(catalog);
    new_doc.trailer.set("Root", Object::Reference(catalog_id));

    let pages = source_doc.get_pages();
    let mut sorted_pages: Vec<(u32, ObjectId)> = pages.into_iter().collect();
    sorted_pages.sort_by_key(|(num, _)| *num);

    for (_page_num, page_id) in &sorted_pages {
        let page_dict = source_doc
            .get_dictionary(*page_id)
            .map_err(|e| format!("Failed to get page dict: {e}"))?;

        // Read MediaBox to get source dimensions
        let media_box = get_media_box(&source_doc, page_dict)?;
        let source_width = media_box[2] - media_box[0];
        let source_height = media_box[3] - media_box[1];

        // Orientation matching
        let (target_width, target_height) = {
            let (pw, ph) = options.paper_size;
            let source_landscape = source_width > source_height;
            let target_landscape = pw > ph;
            if source_landscape == target_landscape {
                (pw, ph)
            } else {
                (ph, pw)
            }
        };

        // Calculate centering offsets
        let x_offset = (target_width - source_width) / 2.0 + options.nudge_x;
        let y_offset = (target_height - source_height) / 2.0 + options.nudge_y;

        // Embed source page as Form XObject in the new document
        let xobject_id = embed_page_as_xobject(&source_doc, &mut new_doc, *page_id, &media_box)?;

        // Build content stream for the new page
        let mut content = format!("q {x_offset} {y_offset} cm /SourcePage Do Q\n");

        // Optional alignment corner
        if options.draw_alignment {
            let is_landscape = target_width > target_height;
            if is_landscape {
                // Top-left L-shape
                content += &format!(
                    "2 w 10 {} m 30 {} l S\n",
                    target_height - 10.0,
                    target_height - 10.0
                );
                content += &format!(
                    "2 w 10 {} m 10 {} l S\n",
                    target_height - 30.0,
                    target_height - 10.0
                );
            } else {
                // Bottom-left L-shape
                content += "2 w 10 10 m 30 10 l S\n";
                content += "2 w 10 30 m 10 10 l S\n";
            }
        }

        // Optional border
        if options.draw_border {
            let bx = x_offset + options.nudge_border_x;
            let by = y_offset + options.nudge_border_y;
            let bw = source_width - options.nudge_border_x * 2.0;
            let bh = source_height - options.nudge_border_y * 2.0;
            content += &format!("2 w {bx} {by} {bw} {bh} re S\n");
        }

        let content_stream = Stream::new(Dictionary::new(), content.into_bytes());
        let content_id = new_doc.add_object(content_stream);

        // Build XObject resources dict for the new page
        let mut xobject_dict = Dictionary::new();
        xobject_dict.set("SourcePage", Object::Reference(xobject_id));

        let mut resources_dict = Dictionary::new();
        resources_dict.set("XObject", Object::Dictionary(xobject_dict));

        // Create new page dictionary
        let mut new_page_dict = Dictionary::new();
        new_page_dict.set("Type", Object::Name(b"Page".to_vec()));
        new_page_dict.set(
            "MediaBox",
            Object::Array(vec![
                Object::Real(0.0),
                Object::Real(0.0),
                Object::Real(target_width as f32),
                Object::Real(target_height as f32),
            ]),
        );
        new_page_dict.set("Contents", Object::Reference(content_id));
        new_page_dict.set("Resources", Object::Dictionary(resources_dict));

        // Set Parent before adding to doc
        new_page_dict.set("Parent", Object::Reference(pages_id));
        let new_page_id = new_doc.add_object(new_page_dict);

        // Add page to the page tree (pages_id was captured at doc init)
        if let Ok(pages_dict) = new_doc.get_dictionary_mut(pages_id) {
            if let Ok(kids) = pages_dict.get_mut(b"Kids") {
                if let Object::Array(ref mut arr) = kids {
                    arr.push(Object::Reference(new_page_id));
                }
            }
            let count = pages_dict
                .get(b"Count")
                .and_then(|c| c.as_i64().ok())
                .unwrap_or(0);
            pages_dict.set("Count", Object::Integer(count + 1));
        }
    }

    let mut buf = Vec::new();
    new_doc.save_to(&mut buf).map_err(|e| format!("Failed to save PDF: {e}"))?;
    Ok(buf)
}
```

Note: The lopdf API at version 0.40 may have minor differences from what's shown (e.g., method signatures, error types). Consult `lopdf` docs if a specific call doesn't compile. The core approach (manual catalog/pages tree, Form XObject embedding, object remapping) is correct for lopdf's architecture. Key types: `Object::Real` wraps `f32` (not `f64`) — all f64→f32 casts are included above.

- [ ] **Step 2: Implement `get_media_box` helper**

```rust
fn get_media_box(doc: &Document, page_dict: &Dictionary) -> Result<[f64; 4], String> {
    let media_box = page_dict
        .get(b"MediaBox")
        .or_else(|_| {
            // Walk up parent chain if MediaBox is inherited
            page_dict
                .get(b"Parent")
                .and_then(|p| p.as_reference())
                .and_then(|parent_id| doc.get_dictionary(parent_id))
                .and_then(|parent| parent.get(b"MediaBox"))
        })
        .map_err(|_| "No MediaBox found")?;

    if let Object::Array(arr) = media_box {
        if arr.len() == 4 {
            let vals: Result<Vec<f64>, _> = arr
                .iter()
                .map(|v| match v {
                    Object::Real(r) => Ok(*r as f64),
                    Object::Integer(i) => Ok(*i as f64),
                    _ => Err("Invalid MediaBox value"),
                })
                .collect();
            let vals = vals?;
            return Ok([vals[0], vals[1], vals[2], vals[3]]);
        }
    }
    Err("Invalid MediaBox format".to_string())
}
```

- [ ] **Step 3: Implement `embed_page_as_xobject` helper**

This extracts a source page's content and resources, wraps them in a Form XObject:

```rust
fn embed_page_as_xobject(
    source_doc: &Document,
    new_doc: &mut Document,
    page_id: ObjectId,
    media_box: &[f64; 4],
) -> Result<ObjectId, String> {
    let page_dict = source_doc
        .get_dictionary(page_id)
        .map_err(|e| format!("Failed to get page: {e}"))?;

    // Collect content stream bytes
    let content_bytes = get_page_content_bytes(source_doc, page_dict)?;

    // Get resources (may be direct or indirect)
    let resources = page_dict
        .get(b"Resources")
        .cloned()
        .unwrap_or(Object::Dictionary(Dictionary::new()));

    // Deep-clone all referenced objects from source doc into new doc
    // using add_object to get safe IDs, and build a remap table
    let mut id_remap = BTreeMap::new();
    collect_and_remap_objects(source_doc, new_doc, &resources, &mut id_remap);

    // Remap references in the resources object
    let remapped_resources = remap_object_refs(&resources, &id_remap);

    // Build Form XObject dictionary
    let mut xobject_dict = Dictionary::new();
    xobject_dict.set("Type", Object::Name(b"XObject".to_vec()));
    xobject_dict.set("Subtype", Object::Name(b"Form".to_vec()));
    xobject_dict.set(
        "BBox",
        Object::Array(vec![
            Object::Real(media_box[0] as f32),
            Object::Real(media_box[1] as f32),
            Object::Real(media_box[2] as f32),
            Object::Real(media_box[3] as f32),
        ]),
    );
    xobject_dict.set("Resources", remapped_resources);

    let xobject_stream = Stream::new(xobject_dict, content_bytes);
    let xobject_id = new_doc.add_object(xobject_stream);

    Ok(xobject_id)
}

fn get_page_content_bytes(doc: &Document, page_dict: &Dictionary) -> Result<Vec<u8>, String> {
    let contents = page_dict
        .get(b"Contents")
        .map_err(|_| "No Contents in page")?;

    match contents {
        Object::Reference(id) => {
            let stream = doc
                .get_object(*id)
                .and_then(|o| o.as_stream().map_err(|_| lopdf::Error::ObjectNotFound))
                .map_err(|e| format!("Failed to get content stream: {e}"))?;
            stream
                .decompressed_content()
                .map_err(|e| format!("Failed to decompress: {e}"))
        }
        Object::Array(arr) => {
            let mut combined = Vec::new();
            for item in arr {
                let id = item.as_reference().map_err(|_| "Non-reference in Contents array")?;
                let stream = doc
                    .get_object(id)
                    .and_then(|o| o.as_stream().map_err(|_| lopdf::Error::ObjectNotFound))
                    .map_err(|e| format!("Failed to get content stream: {e}"))?;
                let bytes = stream
                    .decompressed_content()
                    .map_err(|e| format!("Failed to decompress: {e}"))?;
                combined.extend_from_slice(&bytes);
                combined.push(b'\n');
            }
            Ok(combined)
        }
        _ => Err("Unexpected Contents type".to_string()),
    }
}

/// Recursively copy referenced objects from source to new doc, building an ID remap table.
fn collect_and_remap_objects(
    source: &Document,
    dest: &mut Document,
    obj: &Object,
    remap: &mut BTreeMap<ObjectId, ObjectId>,
) {
    match obj {
        Object::Reference(id) => {
            if !remap.contains_key(id) {
                if let Ok(resolved) = source.get_object(*id) {
                    // Reserve a slot to prevent infinite recursion
                    let new_id = dest.add_object(Object::Null);
                    remap.insert(*id, new_id);
                    // Recurse into the object's references
                    collect_and_remap_objects(source, dest, resolved, remap);
                    // Now remap refs within the object and store the final version
                    let remapped = remap_object_refs(resolved, remap);
                    dest.objects.insert(new_id, remapped);
                }
            }
        }
        Object::Array(arr) => {
            for item in arr {
                collect_and_remap_objects(source, dest, item, remap);
            }
        }
        Object::Dictionary(dict) => {
            for (_, value) in dict.iter() {
                collect_and_remap_objects(source, dest, value, remap);
            }
        }
        Object::Stream(stream) => {
            for (_, value) in stream.dict.iter() {
                collect_and_remap_objects(source, dest, value, remap);
            }
        }
        _ => {}
    }
}

/// Remap all Object::Reference IDs in an object tree using the remap table.
fn remap_object_refs(obj: &Object, remap: &BTreeMap<ObjectId, ObjectId>) -> Object {
    match obj {
        Object::Reference(id) => {
            Object::Reference(*remap.get(id).unwrap_or(id))
        }
        Object::Array(arr) => {
            Object::Array(arr.iter().map(|item| remap_object_refs(item, remap)).collect())
        }
        Object::Dictionary(dict) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in dict.iter() {
                new_dict.set(key.clone(), remap_object_refs(value, remap));
            }
            Object::Dictionary(new_dict)
        }
        Object::Stream(stream) => {
            let mut new_dict = Dictionary::new();
            for (key, value) in stream.dict.iter() {
                new_dict.set(key.clone(), remap_object_refs(value, remap));
            }
            Object::Stream(Stream::new(new_dict, stream.content.clone()))
        }
        other => other.clone(),
    }
}
```

- [ ] **Step 4: Verify it compiles**

Run: `cargo check --package pdf_margins_dioxus`
Expected: Compiles with no errors.

- [ ] **Step 5: Commit**

```bash
git add pdf_margins_dioxus/src/center_pdf.rs
git commit -m "feat: implement PDF centering logic with lopdf"
```

---

### Task 4: UI — File Upload and Blob URL Interop

**Files:**
- Modify: `pdf_margins_dioxus/src/main.rs`

- [ ] **Step 1: Add all signal state and file upload handler**

Replace `main.rs` with the full state setup and file upload:

```rust
mod center_pdf;

use center_pdf::{CenterOptions, PaperSize};
use dioxus::prelude::*;
use gloo_file::futures::read_as_bytes;
use gloo_file::File as GlooFile;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Blob, BlobPropertyBag, Url, HtmlInputElement};
use js_sys::{Array, Uint8Array};

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

/// Create a blob URL from PDF bytes. Returns the URL string.
fn create_pdf_blob_url(bytes: &[u8]) -> Result<String, String> {
    let uint8_array = Uint8Array::from(bytes);
    let array = Array::new();
    array.push(&uint8_array);

    let mut props = BlobPropertyBag::new();
    props.type_("application/pdf");

    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &props)
        .map_err(|_| "Failed to create Blob")?;

    Url::create_object_url_with_blob(&blob).map_err(|_| "Failed to create object URL")
}

/// Revoke a blob URL to free memory.
fn revoke_blob_url(url: &str) {
    let _ = Url::revoke_object_url(url);
}

#[component]
fn App() -> Element {
    let mut original_bytes = use_signal(|| None::<Vec<u8>>);
    let mut original_url = use_signal(|| None::<String>);
    let mut centered_url = use_signal(|| None::<String>);
    let mut error_msg = use_signal(|| None::<String>);
    let mut draw_alignment = use_signal(|| true);
    let mut draw_border = use_signal(|| true);
    let mut nudge_x = use_signal(|| 0.0_f64);
    let mut nudge_y = use_signal(|| 0.0_f64);
    let mut nudge_border_x = use_signal(|| 7.0_f64);
    let mut nudge_border_y = use_signal(|| 7.0_f64);
    let mut paper_size = use_signal(|| PaperSize::Letter);

    // File upload handler — uses web-sys to extract the File, gloo-file to read bytes
    let on_file_change = move |evt: Event<FormData>| {
        spawn(async move {
            // Use Dioxus file engine to read the file
            if let Some(file_engine) = evt.files() {
                let files = file_engine.files();
                if let Some(file_name) = files.first() {
                    if let Some(bytes) = file_engine.read_file(file_name).await {
                        // Create blob URL for original preview
                        if let Some(ref old_url) = *original_url.peek() {
                            revoke_blob_url(old_url);
                        }
                        match create_pdf_blob_url(&bytes) {
                            Ok(url) => original_url.set(Some(url)),
                            Err(e) => error_msg.set(Some(e)),
                        }
                        original_bytes.set(Some(bytes));
                    }
                }
            }
        });
    };

    // Reactive processing — reprocess whenever any input changes
    // Read all signals into owned values to avoid holding borrows across set() calls
    use_effect(move || {
        let bytes_opt = original_bytes().clone();
        if let Some(ref pdf_bytes) = bytes_opt {
            let ps = paper_size();
            let options = CenterOptions {
                paper_size: ps.dimensions(),
                draw_alignment: draw_alignment(),
                draw_border: draw_border(),
                nudge_x: nudge_x(),
                nudge_y: nudge_y(),
                nudge_border_x: nudge_border_x(),
                nudge_border_y: nudge_border_y(),
            };

            match center_pdf::center_pdf(pdf_bytes, &options) {
                Ok(new_bytes) => {
                    // Revoke old centered URL (peek to avoid subscribing)
                    if let Some(ref old_url) = *centered_url.peek() {
                        revoke_blob_url(old_url);
                    }
                    match create_pdf_blob_url(&new_bytes) {
                        Ok(url) => {
                            centered_url.set(Some(url));
                            error_msg.set(None);
                        }
                        Err(e) => error_msg.set(Some(e)),
                    }
                }
                Err(e) => {
                    error_msg.set(Some(e));
                    centered_url.set(None);
                }
            }
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/style.css") }

        div { id: "main",
            h1 { "PDF Margins" }
            p { "Upload handler and UI coming in next steps..." }
        }
    }
}
```

Note: The file upload uses Dioxus 0.7's `FormData::files()` API which provides `read_file()`. If this doesn't work on the web target, fall back to using `web-sys` `HtmlInputElement` + `gloo_file::futures::read_as_bytes()` to read the file. The `gloo-file` dependency is included for this fallback.

- [ ] **Step 2: Verify it compiles**

Run: `cargo check --package pdf_margins_dioxus`
Expected: Compiles (file upload may have placeholder logic).

- [ ] **Step 4: Commit**

```bash
git add pdf_margins_dioxus/src/main.rs
git commit -m "feat: add state management, blob URL helpers, and processing effect"
```

---

### Task 5: Full UI Controls and Layout

**Files:**
- Modify: `pdf_margins_dioxus/src/main.rs`
- Modify: `pdf_margins_dioxus/assets/style.css`

- [ ] **Step 1: Build the complete RSX UI**

Replace the RSX in the `App` component with the full controls and preview layout:

```rust
    rsx! {
        document::Stylesheet { href: asset!("/assets/style.css") }

        div { id: "main",
            h1 { "PDF Margins" }

            div { class: "controls",
                // File upload
                label { class: "upload-btn",
                    "Upload PDF"
                    input {
                        r#type: "file",
                        accept: "application/pdf",
                        onchange: on_file_change,
                        style: "display: none;",
                    }
                }

                // Checkboxes
                label {
                    input {
                        r#type: "checkbox",
                        checked: draw_alignment(),
                        onchange: move |evt: Event<FormData>| {
                            draw_alignment.set(evt.checked());
                        },
                    }
                    " Draw Alignment Corner"
                }
                label {
                    input {
                        r#type: "checkbox",
                        checked: draw_border(),
                        onchange: move |evt: Event<FormData>| {
                            draw_border.set(evt.checked());
                        },
                    }
                    " Draw Border"
                }

                // Nudge inputs
                label {
                    "Nudge X: "
                    input {
                        r#type: "number",
                        value: "{nudge_x}",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(v) = evt.value().parse::<f64>() {
                                nudge_x.set(v);
                            }
                        },
                    }
                }
                label {
                    "Nudge Y: "
                    input {
                        r#type: "number",
                        value: "{nudge_y}",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(v) = evt.value().parse::<f64>() {
                                nudge_y.set(v);
                            }
                        },
                    }
                }
                label {
                    "Border X: "
                    input {
                        r#type: "number",
                        value: "{nudge_border_x}",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(v) = evt.value().parse::<f64>() {
                                nudge_border_x.set(v);
                            }
                        },
                    }
                }
                label {
                    "Border Y: "
                    input {
                        r#type: "number",
                        value: "{nudge_border_y}",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(v) = evt.value().parse::<f64>() {
                                nudge_border_y.set(v);
                            }
                        },
                    }
                }

                // Paper size select
                label {
                    "Paper Size: "
                    select {
                        value: "{paper_size}",
                        onchange: move |evt: Event<FormData>| {
                            let val = evt.value();
                            for ps in PaperSize::ALL {
                                if ps.to_string() == val {
                                    paper_size.set(*ps);
                                    break;
                                }
                            }
                        },
                        for ps in PaperSize::ALL {
                            option {
                                value: "{ps}",
                                selected: *ps == paper_size(),
                                "{ps}"
                            }
                        }
                    }
                }
            }

            // Error display
            if let Some(ref err) = *error_msg.read() {
                div { class: "error", "{err}" }
            }

            // Preview area
            div { class: "preview",
                div { class: "preview-pane",
                    h3 { "Original" }
                    if let Some(ref url) = *original_url.read() {
                        iframe { src: "{url}", title: "Original PDF" }
                    }
                }
                div { class: "preview-pane",
                    h3 { "Centered" }
                    if let Some(ref url) = *centered_url.read() {
                        iframe { src: "{url}", title: "Centered PDF" }
                    }
                }
            }
        }
    }
```

- [ ] **Step 2: Complete the CSS**

```css
html, body {
    margin: 0;
    padding: 0;
    height: 100%;
    font-family: system-ui, -apple-system, sans-serif;
}

#main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 16px;
    box-sizing: border-box;
    gap: 12px;
}

h1 {
    margin: 0;
    font-size: 1.5rem;
}

.controls {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 12px;
}

.upload-btn {
    display: inline-block;
    padding: 8px 16px;
    background: #1976d2;
    color: white;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
}

.upload-btn:hover {
    background: #1565c0;
}

.controls label {
    display: flex;
    align-items: center;
    gap: 4px;
}

.controls input[type="number"] {
    width: 70px;
    padding: 4px 8px;
}

.controls select {
    padding: 4px 8px;
}

.error {
    color: #d32f2f;
    padding: 8px;
    background: #fce4ec;
    border-radius: 4px;
}

.preview {
    display: flex;
    flex: 1;
    gap: 16px;
    min-height: 0;
}

.preview-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
}

.preview-pane h3 {
    margin: 0 0 4px 0;
    font-size: 1rem;
}

.preview-pane iframe {
    flex: 1;
    width: 100%;
    border: 1px solid #ccc;
    border-radius: 4px;
}
```

- [ ] **Step 3: Verify it compiles and renders**

Run: `dx serve --package pdf_margins_dioxus`
Expected: App renders with all controls visible (upload, checkboxes, inputs, select, preview area).

- [ ] **Step 4: Commit**

```bash
git add pdf_margins_dioxus/src/main.rs pdf_margins_dioxus/assets/style.css
git commit -m "feat: complete UI controls and layout"
```

---

### Task 6: Integration Testing and Polish

**Files:**
- Modify: `pdf_margins_dioxus/src/main.rs` (fix file upload if needed)
- Modify: `pdf_margins_dioxus/src/center_pdf.rs` (fix lopdf API issues)

- [ ] **Step 1: Manually test file upload**

Run: `dx serve --package pdf_margins_dioxus`
Upload a PDF file and verify:
- Original preview iframe shows the uploaded PDF
- Centered preview iframe shows the centered version
- If file upload doesn't work, debug and fix the event handling

- [ ] **Step 2: Test all controls**

Verify each control works:
- Toggle "Draw Alignment Corner" — centered PDF updates
- Toggle "Draw Border" — centered PDF updates
- Change Nudge X/Y values — centered PDF content shifts
- Change Border X/Y values — border adjusts
- Change Paper Size — centered PDF re-renders with new page size

- [ ] **Step 3: Test orientation matching**

Upload a landscape PDF and verify:
- New page is also landscape
- Content is centered correctly
- This validates the orientation bugfix

- [ ] **Step 4: Test edge cases**

- Upload a multi-page PDF — all pages should be centered
- Upload a PDF with embedded fonts/images — should render correctly
- Try uploading a non-PDF file — should show error message

- [ ] **Step 5: Fix any issues found**

Address any bugs discovered during testing. Common issues:
- lopdf API differences from what's in the plan (method names, trait implementations)
- `web-sys` feature flags missing for certain APIs
- Signal borrow issues in `use_effect` (read into locals before awaiting)

- [ ] **Step 6: Final compile check**

Run: `cargo clippy --package pdf_margins_dioxus`
Expected: No errors, minimal warnings.

- [ ] **Step 7: Commit**

```bash
git add pdf_margins_dioxus/
git commit -m "feat: pdf_margins_dioxus integration testing and polish"
```
