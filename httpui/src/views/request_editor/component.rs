use crate::components::input::Input;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn RequestEditor() -> Element {
    let app_state = use_context::<AppState>();
    let mut requests = app_state.requests;
    let selected_request = app_state.selected_request;

    let current_request = use_memo(move || {
        let sel = selected_request();
        sel.and_then(|id| requests.read().iter().find(|r| r.id == id).cloned())
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            id: "request_editor",
            if let Some(request) = current_request() {
                div { class: "editor-section",
                    label { class: "editor-label", "Name" }
                    Input {
                        value: "{request.name}",
                        placeholder: "Request name",
                        oninput: move |e: FormEvent| {
                            let new_name = e.value();
                            if let Some(id) = selected_request() {
                                requests.with_mut(|reqs| {
                                    if let Some(req) = reqs.iter_mut().find(|r| r.id == id) {
                                        req.name = new_name;
                                    }
                                });
                            }
                        },
                    }
                }
            } else {
                div { class: "editor-empty",
                    "Select or create a request to begin editing."
                }
            }
        }
    }
}
