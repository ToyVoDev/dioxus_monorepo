use dioxus::prelude::*;

#[component]
pub fn Artists() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 40vh; color: var(--secondary-color-4);",
            h2 { style: "font-family: var(--k-font-display); color: var(--secondary-color-3);", "Artists" }
            p { "Coming soon" }
        }
    }
}
