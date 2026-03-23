use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Default, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum KBadgeVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Error,
    Muted,
}

#[component]
pub fn KBadge(#[props(default)] variant: KBadgeVariant, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        span {
            class: "k-badge",
            "data-variant": "{variant}",
            {children}
        }
    }
}
