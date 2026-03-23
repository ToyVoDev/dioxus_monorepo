mod center_pdf;

use center_pdf::{CenterOptions, PaperSize};
use dioxus::prelude::*;
use js_sys::{Array, Uint8Array};
use web_sys::{Blob, BlobPropertyBag, Url};

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

fn create_pdf_blob_url(bytes: &[u8]) -> Result<String, String> {
    let uint8_array = Uint8Array::new_with_length(bytes.len() as u32);
    uint8_array.copy_from(bytes);

    let parts = Array::new();
    parts.push(&uint8_array.buffer());

    let opts = BlobPropertyBag::new();
    opts.set_type("application/pdf");

    let blob = Blob::new_with_buffer_source_sequence_and_options(&parts, &opts)
        .map_err(|e| format!("Failed to create Blob: {e:?}"))?;

    Url::create_object_url_with_blob(&blob).map_err(|e| format!("Failed to create URL: {e:?}"))
}

fn revoke_blob_url(url: &str) {
    let _ = Url::revoke_object_url(url);
}

#[component]
fn App() -> Element {
    let mut original_bytes: Signal<Option<Vec<u8>>> = use_signal(|| None);
    let mut original_url: Signal<Option<String>> = use_signal(|| None);
    let mut centered_url: Signal<Option<String>> = use_signal(|| None);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut draw_alignment: Signal<bool> = use_signal(|| true);
    let mut draw_border: Signal<bool> = use_signal(|| true);
    let mut nudge_x: Signal<f64> = use_signal(|| 0.0);
    let mut nudge_y: Signal<f64> = use_signal(|| 0.0);
    let mut nudge_border_x: Signal<f64> = use_signal(|| 7.0);
    let mut nudge_border_y: Signal<f64> = use_signal(|| 7.0);
    let mut paper_size: Signal<PaperSize> = use_signal(|| PaperSize::Letter);

    let on_file_change = move |evt: Event<FormData>| {
        spawn(async move {
            let files = evt.files();
            if let Some(file_data) = files.first() {
                match file_data.read_bytes().await {
                    Ok(bytes) => {
                        // Revoke old original URL
                        if let Some(old_url) = original_url.peek().as_ref() {
                            revoke_blob_url(old_url);
                        }

                        // Create new blob URL for original
                        match create_pdf_blob_url(&bytes) {
                            Ok(url) => {
                                original_url.set(Some(url));
                                error_msg.set(None);
                            }
                            Err(e) => {
                                original_url.set(None);
                                error_msg.set(Some(e));
                            }
                        }

                        original_bytes.set(Some(bytes.to_vec()));
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Failed to read file: {e}")));
                    }
                }
            }
        });
    };

    // Reactive processing effect: re-center whenever inputs change
    use_effect(move || {
        let bytes_opt = original_bytes().clone();
        let alignment = draw_alignment();
        let border = draw_border();
        let nx = nudge_x();
        let ny = nudge_y();
        let nbx = nudge_border_x();
        let nby = nudge_border_y();
        let size = paper_size();

        if let Some(ref pdf_bytes) = bytes_opt {
            let options = CenterOptions {
                paper_size: size.dimensions(),
                draw_alignment: alignment,
                draw_border: border,
                nudge_x: nx,
                nudge_y: ny,
                nudge_border_x: nbx,
                nudge_border_y: nby,
            };

            match center_pdf::center_pdf(pdf_bytes, &options) {
                Ok(centered_bytes) => {
                    // Revoke old centered URL (use peek to avoid re-subscribing)
                    if let Some(old_url) = centered_url.peek().as_ref() {
                        revoke_blob_url(old_url);
                    }
                    match create_pdf_blob_url(&centered_bytes) {
                        Ok(url) => {
                            centered_url.set(Some(url));
                            error_msg.set(None);
                        }
                        Err(e) => {
                            centered_url.set(None);
                            error_msg.set(Some(e));
                        }
                    }
                }
                Err(e) => {
                    // Revoke old centered URL on error too
                    if let Some(old_url) = centered_url.peek().as_ref() {
                        revoke_blob_url(old_url);
                    }
                    centered_url.set(None);
                    error_msg.set(Some(e));
                }
            }
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/style.css") }
        div { id: "main",
            h1 { "PDF Margins" }

            div { class: "controls",
                // File upload
                label { class: "upload-btn",
                    "Choose PDF"
                    input {
                        r#type: "file",
                        accept: ".pdf",
                        style: "display: none;",
                        onchange: on_file_change,
                    }
                }

                // Draw Alignment Corner checkbox
                label {
                    input {
                        r#type: "checkbox",
                        checked: draw_alignment(),
                        onchange: move |evt: Event<FormData>| {
                            draw_alignment.set(evt.checked());
                        },
                    }
                    "Draw Alignment Corner"
                }

                // Draw Border checkbox
                label {
                    input {
                        r#type: "checkbox",
                        checked: draw_border(),
                        onchange: move |evt: Event<FormData>| {
                            draw_border.set(evt.checked());
                        },
                    }
                    "Draw Border"
                }

                // Nudge X
                label {
                    "Nudge X"
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

                // Nudge Y
                label {
                    "Nudge Y"
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

                // Border X
                label {
                    "Border X"
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

                // Border Y
                label {
                    "Border Y"
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
                    "Paper Size"
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
}
