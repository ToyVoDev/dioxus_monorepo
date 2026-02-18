use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Desktop Home" }
        p { "Desktop platform - coming soon." }
    }
}
