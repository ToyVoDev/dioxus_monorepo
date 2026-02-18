use dioxus::prelude::*;

#[component]
pub fn SpaceSection(id: i32) -> Element {
    rsx! {
        div {
            id: "space",
            "Space #{id}"
        }
    }
}
