use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    pub children: Element,
    pub onclick: EventHandler<MouseEvent>,
    pub active: Option<bool>,
    pub disabled: Option<bool>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class:"border border-solid border-white hover:bg-neutral-800 hover:cursor-pointer p-3 rounded-md text-white",
            class: if props.active == Some(true) {
                "bg-neutral-700"
            },
            disabled: props.disabled == Some(true),
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
        }
    }
}
