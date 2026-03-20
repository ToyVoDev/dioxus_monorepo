use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Default, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Error,
    Muted,
}

#[component]
pub fn Badge(#[props(default)] variant: BadgeVariant, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        span {
            class: "k-badge",
            "data-variant": "{variant}",
            {children}
        }
    }
}
