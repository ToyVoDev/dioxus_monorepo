use crate::state::{AppState, CreateType};
use dioxus::prelude::*;
use kinetic_ui::{KButton, KButtonVariant, KInput};

#[component]
pub fn CreateModal() -> Element {
    let mut state = use_context::<AppState>();
    let mut modal_type = state.create_modal_type;

    let mut space_name = use_signal(String::new);
    let space_icon = use_signal(|| "folder".to_string());
    let space_color = use_signal(|| "#FFB3AD".to_string());

    let mut collection_name = use_signal(String::new);
    let collection_icon = use_signal(|| "folder".to_string());
    let collection_color = use_signal(|| "#FFB3AD".to_string());
    let mut collection_space_id =
        use_signal(|| state.spaces.read().first().map(|s| s.id).unwrap_or(-1));

    let mut request_name = use_signal(String::new);
    let mut request_method = use_signal(|| "GET".to_string());
    let mut request_url = use_signal(String::new);
    let mut request_collection_id =
        use_signal(|| state.collections.read().first().map(|c| c.id).unwrap_or(-1));

    let mut environment_name = use_signal(String::new);

    match modal_type() {
        None => rsx! {},
        Some(CreateType::Space) => rsx! {
            document::Link { rel: "stylesheet", href: asset!("./style.css") }
            div { class: "modal-backdrop",
                onclick: move |_| modal_type.set(None),
                div { class: "modal",
                    onclick: |e| e.stop_propagation(),
                    h2 { "Create Space" }
                    div { class: "modal__content",
                        KInput {
                            placeholder: "Space name (required)".to_string(),
                            value: space_name(),
                            oninput: move |e: FormEvent| space_name.set(e.value()),
                        }
                        p { "Icon: {space_icon}" }
                        p { "Color: {space_color}" }
                    }
                    div { class: "modal__actions",
                        KButton {
                            variant: KButtonVariant::Secondary,
                            onclick: move |_| modal_type.set(None),
                            "Cancel"
                        }
                        KButton {
                            variant: KButtonVariant::Primary,
                            onclick: move |_| {
                                let name = space_name().clone();
                                if name.trim().is_empty() {
                                    return;
                                }
                                let icon = space_icon().clone();
                                let color = space_color().clone();
                                let id = state.create_space(name, icon, color);
                                state.selected_space.set(Some(id));
                                modal_type.set(None);
                            },
                            "Create"
                        }
                    }
                }
            }
        },
        Some(CreateType::Collection) => rsx! {
            document::Link { rel: "stylesheet", href: asset!("./style.css") }
            div { class: "modal-backdrop",
                onclick: move |_| modal_type.set(None),
                div { class: "modal",
                    onclick: |e| e.stop_propagation(),
                    h2 { "Create Collection" }
                    div { class: "modal__content",
                        KInput {
                            placeholder: "Collection name (required)".to_string(),
                            value: collection_name(),
                            oninput: move |e: FormEvent| collection_name.set(e.value()),
                        }
                        div { class: "modal__field",
                            label { "Space: " }
                            select {
                                value: collection_space_id(),
                                onchange: move |e| {
                                    if let Ok(id) = e.value().parse::<i32>() {
                                        collection_space_id.set(id);
                                    }
                                },
                                for space in state.spaces.read().iter() {
                                    option {
                                        value: "{space.id}",
                                        "{space.name} (#{space.id})"
                                    }
                                }
                            }
                        }
                        p { "Icon: {collection_icon}" }
                        p { "Color: {collection_color}" }
                    }
                    div { class: "modal__actions",
                        KButton {
                            variant: KButtonVariant::Secondary,
                            onclick: move |_| modal_type.set(None),
                            "Cancel"
                        }
                        KButton {
                            variant: KButtonVariant::Primary,
                            onclick: move |_| {
                                let name = collection_name().clone();
                                if name.trim().is_empty() {
                                    return;
                                }
                                let icon = collection_icon().clone();
                                let color = collection_color().clone();
                                let space_id = collection_space_id();
                                let _id = state.create_collection(name, icon, color, space_id);
                                modal_type.set(None);
                            },
                            "Create"
                        }
                    }
                }
            }
        },
        Some(CreateType::Request) => rsx! {
            document::Link { rel: "stylesheet", href: asset!("./style.css") }
            div { class: "modal-backdrop",
                onclick: move |_| modal_type.set(None),
                div { class: "modal",
                    onclick: |e| e.stop_propagation(),
                    h2 { "Create Request" }
                    div { class: "modal__content",
                        KInput {
                            placeholder: "Request name (optional)".to_string(),
                            value: request_name(),
                            oninput: move |e: FormEvent| request_name.set(e.value()),
                        }
                        div { class: "modal__field",
                            label { "Method: " }
                            select {
                                value: request_method(),
                                onchange: move |e| request_method.set(e.value()),
                                option { value: "GET", "GET" }
                                option { value: "POST", "POST" }
                                option { value: "PUT", "PUT" }
                                option { value: "PATCH", "PATCH" }
                                option { value: "DELETE", "DELETE" }
                            }
                        }
                        KInput {
                            placeholder: "URL (required)".to_string(),
                            value: request_url(),
                            oninput: move |e: FormEvent| request_url.set(e.value()),
                        }
                        div { class: "modal__field",
                            label { "Collection: " }
                            select {
                                value: request_collection_id(),
                                onchange: move |e| {
                                    if let Ok(id) = e.value().parse::<i32>() {
                                        request_collection_id.set(id);
                                    }
                                },
                                for collection in state.collections.read().iter() {
                                    option {
                                        value: "{collection.id}",
                                        "{collection.name} (#{collection.id})"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "modal__actions",
                        KButton {
                            variant: KButtonVariant::Secondary,
                            onclick: move |_| modal_type.set(None),
                            "Cancel"
                        }
                        KButton {
                            variant: KButtonVariant::Primary,
                            onclick: move |_| {
                                let url = request_url().clone();
                                if url.trim().is_empty() {
                                    return;
                                }
                                let name = if request_name().trim().is_empty() {
                                    None
                                } else {
                                    Some(request_name().clone())
                                };
                                let method = request_method().clone();
                                let collection_id = request_collection_id();
                                let id = state.create_request(name, method, url, collection_id);
                                state.selected_request.set(Some(id));
                                state.open_requests.write().push(id);
                                modal_type.set(None);
                            },
                            "Create"
                        }
                    }
                }
            }
        },
        Some(CreateType::Environment) => {
            let space_id = state.spaces.read().first().map(|s| s.id).unwrap_or(-1);
            rsx! {
                document::Link { rel: "stylesheet", href: asset!("./style.css") }
                div { class: "modal-backdrop",
                    onclick: move |_| modal_type.set(None),
                    div { class: "modal",
                        onclick: |e| e.stop_propagation(),
                        h2 { "Create Environment" }
                        div { class: "modal__content",
                            KInput {
                                placeholder: "Environment name (required)".to_string(),
                                value: environment_name(),
                                oninput: move |e: FormEvent| environment_name.set(e.value()),
                            }
                        }
                        div { class: "modal__actions",
                            KButton {
                                variant: KButtonVariant::Secondary,
                                onclick: move |_| modal_type.set(None),
                                "Cancel"
                            }
                            KButton {
                                variant: KButtonVariant::Primary,
                                onclick: move |_| {
                                    let name = environment_name().clone();
                                    if name.trim().is_empty() {
                                        return;
                                    }
                                    let _id = state.create_environment(name, space_id);
                                    modal_type.set(None);
                                },
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}
