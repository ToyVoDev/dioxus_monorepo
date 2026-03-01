use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps<T: IconShape + Clone + PartialEq + 'static> {
    pub icon: T,
    pub children: Element,
    pub onclick: EventHandler<MouseEvent>,
    pub active: Option<bool>,
    pub disabled: Option<bool>,
}

#[component]
pub fn IconButton<T: IconShape + Clone + PartialEq + 'static>(
    props: IconButtonProps<T>,
) -> Element {
    rsx! {
        button {
            class:"hover:bg-neutral-800 p-2 rounded-full",
            class: if props.active == Some(true) {
                "bg-neutral-700"
            },
            class: if props.disabled == Some(true) {
                "hover:cursor-not-allowed"
            } else {
                "hover:cursor-pointer"
            },
            disabled: props.disabled == Some(true),
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
            Icon {
                icon: props.icon,
                fill: if props.disabled == Some(true) {
                    "gray"
                } else {
                    "white"
                }
            }
        }
    }
}
