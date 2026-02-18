use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn ResponseViewer() -> Element {
    let app_state = use_context::<AppState>();
    let response = app_state.response;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            id: "response_viewer",
            if response().is_empty() {
                div { class: "response-empty",
                    "Send a request to see the response."
                }
            } else {
                pre { class: "response-body",
                    code { "{response}" }
                }
            }
        }
    }
}
