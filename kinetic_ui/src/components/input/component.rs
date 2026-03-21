use dioxus::prelude::*;

#[component]
pub fn Input(
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] r#type: String,
    #[props(default)] monospace: bool,
) -> Element {
    let class = if monospace {
        "k-input k-input--mono"
    } else {
        "k-input"
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        input {
            class: "{class}",
            r#type: r#type,
            value: "{value}",
            placeholder: "{placeholder}",
            oninput: move |e| {
                if let Some(f) = &oninput {
                    f.call(e);
                }
            },
            onchange: move |e| {
                if let Some(f) = &onchange {
                    f.call(e);
                }
            },
            onfocus: move |e| {
                if let Some(f) = &onfocus {
                    f.call(e);
                }
            },
            onblur: move |e| {
                if let Some(f) = &onblur {
                    f.call(e);
                }
            },
        }
    }
}
