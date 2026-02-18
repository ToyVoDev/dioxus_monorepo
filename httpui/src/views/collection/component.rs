use dioxus::prelude::*;

#[component]
pub fn CollectionSection(id: i32) -> Element {
    rsx! {
        div {
            id: "collection",
            "Collection #{id}"
        }
    }
}
