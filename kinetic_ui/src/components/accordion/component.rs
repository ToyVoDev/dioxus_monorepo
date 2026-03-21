use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};

#[component]
pub fn KAccordion(props: AccordionProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        accordion::Accordion {
            class: "k-accordion",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KAccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            class: "k-accordion__item",
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KAccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            class: "k-accordion__trigger",
            id: props.id,
            attributes: props.attributes,
            {props.children}
            svg {
                class: "k-accordion__expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn KAccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        accordion::AccordionContent {
            class: "k-accordion__content",
            style: "--collapsible-content-width: 140px",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
