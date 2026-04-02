use dioxus::prelude::*;

#[component]
pub fn CollectionSettings(id: i32) -> Element {
    rsx! {
        div { class: "settings-page",
            h1 { "Collection Settings" }
            p { "Collection ID: {id}" }
            p { "Configure headers, authentication, and other settings for this collection." }
        }
    }
}
