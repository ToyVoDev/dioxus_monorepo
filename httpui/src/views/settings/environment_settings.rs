use dioxus::prelude::*;

#[component]
pub fn EnvironmentSettings(id: i32) -> Element {
    rsx! {
        div { class: "settings-page",
            h1 { "Environment Settings" }
            p { "Environment ID: {id}" }
            p { "Configure variables for URL and header substitution." }
        }
    }
}
