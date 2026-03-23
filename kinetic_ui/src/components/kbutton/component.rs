use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Default, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum KButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Destructive,
}

#[component]
pub fn KButton(
    #[props(default)] variant: KButtonVariant,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let class = format!("k-button k-button--{variant}");

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        button {
            class: "{class}",
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            {children}
        }
    }
}
