use dioxus::prelude::*;

#[component]
pub fn IconButton(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] active: Option<bool>,
    children: Element,
) -> Element {
    let active_str = active.unwrap_or(false).to_string();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        button {
            class: "k-icon-button",
            "data-active": "{active_str}",
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            {children}
        }
    }
}
