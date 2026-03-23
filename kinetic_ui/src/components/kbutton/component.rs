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
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let class = format!(
        "k-button k-button--{variant}{}",
        if disabled { " k-button--disabled" } else { "" }
    );

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        button {
            class: "{class}",
            disabled: disabled,
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
