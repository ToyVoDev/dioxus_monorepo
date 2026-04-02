mod center_pdf;

use center_pdf::{CenterOptions, PaperSize};
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use js_sys::{Array, Uint8Array};
use kinetic_ui::{
    Checkbox, KBadge, KBadgeVariant, KButton, KButtonVariant, KInput, KSelect, KSelectList,
    KSelectOption, KSelectTrigger, KSelectValue, KineticTheme, ThemeToggle,
};
use web_sys::{Blob, BlobPropertyBag, Url};

fn main() {
    dioxus_logger::initialize_default();
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
    let paper_size_value: Memo<Option<Option<PaperSize>>> =
        use_memo(move || Some(Some(paper_size())));

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
        let bytes_opt = original_bytes();
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
        KineticTheme {
            div { class: "app-shell",
                div { class: "title-row",
                    h1 { "PDF Margins" }
                    ThemeToggle {}
                }

                div { class: "controls",
                    // File upload
                    KButton {
                        variant: KButtonVariant::Primary,
                        onclick: move |_| {
                            document::eval(r#"document.getElementById('pdf-file-input').click()"#);
                        },
                        "Choose PDF"
                    }
                    input {
                        id: "pdf-file-input",
                        r#type: "file",
                        accept: ".pdf",
                        style: "display: none;",
                        onchange: on_file_change,
                    }

                    // Draw Alignment Corner checkbox
                    label { class: "checkbox-label",
                        Checkbox {
                            checked: if draw_alignment() { CheckboxState::Checked } else { CheckboxState::Unchecked },
                            on_checked_change: move |val: CheckboxState| {
                                draw_alignment.set(bool::from(val));
                            },
                        }
                        "Draw Alignment Corner"
                    }

                    // Draw Border checkbox
                    label { class: "checkbox-label",
                        Checkbox {
                            checked: if draw_border() { CheckboxState::Checked } else { CheckboxState::Unchecked },
                            on_checked_change: move |val: CheckboxState| {
                                draw_border.set(bool::from(val));
                            },
                        }
                        "Draw Border"
                    }

                    // Nudge X
                    label {
                        "Nudge X"
                        KInput {
                            r#type: "number".to_string(),
                            value: format!("{}", nudge_x()),
                            oninput: move |evt: FormEvent| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    nudge_x.set(v);
                                }
                            },
                        }
                    }

                    // Nudge Y
                    label {
                        "Nudge Y"
                        KInput {
                            r#type: "number".to_string(),
                            value: format!("{}", nudge_y()),
                            oninput: move |evt: FormEvent| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    nudge_y.set(v);
                                }
                            },
                        }
                    }

                    // Border X
                    label {
                        "Border X"
                        KInput {
                            r#type: "number".to_string(),
                            value: format!("{}", nudge_border_x()),
                            oninput: move |evt: FormEvent| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    nudge_border_x.set(v);
                                }
                            },
                        }
                    }

                    // Border Y
                    label {
                        "Border Y"
                        KInput {
                            r#type: "number".to_string(),
                            value: format!("{}", nudge_border_y()),
                            oninput: move |evt: FormEvent| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    nudge_border_y.set(v);
                                }
                            },
                        }
                    }

                    // Paper size select
                    label {
                        "Paper Size"
                        KSelect::<PaperSize> {
                            value: paper_size_value,
                            on_value_change: move |val: Option<PaperSize>| {
                                if let Some(ps) = val {
                                    paper_size.set(ps);
                                }
                            },
                            KSelectTrigger {
                                KSelectValue {}
                            }
                            KSelectList {
                                for (i, ps) in PaperSize::ALL.iter().enumerate() {
                                    KSelectOption::<PaperSize> {
                                        value: *ps,
                                        index: i,
                                        text_value: ps.to_string(),
                                        "{ps}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Error display
                if let Some(ref err) = *error_msg.read() {
                    KBadge { variant: KBadgeVariant::Error, "{err}" }
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
}
