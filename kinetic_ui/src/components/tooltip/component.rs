use dioxus::prelude::*;
use dioxus_primitives::tooltip::{self, TooltipContentProps, TooltipProps, TooltipTriggerProps};

#[component]
pub fn KTooltip(props: TooltipProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        tooltip::Tooltip {
            class: "k-tooltip",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KTooltipTrigger(props: TooltipTriggerProps) -> Element {
    rsx! {
        tooltip::TooltipTrigger {
            id: props.id,
            r#as: props.r#as,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KTooltipContent(props: TooltipContentProps) -> Element {
    rsx! {
        tooltip::TooltipContent {
            class: "k-tooltip__content",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
