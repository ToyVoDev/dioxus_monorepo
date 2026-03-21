use dioxus::prelude::*;

#[component]
pub fn StatusBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "statusbar",
            span { "Ready" }
        }
    }
}
