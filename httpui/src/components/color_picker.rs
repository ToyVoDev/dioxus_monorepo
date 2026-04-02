use dioxus::prelude::*;

const COLORS: &[&str] = &["#FFB3AD", "#44E2CD", "#F9BD22", "#FF5451"];

#[component]
pub fn ColorPicker(selected: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "color-picker",
            for color in COLORS {
                button {
                    class: "color-picker__swatch",
                    style:format!("background-color: {}", color),
                    onclick: move |_| {
                        on_change.call(color.to_string());
                    },
                }
            }
        }
    }
}
