use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Mobile Home" }
        p { "Mobile platform - coming soon." }
    }
}
